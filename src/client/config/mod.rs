//! HTTP protocol configuration modules.

pub mod http1;
pub mod http2;

pub use http1::Http1Config;
pub use http2::{Http2Config, Http2Options};
