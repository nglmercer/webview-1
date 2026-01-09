//! Wry structs
//!
//! This module contains all structs from the wry crate.

use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::wry::enums::Theme as WryTheme;

/// An initialization script to be run when creating a webview.
#[napi(object)]
pub struct InitializationScript {
  /// The JavaScript code to run.
  pub js: String,
  /// Whether to run the script only once.
  pub once: bool,
}

/// Features to configure a new window.
#[napi(object)]
pub struct NewWindowFeatures {
  /// Whether the new window should have a menubar.
  pub menubar: bool,
  /// Whether the new window should be visible.
  pub visible: bool,
  /// The width of the new window.
  pub width: u32,
  /// The height of the new window.
  pub height: u32,
  /// The X coordinate of the new window.
  pub x: i32,
  /// The Y coordinate of the new window.
  pub y: i32,
  /// Whether the new window should be maximized.
  pub maximized: bool,
  /// Whether the new window should be focused.
  pub focused: bool,
  /// Whether the new window should have decorations.
  pub decorations: bool,
  /// Whether the new window should always be on top.
  pub always_on_top: bool,
  /// Whether the new window should be transparent.
  pub transparent: bool,
}

/// The opener of a new window.
#[napi(object)]
pub struct NewWindowOpener {
  /// The label of the opener webview.
  pub label: String,
  /// The native ID of the opener webview.
  pub native_id: u32,
}

/// A proxy endpoint for web content.
#[napi(object)]
pub struct ProxyEndpoint {
  /// The host of the proxy.
  pub host: String,
  /// The port of the proxy.
  pub port: u16,
}

/// A rectangle area.
#[napi(object)]
pub struct Rect {
  /// The X coordinate of the top-left corner.
  pub x: i32,
  /// The Y coordinate of the top-left corner.
  pub y: i32,
  /// The width of the rectangle.
  pub width: u32,
  /// The height of the rectangle.
  pub height: u32,
}

/// A responder for a request.
#[napi(object)]
pub struct RequestAsyncResponder {
  /// The URI of the request.
  pub uri: String,
  /// The HTTP method of the request.
  pub method: String,
  /// The body of the request.
  pub body: Buffer,
}

/// The web context for a webview.
#[napi(object)]
pub struct WebContext {
  /// The URL that is currently being navigated to.
  pub url: Option<String>,
  /// The title of the currently loaded page.
  pub title: Option<String>,
  /// Whether the webview is loading content.
  pub is_loading: bool,
}

/// The main webview struct.
#[napi(object)]
pub struct WebView {
  /// The native ID of the webview.
  pub id: u32,
  /// The label of the webview.
  pub label: String,
}

/// Attributes for creating a webview.
#[napi(object)]
pub struct WebViewAttributes {
  /// The URL to load.
  pub url: Option<String>,
  /// The HTML content to load.
  pub html: Option<String>,
  /// The width of the webview.
  pub width: u32,
  /// The height of the webview.
  pub height: u32,
  /// The X coordinate of the webview.
  pub x: i32,
  /// The Y coordinate of the webview.
  pub y: i32,
  /// Whether the webview is resizable.
  pub resizable: bool,
  /// The title of the webview.
  pub title: Option<String>,
  /// Whether the webview has a menubar.
  pub menubar: bool,
  /// Whether the webview is maximized.
  pub maximized: bool,
  /// Whether the webview is minimized.
  pub minimized: bool,
  /// Whether the webview is visible.
  pub visible: bool,
  /// Whether the webview has decorations.
  pub decorations: bool,
  /// Whether the webview is always on top.
  pub always_on_top: bool,
  /// Whether the webview is transparent.
  pub transparent: bool,
  /// Whether the webview has focus.
  pub focused: bool,
  /// The icon of the webview.
  pub icon: Option<Buffer>,
  /// The theme of the webview.
  pub theme: Option<WryTheme>,
  /// The user agent of the webview.
  pub user_agent: Option<String>,
  /// Initialization scripts to run.
  pub initialization_scripts: Vec<InitializationScript>,
  /// Whether to enable drag drop.
  pub drag_drop: bool,
  /// The background color of the webview.
  pub background_color: Option<Buffer>,
}
