//! Port of `Dh_NetClient/Server.cs` — the connection + authentication handshake.
//!
//! The handshake is a single unary call, not a streaming login: open the gRPC
//! channel, call `ConfigService.GetConfigurationConstants` with an
//! `authorization` *request* header, and read the session token back from the
//! *response* `authorization` header. That token authorizes every later call
//! and is refreshed on a keepalive timer before it expires.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tonic::metadata::{MetadataKey, MetadataMap, MetadataValue};
use tonic::transport::Channel;
use tonic::Request;

use crate::error::{Error, Result};
use crate::proto::grpc::config_service_client::ConfigServiceClient;
use crate::proto::grpc::{config_value, ConfigValue, ConfigurationConstantsRequest};

const AUTHORIZATION_KEY: &str = "authorization";
const TIMEOUT_KEY: &str = "http.session.durationMs";

/// How the client identifies itself in the initial `authorization` header.
///
/// Mirrors the documented Deephaven auth handlers (see CLAUDE.md).
#[derive(Debug, Clone)]
pub struct ClientOptions {
    authorization_value: String,
    extra_headers: Vec<(String, String)>,
}

impl ClientOptions {
    /// Anonymous access (`Anonymous`).
    pub fn anonymous() -> ClientOptions {
        ClientOptions {
            authorization_value: "Anonymous".to_string(),
            extra_headers: Vec::new(),
        }
    }

    /// Pre-shared-key auth (`io.deephaven.authentication.psk.PskAuthenticationHandler <psk>`).
    pub fn psk(psk: impl AsRef<str>) -> ClientOptions {
        ClientOptions {
            authorization_value: format!(
                "io.deephaven.authentication.psk.PskAuthenticationHandler {}",
                psk.as_ref()
            ),
            extra_headers: Vec::new(),
        }
    }

    /// Add an extra header sent on every request (e.g. an envoy route prefix).
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> ClientOptions {
        self.extra_headers.push((key.into(), value.into()));
        self
    }
}

/// Fields shared between the [`Server`] handle and its keepalive task.
struct Inner {
    /// Current session token (refreshed on each authorized RPC / keepalive).
    token: Mutex<String>,
    extra_headers: Vec<(String, String)>,
    /// How long to wait between keepalive pings (already halved per server policy).
    expiration: Duration,
}

impl Inner {
    fn token(&self) -> String {
        self.token.lock().unwrap_or_else(|p| p.into_inner()).clone()
    }

    fn set_token(&self, token: String) {
        *self.token.lock().unwrap_or_else(|p| p.into_inner()) = token;
    }
}

/// An authenticated connection to a Deephaven server. Cloneable handle to the
/// gRPC channel plus the live session token. Dropping it stops the keepalive.
pub struct Server {
    channel: Channel,
    inner: Arc<Inner>,
    keepalive: Option<tokio::task::JoinHandle<()>>,
}

impl Server {
    /// Connect to `target` (e.g. `localhost:10000` or `http://host:port`) and
    /// perform the auth handshake. Port of `Server.CreateFromTarget`.
    pub async fn connect(target: &str, options: ClientOptions) -> Result<Server> {
        let endpoint = if target.contains("://") {
            target.to_string()
        } else {
            format!("http://{target}")
        };

        let channel = Channel::from_shared(endpoint)
            .map_err(|e| Error::InvalidUri(e.to_string()))?
            .connect()
            .await?;

        let mut config = ConfigServiceClient::new(channel.clone());

        // Initial request carries the auth value; the token comes back in the
        // response headers.
        let mut request = Request::new(ConfigurationConstantsRequest {});
        let auth_val = MetadataValue::try_from(options.authorization_value.as_str())
            .map_err(|e| Error::Metadata(e.to_string()))?;
        request.metadata_mut().insert(AUTHORIZATION_KEY, auth_val);
        insert_extra_headers(request.metadata_mut(), &options.extra_headers)?;

        let response = config.get_configuration_constants(request).await?;
        let token = response
            .metadata()
            .get(AUTHORIZATION_KEY)
            .ok_or(Error::MissingAuthToken)?
            .to_str()
            .map_err(|e| Error::Metadata(e.to_string()))?
            .to_string();

        let config_values = response.into_inner().config_values;
        let expiration =
            extract_expiration(&config_values).unwrap_or_else(|| Duration::from_secs(10));

        let inner = Arc::new(Inner {
            token: Mutex::new(token),
            extra_headers: options.extra_headers,
            expiration,
        });

        let mut server = Server { channel, inner: inner.clone(), keepalive: None };
        server.keepalive = Some(server.spawn_keepalive());
        Ok(server)
    }

