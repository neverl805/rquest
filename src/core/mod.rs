//! http client protocol implementation and low level utilities.

mod error;

pub mod client;
pub mod ext;
pub mod rt;

pub use self::error::{BoxError, Error, Result};

// Re-export upgrade module for client usage
pub use self::client::upgrade;
