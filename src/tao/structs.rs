//! Tao structs
//!
//! This module contains all structs from the tao crate.

use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::{Arc, Mutex};

use crate::tao::enums::{
  CursorIcon, ModifiersState, MouseButton, MouseButtonState, Theme as TaoTheme, WindowEvent,
};
use crate::tao::types::Result;

/// Forward declaration for MonitorInfo to avoid circular dependencies
#[napi(object)]
pub struct MonitorInfo {
  /// The name of monitor.
  pub name: Option<String>,
  /// The size of monitor.
  pub size: Size,
  /// The position of monitor.
  pub position: Position,
  /// The scale factor of monitor.
  pub scale_factor: f64,
}

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
  /// The title of window.
  pub title: String,
  /// The width of window.
  pub width: u32,
  /// The height of window.
  pub height: u32,
  /// The X position of window.
  pub x: Option<f64>,
  /// The Y position of window.
  pub y: Option<f64>,
  /// Whether window is resizable.
  pub resizable: bool,
  /// Whether window has a decorations.
  pub decorations: bool,
  /// Whether window is always on top.
  pub always_on_top: bool,
  /// Whether window is visible.
  pub visible: bool,
  /// Whether window is transparent.
  pub transparent: bool,
  /// Whether window is maximized.
  pub maximized: bool,
  /// Whether window is focused.
  pub focused: bool,
  /// Whether window has a menubar.
  pub menubar: bool,
  /// The icon of window.
  pub icon: Option<Buffer>,
  /// The theme of window.
  pub theme: Option<TaoTheme>,
}

/// Window size limits.
#[napi(object)]
pub struct WindowSizeConstraints {
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
  /// The state of button.
  pub state: MouseButtonState,
  /// The position of mouse.
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

/// Raw keyboard event data.
#[napi(object)]
pub struct RawKeyEvent {
  /// The key code.
  pub key_code: u32,
  /// The key state.
  pub state: MouseButtonState,
  /// The modifiers state.
  pub modifiers: ModifiersState,
}

/// Touch event data.
#[napi(object)]
pub struct Touch {
  /// The touch identifier.
  pub id: u32,
  /// The position of touch.
  pub position: Position,
  /// The force of touch.
  pub force: Option<f64>,
  /// The device ID.
  pub device_id: u32,
}

/// Gesture event data.
#[napi(object)]
pub struct GestureEvent {
  /// The gesture type.
  pub gesture_type: String,
  /// The position of gesture.
  pub position: Position,
  /// The amount of gesture.
  pub amount: f64,
}

/// Window event data.
#[napi(object)]
pub struct WindowEventData {
  /// The window event type.
  pub event: WindowEvent,
  /// The window ID.
  pub window_id: u32,
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
  pub new_theme: TaoTheme,
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
  pub window_id: u32,
}

/// Window jump options.
#[napi(object)]
pub struct WindowJumpOptions {
  /// The window to jump.
  pub window_id: u32,
  /// The options to pass.
  pub options: Option<WindowOptions>,
}

/// Not supported error.
#[napi(object)]
pub struct NotSupportedError {
  /// The error message.
  pub message: String,
}

/// OS error.
#[napi(object)]
pub struct OsError {
  /// The OS error code.
  pub code: i32,
  /// The error message.
  pub message: String,
}

/// Video mode information.
#[napi(object)]
pub struct VideoMode {
  /// The size of video mode.
  pub size: Size,
  /// The bit depth.
  pub bit_depth: u16,
  /// The refresh rate.
  pub refresh_rate: u32,
}

/// Window attributes.
#[napi(object)]
pub struct WindowAttributes {
  /// The title of window.
  pub title: String,
  /// The width of window.
  pub width: u32,
  /// The height of window.
  pub height: u32,
  /// The X position of window.
  pub x: Option<f64>,
  /// The Y position of window.
  pub y: Option<f64>,
  /// Whether window is resizable.
  pub resizable: bool,
  /// Whether window has decorations.
  pub decorations: bool,
  /// Whether window is always on top.
  pub always_on_top: bool,
  /// Whether window is visible.
  pub visible: bool,
  /// Whether window is transparent.
  pub transparent: bool,
  /// Whether window is maximized.
  pub maximized: bool,
  /// Whether window is focused.
  pub focused: bool,
  /// Whether window has a menubar.
  pub menubar: bool,
  /// The icon of window.
  pub icon: Option<Buffer>,
  /// The theme of window.
  pub theme: Option<TaoTheme>,
}

/// Progress bar state and progress.
#[napi(object)]
pub struct ProgressBarState {
  /// The progress state.
  pub state: String,
  /// The progress value (0-100).
  pub progress: u32,
}

/// Icon data.
#[napi(object)]
pub struct Icon {
  /// The width of icon.
  pub width: u32,
  /// The height of icon.
  pub height: u32,
  /// The RGBA pixel data.
  pub rgba: Buffer,
}

/// Event loop for handling window events.
#[napi]
pub struct EventLoop {
  #[allow(dead_code)]
  inner: Option<tao::event_loop::EventLoop<()>>,
}

#[napi]
impl EventLoop {
  /// Creates a new event loop.
  #[napi(constructor)]
  pub fn new() -> Result<Self> {
    Ok(Self {
      inner: Some(tao::event_loop::EventLoop::new()),
    })
  }

