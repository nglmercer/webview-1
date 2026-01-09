//! Shared types used throughout the application
//!
//! This module contains enums, structs, and data types that are
//! used by multiple modules of the application.

use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Window commands that can be sent from JavaScript
#[napi]
pub enum WindowCommand {
  /// Close the window
  Close,
  /// Show the window
  Show,
  /// Hide the window
  Hide,
}

/// Webview application events
#[napi]
pub enum WebviewApplicationEvent {
  /// Window close event
  WindowCloseRequested,
  /// Application close event
  ApplicationCloseRequested,
}

/// HTTP header data
#[napi(object)]
pub struct HeaderData {
  /// The header key
  pub key: String,
  /// The header value
  pub value: Option<String>,
}

/// IPC message
#[napi(object)]
pub struct IpcMessage {
  /// The message body
  pub body: Buffer,
  /// The HTTP method of the message
  pub method: String,
  /// The HTTP headers of the message
  pub headers: Vec<HeaderData>,
  /// The URI of the message
  pub uri: String,
}

/// Application control flow (mapped from tao::ControlFlow)
#[napi(js_name = "ControlFlow")]
#[derive(Clone)]
pub enum JsControlFlow {
  /// The application will continue running
  Poll,
  /// The application will wait until the specified time
  WaitUntil,
  /// The application will exit
  Exit,
  /// The application will exit with the given exit code
  ExitWithCode,
}

/// Options for creating an application
#[napi(object)]
#[derive(Clone)]
pub struct ApplicationOptions {
  /// The control flow of the application. Default is `Poll`
  pub control_flow: Option<JsControlFlow>,
  /// The waiting time in ms for the application (only applicable if control_flow is set to `WaitUntil`)
  pub wait_time: Option<i32>,
  /// The exit code of the application. Only applicable if control_flow is set to `ExitWithCode`
  pub exit_code: Option<i32>,
}

/// Event for the application
#[napi(object)]
pub struct ApplicationEvent {
  /// The event type
  pub event: WebviewApplicationEvent,
}

/// Progress bar state
#[napi(js_name = "ProgressBarState")]
pub enum JsProgressBarState {
  None,
  Normal,
  /// Treated as normal in Linux and macOS
  Indeterminate,
  /// Treated as normal in Linux
  Paused,
  /// Treated as normal in Linux
  Error,
}

/// Progress bar
#[napi(object)]
pub struct JsProgressBar {
  /// The progress state
  pub state: Option<JsProgressBarState>,
  /// The progress value
  pub progress: Option<u32>,
}

/// Fullscreen type
#[napi]
#[derive(Clone, Copy)]
pub enum FullscreenType {
  /// Exclusive fullscreen
  Exclusive,
  /// Borderless fullscreen
  Borderless,
}

/// Dimensions
#[napi(object)]
pub struct Dimensions {
  /// The width of the size
  pub width: u32,
  /// The height of the size
  pub height: u32,
}

/// Position
#[napi(object)]
pub struct Position {
  /// The x position
  pub x: i32,
  /// The y position
  pub y: i32,
}

/// Video mode
#[napi(object, js_name = "VideoMode")]
pub struct JsVideoMode {
  /// The size of the video mode
  pub size: Dimensions,
  /// The bit depth of the video mode
  pub bit_depth: u16,
  /// The refresh rate of the video mode
  pub refresh_rate: u16,
}

/// Monitor
#[napi(object)]
pub struct Monitor {
  /// The name of the monitor
  pub name: Option<String>,
  /// The scale factor of the monitor
  pub scale_factor: f64,
  /// The size of the monitor
  pub size: Dimensions,
  /// The position of the monitor
  pub position: Position,
  /// The video modes of the monitor
  pub video_modes: Vec<JsVideoMode>,
}

/// Window theme
#[napi]
#[derive(Clone, Copy)]
pub enum Theme {
  /// Light theme
  Light,
  /// Dark theme
  Dark,
  /// System theme
  System,
}

