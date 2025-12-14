//! HTTP/2 config.

#![allow(missing_docs)]

pub use http2::frame::{
    ExperimentalSettings, Priorities, PrioritiesBuilder, Priority, PseudoId, PseudoOrder,
    PseudoOrderBuilder, Setting, SettingId, SettingsOrder, SettingsOrderBuilder,
    StreamDependency, StreamId,
};
// Re-export enum variants for convenience
pub use http2::frame::PseudoId::*;
pub use http2::frame::SettingId::*;
use std::borrow::Cow;

/// Builder for `Http2Config`.
#[must_use]
#[derive(Debug, Clone)]
pub struct Http2ConfigBuilder {
    opts: Http2Config,
}

/// Configuration for an HTTP/2 connection.
///
/// This struct defines various parameters to fine-tune the behavior of an HTTP/2 connection,
/// including stream management, window sizes, frame limits, and header config.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Http2Config {
    /// The initial stream ID for HTTP/2 communication.
    pub initial_stream_id: Option<u32>,

    /// The initial connection-level window size.
    pub initial_connection_window_size: Option<u32>,

    /// The size of the header compression table.
    pub header_table_size: Option<u32>,

    /// Enables or disables server push functionality.
    pub enable_push: Option<bool>,

    /// The maximum number of concurrent streams allowed.
    pub max_concurrent_streams: Option<u32>,

    /// The initial window size for stream-level flow control.
    pub initial_stream_window_size: Option<u32>,

    /// The maximum frame size allowed.
    pub max_frame_size: Option<u32>,

    /// The maximum size of header lists.
    pub max_header_list_size: Option<u32>,

    /// Placeholder for an unknown HTTP/2 setting with identifier `8`.
    pub unknown_setting8: Option<bool>,

    /// Placeholder for an unknown HTTP/2 setting with identifier `9`.
    pub unknown_setting9: Option<bool>,

    /// Whether to enable the CONNECT protocol.
    pub enable_connect_protocol: Option<bool>,

    /// Whether to disable RFC 7540 Stream Priorities.
    pub no_rfc7540_priorities: Option<bool>,

    /// The order in which settings are applied.
    pub settings_order: Option<SettingsOrder>,

    /// The priority settings for header frames.
    pub headers_stream_dependency: Option<StreamDependency>,

    /// The order of pseudo-header fields.
    pub headers_pseudo_order: Option<PseudoOrder>,

    /// Custom experimental HTTP/2 settings.
    pub experimental_settings: Option<ExperimentalSettings>,

    /// The priority configuration for priority frames.
    pub priorities: Option<Priorities>,
}

impl Default for Http2Config {
    fn default() -> Self {
        Self {
            initial_stream_id: None,
            initial_connection_window_size: None,
            header_table_size: None,
            enable_push: None,
            max_concurrent_streams: None,
            initial_stream_window_size: None,
            max_frame_size: None,
            max_header_list_size: None,
            unknown_setting8: None,
            unknown_setting9: None,
            enable_connect_protocol: None,
            no_rfc7540_priorities: None,
            settings_order: None,
            headers_stream_dependency: None,
            headers_pseudo_order: None,
            experimental_settings: None,
            priorities: None,
        }
    }
}

impl Http2ConfigBuilder {
    #[inline]
    pub fn initial_stream_id(mut self, id: impl Into<Option<u32>>) -> Self {
        if let Some(id) = id.into() {
            self.opts.initial_stream_id = Some(id);
        }
        self
    }

    #[inline]
    pub fn initial_connection_window_size(mut self, sz: impl Into<Option<u32>>) -> Self {
        if let Some(sz) = sz.into() {
            self.opts.initial_connection_window_size = Some(sz);
        }
        self
    }

    #[inline]
    pub fn header_table_size(mut self, size: impl Into<Option<u32>>) -> Self {
        if let Some(size) = size.into() {
            self.opts.header_table_size = Some(size);
        }
        self
    }

    #[inline]
    pub fn enable_push(mut self, opt: bool) -> Self {
        self.opts.enable_push = Some(opt);
        self
    }

    #[inline]
    pub fn max_concurrent_streams(mut self, max: impl Into<Option<u32>>) -> Self {
        if let Some(max) = max.into() {
            self.opts.max_concurrent_streams = Some(max);
        }
        self
    }