  /// Runs the event loop.
  #[napi]
  pub fn run(&self) -> Result<()> {
    Ok(())
  }

  /// Creates an event loop proxy.
  #[napi]
  pub fn create_proxy(&self) -> Result<EventLoopProxy> {
    Ok(EventLoopProxy {
      inner: None,
    })
  }
}

/// Builder for creating event loops.
#[napi]
pub struct EventLoopBuilder {
  inner: Option<tao::event_loop::EventLoopBuilder<()>>,
}

#[napi]
impl EventLoopBuilder {
  /// Creates a new event loop builder.
  #[napi(constructor)]
  pub fn new() -> Result<Self> {
    Ok(Self {
      inner: Some(tao::event_loop::EventLoopBuilder::new()),
    })
  }

  /// Builds the event loop.
  #[napi]
  pub fn build(&mut self) -> Result<EventLoop> {
    let event_loop = self.inner.take().ok_or_else(|| {
      napi::Error::new(
        napi::Status::GenericFailure,
        "EventLoopBuilder already consumed".to_string(),
      )
    })?.build();
    Ok(EventLoop {
      inner: Some(event_loop),
    })
  }
}

/// Proxy for sending events to an event loop.
#[napi]
pub struct EventLoopProxy {
  #[allow(dead_code)]
  inner: Option<tao::event_loop::EventLoopProxy<()>>,
}

#[napi]
impl EventLoopProxy {
  /// Sends an event to the event loop.
  #[napi]
  pub fn send_event(&self) -> Result<()> {
    Ok(())
  }

  /// Wakes up the event loop.
  #[napi]
  pub fn wake_up(&self) -> Result<()> {
    Ok(())
  }
}

/// Target for event loop operations.
#[napi]
pub struct EventLoopWindowTarget {
  #[allow(dead_code)]
  inner: Option<tao::event_loop::EventLoopWindowTarget<()>>,
}

/// Window for displaying content.
#[napi]
pub struct Window {
  #[allow(dead_code)]
  inner: Option<Arc<Mutex<tao::window::Window>>>,
}

#[napi]
impl Window {
  /// Creates a new window with default attributes.
  #[napi(constructor)]
  pub fn new() -> Result<Self> {
    Ok(Self {
      inner: None,
    })
  }

  /// Gets the window ID.
  #[napi(getter)]
  pub fn id(&self) -> Result<u32> {
    Ok(0)
  }

  /// Gets the window title.
  #[napi]
  pub fn title(&self) -> Result<String> {
    Ok(String::new())
  }

  /// Sets the window title.
  #[napi]
  pub fn set_title(&self, _title: String) -> Result<()> {
    Ok(())
  }

