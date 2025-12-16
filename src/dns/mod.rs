//! DNS resolution

#[cfg(feature = "hickory-dns")]
pub use hickory::{HickoryDnsResolver, LookupIpStrategy};
pub use resolve::{Addrs, Name, Resolve, Resolving};
pub(crate) use resolve::{DnsResolverWithOverrides, DynResolver};

pub(crate) mod gai;
#[cfg(feature = "hickory-dns")]
pub(crate) mod hickory;
pub(crate) mod resolve;

// Re-export types for core/client/connect modules
pub(crate) use crate::util::client::connect::dns::{
    GaiResolver,
    Name as HyperName,
    SocketAddrs,
    resolve,
    sealed::Resolve as InternalResolve,
};
