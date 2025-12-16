//! Runtime utilities
//!
//! Re-exports from core::rt for backwards compatibility.

use std::pin::Pin;
use std::task::{Context, Poll};
use pin_project_lite::pin_project;

// Re-export from core/rt
pub use crate::core::rt::{
    ArcTimer, Executor, Sleep, Time, Timer, TokioExecutor, TokioTimer,
};

pin_project! {
    /// A wrapper that implements Tokio's IO traits.
    /// In the new architecture, this is a simple pass-through wrapper.
    #[derive(Debug)]
    pub struct TokioIo<T> {
        #[pin]
        inner: T,
    }
}

impl<T> TokioIo<T> {
    /// Wrap a type implementing Tokio's IO traits.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Borrow the inner type.
    #[inline(always)]
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// Mut borrow the inner type.
    #[inline(always)]
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Consume this wrapper and get the inner type.
    #[inline(always)]
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> tokio::io::AsyncRead for TokioIo<T>
where
    T: tokio::io::AsyncRead,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        self.project().inner.poll_read(cx, buf)
    }
}

impl<T> tokio::io::AsyncWrite for TokioIo<T>
where
    T: tokio::io::AsyncWrite,
{
    #[inline(always)]
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        self.project().inner.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        self.project().inner.poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        self.project().inner.poll_shutdown(cx)
    }

    fn is_write_vectored(&self) -> bool {
        self.inner.is_write_vectored()
    }

    #[inline(always)]
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> Poll<std::io::Result<usize>> {
        self.project().inner.poll_write_vectored(cx, bufs)
    }
}
