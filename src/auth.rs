//! Port of `Dh_NetClient/Server.cs` — the connection + authentication handshake.
//!
//! The handshake is a single unary call, not a streaming login: open the gRPC
//! channel, call `ConfigService.GetConfigurationConstants` with an
//! `authorization` *request* header, and read the session token back from the
//! *response* `authorization` header. That token authorizes every later call
//! and is refreshed on a keepalive timer before it expires.

use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tonic::metadata::{MetadataKey, MetadataMap, MetadataValue};
use tonic::transport::{Certificate, Channel, ClientTlsConfig};
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
    /// Extra CA certificate (PEM) to trust, in addition to the OS roots. For
    /// servers (e.g. Enterprise) whose internal CA isn't in the OS trust store.
    ca_cert_pem: Option<Vec<u8>>,
    /// Override the TLS server name (SNI / cert hostname) if it differs from the
    /// host in the URL.
    tls_domain: Option<String>,
    /// DANGEROUS: skip TLS certificate verification entirely.
    tls_insecure: bool,
}

impl ClientOptions {
    fn with_value(authorization_value: String) -> ClientOptions {
        ClientOptions {
            authorization_value,
            extra_headers: Vec::new(),
            ca_cert_pem: None,
            tls_domain: None,
            tls_insecure: false,
        }
    }

    /// Anonymous access (`Anonymous`).
    pub fn anonymous() -> ClientOptions {
        ClientOptions::with_value("Anonymous".to_string())
    }

    /// Pre-shared-key auth (`io.deephaven.authentication.psk.PskAuthenticationHandler <psk>`).
    pub fn psk(psk: impl AsRef<str>) -> ClientOptions {
        ClientOptions::with_value(format!(
            "io.deephaven.authentication.psk.PskAuthenticationHandler {}",
            psk.as_ref()
        ))
    }

    /// HTTP Basic auth (`Basic <base64(user:password)>`). Used by Deephaven's
    /// basic username/password authentication handler.
    pub fn basic(user: impl AsRef<str>, password: impl AsRef<str>) -> ClientOptions {
        let creds = format!("{}:{}", user.as_ref(), password.as_ref());
        ClientOptions::with_value(format!("Basic {}", base64_encode(creds.as_bytes())))
    }

    /// Use a fully-formed `authorization` value verbatim, for auth handlers not
    /// covered by the constructors above (e.g. a custom Enterprise handler:
    /// `"io.deephaven.<...>Handler <payload>"`).
    pub fn with_authorization(value: impl Into<String>) -> ClientOptions {
        ClientOptions::with_value(value.into())
    }

    /// Add an extra header sent on every request (e.g. an envoy route prefix).
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> ClientOptions {
        self.extra_headers.push((key.into(), value.into()));
        self
    }

    /// Trust an additional CA certificate (PEM bytes) for TLS, on top of the OS
    /// roots. Use when the server presents a cert signed by an internal CA.
    pub fn with_ca_certificate(mut self, pem: impl Into<Vec<u8>>) -> ClientOptions {
        self.ca_cert_pem = Some(pem.into());
        self
    }

    /// Override the expected TLS server name (cert hostname / SNI).
    pub fn with_tls_domain(mut self, domain: impl Into<String>) -> ClientOptions {
        self.tls_domain = Some(domain.into());
        self
    }

    /// DANGEROUS: disable TLS certificate verification. Only for internal
    /// testing against a server whose chain you can't otherwise trust. Prefer
    /// [`with_ca_certificate`](Self::with_ca_certificate).
    pub fn danger_accept_invalid_certs(mut self, insecure: bool) -> ClientOptions {
        self.tls_insecure = insecure;
        self
    }
}

/// Standard base64 encoding (with padding). Kept inline to avoid a dependency.
fn base64_encode(input: &[u8]) -> String {
    const ALPHABET: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(input.len().div_ceil(3) * 4);
    for chunk in input.chunks(3) {
        let b0 = chunk[0];
        let b1 = *chunk.get(1).unwrap_or(&0);
        let b2 = *chunk.get(2).unwrap_or(&0);
        let n = ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32);
        out.push(ALPHABET[((n >> 18) & 63) as usize] as char);
        out.push(ALPHABET[((n >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 { ALPHABET[((n >> 6) & 63) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { ALPHABET[(n & 63) as usize] as char } else { '=' });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_matches_known_vectors() {
        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_encode(b"f"), "Zg==");
        assert_eq!(base64_encode(b"fo"), "Zm8=");
        assert_eq!(base64_encode(b"foo"), "Zm9v");
        assert_eq!(base64_encode(b"foob"), "Zm9vYg==");
        assert_eq!(base64_encode(b"fooba"), "Zm9vYmE=");
        assert_eq!(base64_encode(b"foobar"), "Zm9vYmFy");
        // user:password style
        assert_eq!(base64_encode(b"iris:iris"), "aXJpczppcmlz");
    }

    #[test]
    fn basic_builds_authorization_value() {
        let opts = ClientOptions::basic("iris", "iris");
        assert_eq!(opts.authorization_value, "Basic aXJpczppcmlz");
    }
}

/// Fields shared between the [`Server`] handle and its keepalive task.
struct Inner {
    /// Current session token (refreshed on each authorized RPC / keepalive).
    token: Mutex<String>,
    extra_headers: Vec<(String, String)>,
    /// How long to wait between keepalive pings (already halved per server policy).
    expiration: Duration,
    /// Monotonic source for client-minted export ticket ids (starts at 1).
    next_ticket_id: AtomicI32,
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
        let use_tls = endpoint.starts_with("https://");

        let channel = if use_tls && options.tls_insecure {
            // Opt-in insecure path: TLS with certificate verification disabled.
            eprintln!(
                "WARNING: TLS certificate verification is DISABLED for {endpoint} \
                 (insecure mode). Do not use against untrusted networks."
            );
            crate::tls::connect_insecure(endpoint, options.tls_domain.clone()).await?
        } else {
            let mut builder =
                Channel::from_shared(endpoint).map_err(|e| Error::InvalidUri(e.to_string()))?;

            // https:// => TLS via rustls using the OS trust store (plus any extra
            // CA the caller supplied). Required for Enterprise behind Envoy.
            if use_tls {
                let mut tls = ClientTlsConfig::new().with_native_roots();
                if let Some(pem) = &options.ca_cert_pem {
                    tls = tls.ca_certificate(Certificate::from_pem(pem.clone()));
                }
                if let Some(domain) = &options.tls_domain {
                    tls = tls.domain_name(domain.clone());
                }
                builder = builder.tls_config(tls)?;
            }

            builder.connect().await?
        };

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
            next_ticket_id: AtomicI32::new(1),
        });

        let mut server = Server { channel, inner: inner.clone(), keepalive: None };
        server.keepalive = Some(server.spawn_keepalive());
        Ok(server)
    }

    /// A clone of the underlying channel for building other service clients.
    pub fn channel(&self) -> Channel {
        self.channel.clone()
    }

    /// Mint a fresh client-controlled export ticket: the ASCII byte `'e'`
    /// followed by a little-endian 4-byte id. Port of `Server.MakeNewTicket`/
    /// `NewTicket`.
    pub fn new_export_ticket(&self) -> Vec<u8> {
        let id = self.inner.next_ticket_id.fetch_add(1, Ordering::SeqCst);
        let mut bytes = Vec::with_capacity(5);
        bytes.push(b'e');
        bytes.extend_from_slice(&id.to_le_bytes());
        bytes
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
