#![deny(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! Webview N-API Bindings
//!
//! This library provides N-API bindings for using tao and wry
//! in Node.js applications. All methods, APIs, enums, and types are exported
//! directly for Node.js composition.

// Wry bindings
pub mod wry;

// Tao bindings
pub mod tao;

// Re-export wry types
pub use wry::enums::{
  BackgroundThrottlingPolicy, DragDropEvent, Error, NewWindowResponse, PageLoadEvent, ProxyConfig,
  Theme as WryTheme,
};
pub use wry::functions::webview_version;
pub use wry::structs::{
  InitializationScript, NewWindowFeatures, NewWindowOpener, ProxyEndpoint, Rect,
  RequestAsyncResponder, WebContext, WebView, WebViewAttributes, WebViewBuilder,
};
pub use wry::types::{Result, WebViewId, RGBA};

// Re-export tao types
pub use tao::enums::{
  ControlFlow, CursorIcon, DeviceEvent, ElementState, Force, Fullscreen as FullscreenType, Key,
  KeyCode, KeyLocation, ModifiersState, MouseButton, MouseButtonState, ProgressState,
  ResizeDirection, StartCause, Theme as TaoTheme, TouchPhase, UserAttentionType, WindowEvent,
};
pub use tao::functions::{available_monitors, primary_monitor, tao_version};
pub use tao::structs::{
  CursorPosition, EventLoop, EventLoopBuilder, EventLoopProxy, EventLoopWindowTarget, GestureEvent,
  HiDpiScaling, Icon, KeyboardEvent, MonitorInfo, MouseEvent, NotSupportedError, OsError, Position,
  ProgressBarState, RawKeyEvent, Rectangle, ResizeDetails, ScaleFactorChangeDetails, Size,
  ThemeChangeDetails, Touch, VideoMode, Window, WindowAttributes, WindowBuilder, WindowDragOptions,
  WindowJumpOptions, WindowOptions, WindowSizeConstraints,
};
pub use tao::types::{AxisId, ButtonId, DeviceId, Result as TaoResult, WindowId, RGBA as TaoRGBA};