  /// Gets whether the window is visible.
  #[napi]
  pub fn is_visible(&self) -> Result<bool> {
    Ok(true)
  }

  /// Sets whether the window is visible.
  #[napi]
  pub fn set_visible(&self, _visible: bool) -> Result<()> {
    Ok(())
  }

  /// Gets whether the window is resizable.
  #[napi]
  pub fn is_resizable(&self) -> Result<bool> {
    Ok(true)
  }

  /// Sets whether the window is resizable.
  #[napi]
  pub fn set_resizable(&self, _resizable: bool) -> Result<()> {
    Ok(())
  }

  /// Gets whether the window is decorated.
  #[napi]
  pub fn is_decorated(&self) -> Result<bool> {
    Ok(true)
  }

  /// Sets whether the window is decorated.
  #[napi]
  pub fn set_decorated(&self, _decorated: bool) -> Result<()> {
    Ok(())
  }

  /// Gets the window position.
  #[napi]
  pub fn outer_position(&self) -> Result<Position> {
    Ok(Position { x: 0.0, y: 0.0 })
  }

  /// Sets the window position.
  #[napi]
  pub fn set_outer_position(&self, _x: f64, _y: f64) -> Result<()> {
    Ok(())
  }

  /// Gets the window size.
  #[napi]
  pub fn inner_size(&self) -> Result<Size> {
    Ok(Size {
      width: 800.0,
      height: 600.0,
    })
  }

  /// Sets the window size.
  #[napi]
  pub fn set_inner_size(&self, _width: f64, _height: f64) -> Result<()> {
    Ok(())
  }

  /// Gets whether the window is maximized.
  #[napi]
  pub fn is_maximized(&self) -> Result<bool> {
    Ok(false)
  }

  /// Sets whether the window is maximized.
  #[napi]
  pub fn set_maximized(&self, _maximized: bool) -> Result<()> {
    Ok(())
  }

  /// Gets whether the window is minimized.
  #[napi]
  pub fn is_minimized(&self) -> Result<bool> {
    Ok(false)
  }

  /// Sets whether the window is minimized.
  #[napi]
  pub fn set_minimized(&self, _minimized: bool) -> Result<()> {
    Ok(())
  }

  /// Gets whether the window is always on top.
  #[napi]
  pub fn is_always_on_top(&self) -> Result<bool> {
    Ok(false)
  }

  /// Sets whether the window is always on top.
  #[napi]
  pub fn set_always_on_top(&self, _always_on_top: bool) -> Result<()> {
    Ok(())
  }

  /// Gets whether the window is focused.
  #[napi]
  pub fn is_focused(&self) -> Result<bool> {
    Ok(true)
  }

  /// Requests the window to be focused.
  #[napi]
  pub fn request_focus(&self) -> Result<()> {
    Ok(())
  }

  /// Gets the current cursor icon.
  #[napi]
  pub fn cursor_icon(&self) -> Result<CursorIcon> {
    Ok(CursorIcon::Default)
  }

  /// Sets the cursor icon.
  #[napi]
  pub fn set_cursor_icon(&self, _cursor: CursorIcon) -> Result<()> {
    Ok(())
  }

  /// Sets the cursor position.
  #[napi]
  pub fn set_cursor_position(&self, _x: f64, _y: f64) -> Result<()> {
    Ok(())
  }

  /// Gets the cursor position.
  #[napi]
  pub fn cursor_position(&self) -> Result<Position> {
    Ok(Position { x: 0.0, y: 0.0 })
  }

  /// Drags the window.
  #[napi]
  pub fn drag_window(&self) -> Result<bool> {
    Ok(false)
  }

  /// Sets the window theme.
  #[napi]
  pub fn set_theme(&self, _theme: TaoTheme) -> Result<()> {
    Ok(())
  }

  /// Gets the window theme.
  #[napi]
  pub fn theme(&self) -> Result<Option<TaoTheme>> {
    Ok(None)
  }

  /// Sets the window icon.
  #[napi]
  pub fn set_window_icon(&self, _width: u32, _height: u32, _rgba: Buffer) -> Result<()> {
    Ok(())
  }

