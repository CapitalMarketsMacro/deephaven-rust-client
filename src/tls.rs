//! Opt-in **insecure** TLS connector: connects over TLS but skips all server
//! certificate verification. This is a security downgrade — only for internal
//! testing against servers whose certificate chain you can't otherwise trust
//! (e.g. a private CA you can't export). Prefer `ClientOptions::with_ca_certificate`.
//!
//! tonic's `ClientTlsConfig` doesn't expose a "disable verification" switch, so
//! we build a rustls `ClientConfig` with a no-op verifier and drive it through
//! `Endpoint::connect_with_connector`.

use std::sync::Arc;

use http::Uri;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use tokio_rustls::rustls::client::danger::{
    HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier,
};
use tokio_rustls::rustls::crypto::ring::default_provider;
use tokio_rustls::rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use tokio_rustls::rustls::{ClientConfig, DigitallySignedStruct, SignatureScheme};
use tokio_rustls::TlsConnector;
use tonic::transport::{Channel, Endpoint};

use crate::error::{Error, Result};

type BoxError = Box<dyn std::error::Error + Send + Sync>;

/// A certificate verifier that accepts everything. DANGEROUS.
#[derive(Debug)]
struct NoCertVerification;

impl ServerCertVerifier for NoCertVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> std::result::Result<ServerCertVerified, tokio_rustls::rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> std::result::Result<HandshakeSignatureValid, tokio_rustls::rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> std::result::Result<HandshakeSignatureValid, tokio_rustls::rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        default_provider().signature_verification_algorithms.supported_schemes()
    }
}

/// Connect to an `https://` endpoint over TLS with certificate verification
/// **disabled**. `domain` overrides the SNI server name if given.
pub async fn connect_insecure(endpoint: String, domain: Option<String>) -> Result<Channel> {
    let mut config = ClientConfig::builder_with_provider(Arc::new(default_provider()))
        .with_safe_default_protocol_versions()
        .map_err(|e| Error::Tls(e.to_string()))?
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(NoCertVerification))
        .with_no_client_auth();
    // gRPC requires HTTP/2; advertise it via ALPN (tonic's own TLS does this).
    config.alpn_protocols = vec![b"h2".to_vec()];
    let config = Arc::new(config);

    let ep = Endpoint::from_shared(endpoint).map_err(|e| Error::InvalidUri(e.to_string()))?;

    let connector = tower::service_fn(move |uri: Uri| {
        let config = config.clone();
        let domain = domain.clone();
        async move {
            let host = uri
                .host()
                .ok_or_else(|| -> BoxError { "endpoint URI has no host".into() })?
                .to_string();
            let port = uri.port_u16().unwrap_or(443);
            let tcp = TcpStream::connect((host.as_str(), port)).await?;
            let sni = domain.unwrap_or(host);
            let server_name =
                ServerName::try_from(sni).map_err(|e| -> BoxError { Box::new(e) })?;
            let tls = TlsConnector::from(config).connect(server_name, tcp).await?;
            Ok::<_, BoxError>(TokioIo::new(tls))
        }
    });

    ep.connect_with_connector(connector).await.map_err(Error::from)
}
