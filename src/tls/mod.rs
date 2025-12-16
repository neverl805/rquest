//! TLS configuration
//!
//! By default, a `Client` will make use of BoringSSL for TLS.
//!
//! - Various parts of TLS can also be configured or even disabled on the
//!   `ClientBuilder`.

mod config;
mod conn;
mod ext;
mod x509;

pub(crate) use self::conn::{HttpsConnector, MaybeHttpsStream, TlsConnector};
pub use self::{
    config::{IntoCertStore, TlsConfig, TlsOptions, TlsOptionsBuilder},
    x509::{CertStore, CertStoreBuilder, Certificate, CertificateInput, Identity},
};
pub use boring2::ssl::{CertificateCompressionAlgorithm, ExtensionType, SslCurve};
use boring2::ssl::SslVersion;

/// A TLS protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TlsVersion(SslVersion);

// These could perhaps be From/TryFrom implementations, but those would be
// part of the public API so let's be careful
impl TlsVersion {
    /// Version 1.0 of the TLS protocol.
    pub const TLS_1_0: TlsVersion = TlsVersion(SslVersion::TLS1);
    /// Version 1.1 of the TLS protocol.
    pub const TLS_1_1: TlsVersion = TlsVersion(SslVersion::TLS1_1);
    /// Version 1.2 of the TLS protocol.
    pub const TLS_1_2: TlsVersion = TlsVersion(SslVersion::TLS1_2);
    /// Version 1.3 of the TLS protocol.
    pub const TLS_1_3: TlsVersion = TlsVersion(SslVersion::TLS1_3);
}

/// A TLS ALPN protocol.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct AlpnProtos(&'static [u8]);

/// A `AlpnProtos` is used to set the HTTP version preference.
impl AlpnProtos {
    /// Prefer HTTP/1.1
    pub const HTTP1: AlpnProtos = AlpnProtos(b"\x08http/1.1");
    /// Prefer HTTP/2
    pub const HTTP2: AlpnProtos = AlpnProtos(b"\x02h2");
    /// Prefer HTTP/1 and HTTP/2
    pub const ALL: AlpnProtos = AlpnProtos(b"\x02h2\x08http/1.1");
}

impl Default for AlpnProtos {
    fn default() -> Self {
        Self::ALL
    }
}

/// A TLS ALPN protocol identifier.
/// Used for specifying application layer protocol negotiation.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct AlpnProtocol(&'static [u8]);

impl AlpnProtocol {
    /// HTTP/1.1 ALPN protocol
    pub const HTTP1: AlpnProtocol = AlpnProtocol(b"http/1.1");
    /// HTTP/2 ALPN protocol
    pub const HTTP2: AlpnProtocol = AlpnProtocol(b"h2");
    /// HTTP/3 ALPN protocol
    pub const HTTP3: AlpnProtocol = AlpnProtocol(b"h3");

    /// Create a new `AlpnProtocol` from a static byte slice.
    pub const fn new(value: &'static [u8]) -> Self {
        AlpnProtocol(value)
    }

    /// Encode this protocol identifier with length prefix.
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(1 + self.0.len());
        buf.push(self.0.len() as u8);
        buf.extend_from_slice(self.0);
        buf
    }

    /// Encode a sequence of ALPN protocols into a single buffer.
    pub fn encode_sequence<'a, I>(protocols: I) -> Vec<u8>
    where
        I: IntoIterator<Item = &'a AlpnProtocol>,
    {
        let mut buf = Vec::new();
        for protocol in protocols {
            buf.push(protocol.0.len() as u8);
            buf.extend_from_slice(protocol.0);
        }
        buf
    }
}

/// Application-layer protocol settings for HTTP/1.1 and HTTP/2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlpsProtos(&'static [u8]);

impl AlpsProtos {
    /// Application Settings protocol for HTTP/1.1
    pub const HTTP1: AlpsProtos = AlpsProtos(b"http/1.1");
    /// Application Settings protocol for HTTP/2
    pub const HTTP2: AlpsProtos = AlpsProtos(b"h2");
}

/// Hyper extension carrying extra TLS layer information.
/// Made available to clients on responses when `tls_info` is set.
#[derive(Debug, Clone)]
pub struct TlsInfo {
    pub(crate) peer_certificate: Option<Vec<u8>>,
}

impl TlsInfo {
    /// Get the DER encoded leaf certificate of the peer.
    pub fn peer_certificate(&self) -> Option<&[u8]> {
        self.peer_certificate.as_ref().map(|der| &der[..])
    }
}