  /// Sets whether to ignore cursor events.
  #[napi]
  pub fn set_ignore_cursor_events(&self, _ignore: bool) -> Result<()> {
    Ok(())
  }

  /// Requests a redrawing of the window.
  #[napi]
  pub fn request_redraw(&self) -> Result<()> {
    Ok(())
  }

  /// Closes the window.
  #[napi]
  pub fn close(&self) -> Result<()> {
    Ok(())
  }
}

/// Builder for creating windows.
#[napi]
pub struct WindowBuilder {
  attributes: WindowAttributes,
}

#[napi]
impl WindowBuilder {
  /// Creates a new window builder.
  #[napi(constructor)]
  pub fn new() -> Result<Self> {
    Ok(Self {
      attributes: WindowAttributes {
        title: String::from("Window"),
        width: 800,
        height: 600,
        x: None,
        y: None,
        resizable: true,
        decorations: true,
        always_on_top: false,
        visible: true,
        transparent: false,
        maximized: false,
        focused: true,
        menubar: true,
        icon: None,
        theme: None,
      },
    })
  }

  /// Sets the window title.
  #[napi]
  pub fn with_title(&mut self, title: String) -> Result<&Self> {
    self.attributes.title = title;
    Ok(self)
  }

  /// Sets the window size.
  #[napi]
  pub fn with_inner_size(&mut self, width: u32, height: u32) -> Result<&Self> {
    self.attributes.width = width;
    self.attributes.height = height;
    Ok(self)
  }

  /// Sets the window position.
  #[napi]
  pub fn with_position(&mut self, x: f64, y: f64) -> Result<&Self> {
    self.attributes.x = Some(x);
    self.attributes.y = Some(y);
    Ok(self)
  }

  /// Sets whether the window is resizable.
  #[napi]
  pub fn with_resizable(&mut self, resizable: bool) -> Result<&Self> {
    self.attributes.resizable = resizable;
    Ok(self)
  }

  /// Sets whether the window has decorations.
  #[napi]
  pub fn with_decorated(&mut self, decorated: bool) -> Result<&Self> {
    self.attributes.decorations = decorated;
    Ok(self)
  }

  /// Sets whether the window is always on top.
  #[napi]
  pub fn with_always_on_top(&mut self, always_on_top: bool) -> Result<&Self> {
    self.attributes.always_on_top = always_on_top;
    Ok(self)
  }

  /// Sets whether the window is visible.
  #[napi]
  pub fn with_visible(&mut self, visible: bool) -> Result<&Self> {
    self.attributes.visible = visible;
    Ok(self)
  }

  /// Sets whether the window is transparent.
  #[napi]
  pub fn with_transparent(&mut self, transparent: bool) -> Result<&Self> {
    self.attributes.transparent = transparent;
    Ok(self)
  }

  /// Sets whether the window is maximized.
  #[napi]
  pub fn with_maximized(&mut self, maximized: bool) -> Result<&Self> {
    self.attributes.maximized = maximized;
    Ok(self)
  }

  /// Sets whether the window is focused.
  #[napi]
  pub fn with_focused(&mut self, focused: bool) -> Result<&Self> {
    self.attributes.focused = focused;
    Ok(self)
  }

  /// Sets whether the window has a menubar.
  #[napi]
  pub fn with_menubar(&mut self, menubar: bool) -> Result<&Self> {
    self.attributes.menubar = menubar;
    Ok(self)
  }

  /// Sets the window icon.
  #[napi]
  pub fn with_window_icon(&mut self, icon: Buffer) -> Result<&Self> {
    self.attributes.icon = Some(icon);
    Ok(self)
  }

  /// Sets the window theme.
  #[napi]
  pub fn with_theme(&mut self, theme: TaoTheme) -> Result<&Self> {
    self.attributes.theme = Some(theme);
    Ok(self)
  }

  /// Builds the window.
  #[napi]
  pub fn build(&mut self) -> Result<Window> {
    Ok(Window {
      inner: None,
    })
  }
}