    /// A clone of the underlying channel for building other service clients.
    pub fn channel(&self) -> Channel {
        self.channel.clone()
    }

    /// Wrap `message` in a request carrying the current session token plus any
    /// configured extra headers. Used by every authorized RPC in later phases.
    pub fn authorize<T>(&self, message: T) -> Result<Request<T>> {
        let mut request = Request::new(message);
        let token = self.inner.token();
        let val = MetadataValue::try_from(token.as_str())
            .map_err(|e| Error::Metadata(e.to_string()))?;
        request.metadata_mut().insert(AUTHORIZATION_KEY, val);
        insert_extra_headers(request.metadata_mut(), &self.inner.extra_headers)?;
        Ok(request)
    }

    /// Fetch the server configuration constants (also refreshing the token if
    /// the server rotated it). Returns the string-valued entries.
    pub async fn configuration_constants(&self) -> Result<HashMap<String, String>> {
        let request = self.authorize(ConfigurationConstantsRequest {})?;
        let mut config = ConfigServiceClient::new(self.channel.clone());
        let response = config.get_configuration_constants(request).await?;
        maybe_refresh_token(&self.inner, response.metadata());

        let config_values = response.into_inner().config_values;
        let out = config_values
            .iter()
            .filter_map(|(k, v)| config_string(v).map(|s| (k.clone(), s.to_string())))
            .collect();
        Ok(out)
    }

    /// Keepalive loop: ping `GetConfigurationConstants` on the expiration timer
    /// and roll the token forward. Port of `Server.SendKeepaliveMessage`/`Ping`.
    fn spawn_keepalive(&self) -> tokio::task::JoinHandle<()> {
        let channel = self.channel.clone();
        let inner = self.inner.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(inner.expiration).await;

                let mut request = Request::new(ConfigurationConstantsRequest {});
                if let Ok(val) = MetadataValue::try_from(inner.token().as_str()) {
                    request.metadata_mut().insert(AUTHORIZATION_KEY, val);
                }
                let _ = insert_extra_headers(request.metadata_mut(), &inner.extra_headers);

                let mut config = ConfigServiceClient::new(channel.clone());
                match config.get_configuration_constants(request).await {
                    Ok(response) => maybe_refresh_token(&inner, response.metadata()),
                    Err(_) => {
                        // On failure, retry relatively frequently (as in C#).
                        tokio::time::sleep(Duration::from_secs(10)).await;
                    }
                }
            }
        })
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        if let Some(handle) = self.keepalive.take() {
            handle.abort();
        }
    }
}

/// Roll the session token forward if the response carried a fresh one.
fn maybe_refresh_token(inner: &Inner, metadata: &MetadataMap) {
    if let Some(tok) = metadata.get(AUTHORIZATION_KEY) {
        if let Ok(s) = tok.to_str() {
            inner.set_token(s.to_string());
        }
    }
}

fn insert_extra_headers(meta: &mut MetadataMap, headers: &[(String, String)]) -> Result<()> {
    for (k, v) in headers {
        let key = MetadataKey::from_bytes(k.as_bytes())
            .map_err(|e| Error::Metadata(e.to_string()))?;
        let val =
            MetadataValue::try_from(v.as_str()).map_err(|e| Error::Metadata(e.to_string()))?;
        meta.insert(key, val);
    }
    Ok(())
}

fn config_string(value: &ConfigValue) -> Option<&str> {
    match &value.kind {
        Some(config_value::Kind::StringValue(s)) => Some(s.as_str()),
        None => None,
    }
}

/// Extract the session duration and halve it (server policy), as in
/// `Server.TryExtractExpirationInterval`.
fn extract_expiration(config_values: &HashMap<String, ConfigValue>) -> Option<Duration> {
    let value = config_values.get(TIMEOUT_KEY)?;
    let s = config_string(value)?;
    let millis: i64 = s.parse().ok()?;
    if millis <= 0 {
        return None;
    }
    Some(Duration::from_millis((millis / 2) as u64))
}
