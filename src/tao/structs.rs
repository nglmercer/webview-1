//! Tao structs
//!
//! This module contains all structs from the tao crate.

use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::tao::enums::{
  CursorIcon, FullscreenType, ModifiersState, MouseButton, MouseButtonState, Theme, WindowLevel,
  WindowEvent,
};

/// 2D position.
#[napi(object)]
pub struct Position {
  /// The X coordinate.
  pub x: f64,
  /// The Y coordinate.
  pub y: f64,
}

/// 2D size.
#[napi(object)]
pub struct Size {
  /// The width.
  pub width: f64,
  /// The height.
  pub height: f64,
}

/// 2D rectangle.
#[napi(object)]
pub struct Rectangle {
  /// The position.
  pub origin: Position,
  /// The size.
  pub size: Size,
}

/// Window options for creating a window.
#[napi(object)]
pub struct WindowOptions {
  /// The title of the window.
  pub title: String,
  /// The width of the window.
  pub width: u32,
  /// The height of the window.
  pub height: u32,
  /// The X position of the window.
  pub x: Option<f64>,
  /// The Y position of the window.
  pub y: Option<f64>,
  /// Whether the window is resizable.
  pub resizable: bool,
  /// Whether the window has a decorations.
  pub decorations: bool,
  /// Whether the window is always on top.
  pub always_on_top: bool,
  /// Whether the window is visible.
  pub visible: bool,
  /// Whether the window is transparent.
  pub transparent: bool,
  /// Whether the window is maximized.
  pub maximized: bool,
  /// Whether the window is focused.
  pub focused: bool,
  /// Whether the window has a menubar.
  pub menubar: bool,
  /// The icon of the window.
  pub icon: Option<Buffer>,
  /// The theme of the window.
  pub theme: Option<Theme>,
}

/// Window size limits.
#[napi(object)]
pub struct WindowSizeLimits {
  /// The minimum width.
  pub min_width: Option<u32>,
  /// The minimum height.
  pub min_height: Option<u32>,
  /// The maximum width.
  pub max_width: Option<u32>,
  /// The maximum height.
  pub max_height: Option<u32>,
}

/// Cursor position.
#[napi(object)]
pub struct CursorPosition {
  /// The X coordinate.
  pub x: f64,
  /// The Y coordinate.
  pub y: f64,
}

/// Mouse event data.
#[napi(object)]
pub struct MouseEvent {
  /// The button that was pressed/released.
  pub button: MouseButton,
  /// The state of the button.
  pub state: MouseButtonState,
  /// The position of the mouse.
  pub position: Position,
  /// The number of clicks.
  pub click_count: u16,
  /// The modifiers state.
  pub modifiers: ModifiersState,
}

/// Keyboard event data.
#[napi(object)]
pub struct KeyboardEvent {
  /// The key that was pressed.
  pub key: String,
  /// The key code.
  pub code: String,
  /// The key state.
  pub state: MouseButtonState,
  /// The modifiers state.
  pub modifiers: ModifiersState,
}

/// Touch event data.
#[napi(object)]
pub struct TouchEvent {
  /// The touch identifier.
  pub id: u64,
  /// The position of the touch.
  pub position: Position,
  /// The force of the touch.
  pub force: f64,
}

/// Gesture event data.
#[napi(object)]
pub struct GestureEvent {
  /// The gesture type.
  pub gesture_type: String,
  /// The position of the gesture.
  pub position: Position,
  /// The amount of the gesture.
  pub amount: f64,
}

/// Window event data.
#[napi(object)]
pub struct WindowEventData {
  /// The window event type.
  pub event: WindowEvent,
  /// The window ID.
  pub window_id: u64,
}

/// Monitor information.
#[napi(object)]
pub struct MonitorInfo {
  /// The name of the monitor.
  pub name: Option<String>,
  /// The size of the monitor.
  pub size: Size,
  /// The position of the monitor.
  pub position: Position,
  /// The scale factor of the monitor.
  pub scale_factor: f64,
}

/// HiDPI scaling information.
#[napi(object)]
pub struct HiDpiScaling {
  /// The scale factor.
  pub scale_factor: f64,
  /// The position in pixels.
  pub position_in_pixels: Position,
}

/// Theme change details.
#[napi(object)]
pub struct ThemeChangeDetails {
  /// The new theme.
  pub new_theme: Theme,
}

/// Cursor icon change details.
#[napi(object)]
pub struct CursorChangeDetails {
  /// The new cursor icon.
  pub new_cursor: CursorIcon,
}

/// Window scale factor change details.
#[napi(object)]
pub struct ScaleFactorChangeDetails {
  /// The new scale factor.
  pub scale_factor: f64,
  /// The new inner size in logical pixels.
  pub new_inner_size: Size,
}

/// Window resize details.
#[napi(object)]
pub struct ResizeDetails {
  /// The new width.
  pub width: u32,
  /// The new height.
  pub height: u32,
}

/// Window drag details.
#[napi(object)]
pub struct WindowDragOptions {
  /// The window to drag.
  pub window_id: u64,
}

/// Window jump options.
#[napi(object)]
pub struct WindowJumpOptions {
  /// The window to jump.
  pub window_id: u64,
  /// The options to pass.
  pub options: Option<WindowOptions>,
}
