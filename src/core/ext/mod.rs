//! HTTP extensions.

mod config;
mod h1_reason_phrase;

pub(crate) use self::{
    config::{RequestConfig, RequestConfigValue, RequestLayerOptions, RequestOrigHeaderMap},
    h1_reason_phrase::ReasonPhrase,
};

/// HTTP/2 CONNECT protocol extension.
/// Re-exported from http2 crate.
pub use http2::ext::Protocol;
