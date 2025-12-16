use std::sync::Arc;

use http::{Uri, Version};

use crate::hash::HashMemo;
use crate::tls::AlpnProtocol;

/// Uniquely identifies a connection configuration and its lifecycle.
///
/// [`Identifier`] serves as the unique key for a connection, representing all parameters
/// that define its identity (URI, protocol, proxy, TCP/TLS options). It is used for pooling,
/// caching, and tracking connections throughout their entire lifecycle.
pub(crate) type Identifier = Arc<HashMemo<ConnectExtra>>;

/// Metadata describing a reusable network connection.
///
/// [`ConnectExtra`] holds connection-specific parameters such as the target URI and
/// enforced HTTP version. Used for connection pooling and identification.
#[must_use]
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub(crate) struct ConnectExtra {
    /// Target URI.
    uri: Uri,
    /// Enforced HTTP version (if any).
    enforced_version: Option<Version>,
}

// ===== impl ConnectExtra =====

impl ConnectExtra {
    /// Create a new [`ConnectExtra`] with the given URI and enforced version.
    #[inline]
    pub fn new(uri: Uri, enforced_version: Option<Version>) -> Self {
        Self { uri, enforced_version }
    }

    /// Get the URI.
    #[inline]
    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    /// Return the negotiated [`AlpnProtocol`] based on the enforced version.
    pub fn alpn_protocol(&self) -> Option<AlpnProtocol> {
        match self.enforced_version {
            Some(Version::HTTP_11 | Version::HTTP_10 | Version::HTTP_09) => {
                Some(AlpnProtocol::HTTP1)
            }
            Some(Version::HTTP_2) => Some(AlpnProtocol::HTTP2),
            _ => None,
        }
    }
}