/// Options for creating a browser window
#[napi(object)]
pub struct BrowserWindowOptions {
  /// Whether window is resizable. Default is `true`.
  pub resizable: Option<bool>,
  /// The window title.
  pub title: Option<String>,
  /// The width of window.
  pub width: Option<f64>,
  /// The height of window.
  pub height: Option<f64>,
  /// The x position of window.
  pub x: Option<f64>,
  /// The y position of window.
  pub y: Option<f64>,
  /// Whether or not window should be created with content protection mode.
  pub content_protection: Option<bool>,
  /// Whether or not window is always on top.
  pub always_on_top: Option<bool>,
  /// Whether or not window is always on bottom.
  pub always_on_bottom: Option<bool>,
  /// Whether or not window is visible.
  pub visible: Option<bool>,
  /// Whether or not window decorations are enabled.
  pub decorations: Option<bool>,
  /// Whether or not window is visible on all workspaces.
  pub visible_on_all_workspaces: Option<bool>,
  /// Whether or not window is maximized.
  pub maximized: Option<bool>,
  /// Whether or not window is maximizable.
  pub maximizable: Option<bool>,
  /// Whether or not window is minimizable.
  pub minimizable: Option<bool>,
  /// Whether or not window is focused.
  pub focused: Option<bool>,
  /// Whether or not window is transparent.
  pub transparent: Option<bool>,
  /// The fullscreen state of window.
  pub fullscreen: Option<FullscreenType>,
}

impl Default for BrowserWindowOptions {
  fn default() -> Self {
    Self {
      resizable: Some(true),
      title: Some("WebviewJS".to_owned()),
      width: Some(800.0),
      height: Some(600.0),
      x: Some(0.0),
      y: Some(0.0),
      content_protection: Some(false),
      always_on_top: Some(false),
      always_on_bottom: Some(false),
      visible: Some(true),
      decorations: Some(true),
      visible_on_all_workspaces: Some(false),
      maximized: Some(false),
      maximizable: Some(true),
      minimizable: Some(true),
      focused: Some(true),
      transparent: Some(false),
      fullscreen: None,
    }
  }
}

/// Options for creating a webview
#[napi(object)]
pub struct WebviewOptions {
  /// The URL to load.
  pub url: Option<String>,
  /// The HTML content to load.
  pub html: Option<String>,
  /// The width of window.
  pub width: Option<f64>,
  /// The height of window.
  pub height: Option<f64>,
  /// The x position of window.
  pub x: Option<f64>,
  /// The y position of window.
  pub y: Option<f64>,
  /// Whether to enable devtools. Default is `true`.
  pub enable_devtools: Option<bool>,
  /// Whether window is incognito. Default is `false`.
  pub incognito: Option<bool>,
  /// The default user agent.
  pub user_agent: Option<String>,
  /// Whether webview should be built as a child.
  pub child: Option<bool>,
  /// The preload script to inject.
  pub preload: Option<String>,
  /// Whether window is transparent. Default is `false`.
  pub transparent: Option<bool>,
  /// The default theme.
  pub theme: Option<Theme>,
  /// Whether window is zoomable via hotkeys or gestures.
  pub hotkeys_zoom: Option<bool>,
  /// Whether clipboard access is enabled.
  pub clipboard: Option<bool>,
  /// Whether autoplay policy is enabled.
  pub autoplay: Option<bool>,
  /// Indicates whether horizontal swipe gestures trigger backward and forward page navigation.
  pub back_forward_navigation_gestures: Option<bool>,
}

impl Default for WebviewOptions {
  fn default() -> Self {
    Self {
      url: None,
      html: None,
      width: None,
      height: None,
      x: None,
      y: None,
      enable_devtools: Some(true),
      incognito: Some(false),
      user_agent: Some("WebviewJS".to_owned()),
      child: Some(false),
      preload: None,
      transparent: Some(false),
      theme: None,
      hotkeys_zoom: Some(true),
      clipboard: Some(true),
      autoplay: Some(true),
      back_forward_navigation_gestures: Some(true),
    }
  }
}
