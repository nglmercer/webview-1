//! Wry enums
//!
//! This module contains all enums from the wry crate.

use napi_derive::napi;

/// Background throttling policy for webviews.
#[napi]
pub enum BackgroundThrottlingPolicy {
  /// Throttling is suspended when the page is in the background.
  Suspend,
  /// Throttling is not suspended when the page is in the background.
  Unsuspend,
  /// Throttling is suspended when the page is in the background and the webview is not visible.
  UnsuspendWhenFirstVisible,
}

/// Drag drop event.
#[napi]
pub enum DragDropEvent {
  /// The drag has entered the webview area.
  Entered,
  /// The drag is hovering over the webview area.
  Hovered,
  /// The drag has left the webview area.
  Left,
  /// The drag has been dropped on the webview.
  Dropped,
}

/// Error type for webview operations.
#[napi]
pub enum Error {
  /// The webview was not initialized.
  Uninitialized,
  /// The webview has already been destroyed.
  AlreadyDestroyed,
  /// The script call failed.
  ScriptCallFailed,
  /// An IPC error occurred.
  Ipc,
  /// The webview is invalid.
  InvalidWebview,
  /// The URL is invalid.
  InvalidUrl,
  /// The operation is not supported on this platform.
  Unsupported,
  /// The icon is invalid.
  InvalidIcon,
}

/// Response to a new window request.
#[napi]
pub enum NewWindowResponse {
  /// Deny the new window request.
  Deny,
  /// Allow the new window request.
  Allow,
  /// Allow the new window request and navigate to the URL.
  AllowAndNavigate,
}

/// Page load event.
#[napi]
pub enum PageLoadEvent {
  /// The page has started loading.
  Started,
  /// The page has completed loading.
  Completed,
}

/// Proxy configuration.
#[napi]
pub enum ProxyConfig {
  /// Direct connection (no proxy).
  None,
  /// HTTP proxy.
  Http(String),
  /// HTTPS proxy.
  Https(String),
  /// SOCKS5 proxy.
  Socks5(String),
}

/// Theme for the webview.
#[napi(js_name = "WryTheme")]
pub enum Theme {
  /// Light theme.
  Light,
  /// Dark theme.
  Dark,
  /// System theme.
  Auto,
}