    #[inline]
    pub fn initial_stream_window_size(mut self, sz: impl Into<Option<u32>>) -> Self {
        if let Some(sz) = sz.into() {
            self.opts.initial_stream_window_size = Some(sz);
        }
        self
    }

    /// Alias for `initial_stream_window_size` to match wreq's API
    #[inline]
    pub fn initial_window_size(self, sz: impl Into<Option<u32>>) -> Self {
        self.initial_stream_window_size(sz)
    }

    #[inline]
    pub fn max_frame_size(mut self, sz: impl Into<Option<u32>>) -> Self {
        if let Some(sz) = sz.into() {
            self.opts.max_frame_size = Some(sz);
        }
        self
    }

    #[inline]
    pub fn max_header_list_size(mut self, max: u32) -> Self {
        self.opts.max_header_list_size = Some(max);
        self
    }

    #[inline]
    pub fn unknown_setting8(mut self, opt: bool) -> Self {
        self.opts.unknown_setting8 = Some(opt);
        self
    }

    #[inline]
    pub fn unknown_setting9(mut self, opt: bool) -> Self {
        self.opts.unknown_setting9 = Some(opt);
        self
    }

    #[inline]
    pub fn enable_connect_protocol(mut self, opt: bool) -> Self {
        self.opts.enable_connect_protocol = Some(opt);
        self
    }

    #[inline]
    pub fn no_rfc7540_priorities(mut self, opt: bool) -> Self {
        self.opts.no_rfc7540_priorities = Some(opt);
        self
    }

    #[inline]
    pub fn settings_order<T>(mut self, settings_order: T) -> Self
    where
        T: Into<Option<SettingsOrder>>,
    {
        if let Some(settings_order) = settings_order.into() {
            self.opts.settings_order = Some(settings_order);
        }
        self
    }

    #[inline]
    pub fn headers_priority<T>(mut self, stream_dependency: T) -> Self
    where
        T: Into<Option<StreamDependency>>,
    {
        if let Some(stream_dependency) = stream_dependency.into() {
            self.opts.headers_stream_dependency = Some(stream_dependency);
        }
        self
    }

    #[inline]
    pub fn headers_stream_dependency<T>(mut self, stream_dependency: T) -> Self
    where
        T: Into<Option<StreamDependency>>,
    {
        if let Some(stream_dependency) = stream_dependency.into() {
            self.opts.headers_stream_dependency = Some(stream_dependency);
        }
        self
    }

    #[inline]
    pub fn headers_pseudo_order<T>(mut self, headers_pseudo_order: T) -> Self
    where
        T: Into<Option<PseudoOrder>>,
    {
        if let Some(headers_pseudo_order) = headers_pseudo_order.into() {
            self.opts.headers_pseudo_order = Some(headers_pseudo_order);
        }
        self
    }

    #[inline]
    pub fn experimental_settings<T>(mut self, experimental_settings: T) -> Self
    where
        T: Into<Option<ExperimentalSettings>>,
    {
        if let Some(experimental_settings) = experimental_settings.into() {
            self.opts.experimental_settings = Some(experimental_settings);
        }
        self
    }

    #[inline]
    pub fn priority<T>(mut self, priorities: T) -> Self
    where
        T: Into<Option<Priorities>>,
    {
        if let Some(priorities) = priorities.into() {
            self.opts.priorities = Some(priorities);
        }
        self
    }

    #[inline]
    pub fn priorities<T>(mut self, priorities: T) -> Self
    where
        T: Into<Option<Priorities>>,
    {
        if let Some(priorities) = priorities.into() {
            self.opts.priorities = Some(priorities);
        }
        self
    }

    #[inline]
    pub fn build(self) -> Http2Config {
        self.opts
    }
}

impl Http2Config {
    /// Creates a new `Http2ConfigBuilder` instance.
    pub fn builder() -> Http2ConfigBuilder {
        Http2ConfigBuilder {
            opts: Http2Config::default(),
        }
    }
}

/// Type alias for backwards compatibility
pub type Http2Options = Http2Config;
pub type Http2OptionsBuilder = Http2ConfigBuilder;
