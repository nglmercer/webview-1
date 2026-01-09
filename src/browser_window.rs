use napi::{Either, Env, Result};
use napi_derive::*;
use std::cell::RefCell;
use std::rc::Rc;
use tao::{
  dpi::{LogicalPosition, PhysicalSize},
  event_loop::EventLoop,
  window::{Fullscreen, Icon, ProgressBarState, Window, WindowBuilder},
};

use crate::ipc;
use crate::webview::{JsWebview, Theme, WebviewOptions};

// #[cfg(target_os = "windows")]
// use tao::platform::windows::IconExtWindows;

#[napi]
#[derive(serde_derive::Serialize)]
pub enum FullscreenType {
  /// Exclusive fullscreen.
  Exclusive,
  /// Borderless fullscreen.
  Borderless,
}

#[napi(object)]
pub struct Dimensions {
  /// The width of the size.
  pub width: u32,
  /// The height of the size.
  pub height: u32,
}

#[napi(object)]
pub struct Position {
  /// The x position.
  pub x: i32,
  /// The y position.
  pub y: i32,
}

#[napi(object, js_name = "VideoMode")]
pub struct JsVideoMode {
  /// The size of the video mode.
  pub size: Dimensions,
  /// The bit depth of the video mode.
  pub bit_depth: u16,
  /// The refresh rate of the video mode.
  pub refresh_rate: u16,
}

#[napi(object)]
pub struct Monitor {
  /// The name of the monitor.
  pub name: Option<String>,
  /// The scale factor of the monitor.
  pub scale_factor: f64,
  /// The size of the monitor.
  pub size: Dimensions,
  /// The position of the monitor.
  pub position: Position,
  /// The video modes of the monitor.
  pub video_modes: Vec<JsVideoMode>,
}

#[napi(js_name = "ProgressBarState")]
pub enum JsProgressBarState {
  None,
  Normal,
  /// Treated as normal in linux and macos
  Indeterminate,
  /// Treated as normal in linux
  Paused,
  /// Treated as normal in linux
  Error,
}

#[napi(object)]
pub struct JsProgressBar {
  /// The progress state.
  pub state: Option<JsProgressBarState>,
  /// The progress value.
  pub progress: Option<u32>,
}

#[napi(object)]
#[derive(serde_derive::Serialize)]
pub struct BrowserWindowOptions {
  /// Whether the window is resizable. Default is `true`.
  pub resizable: Option<bool>,
  /// The window title.
  pub title: Option<String>,
  /// The width of the window.
  pub width: Option<f64>,
  /// The height of the window.
  pub height: Option<f64>,
  /// The x position of the window.
  pub x: Option<f64>,
  /// The y position of the window.
  pub y: Option<f64>,
  /// Whether or not the window should be created with content protection mode.
  pub content_protection: Option<bool>,
  /// Whether or not the window is always on top.
  pub always_on_top: Option<bool>,
  /// Whether or not the window is always on bottom.
  pub always_on_bottom: Option<bool>,
  /// Whether or not the window is visible.
  pub visible: Option<bool>,
  /// Whether or not the window decorations are enabled.
  pub decorations: Option<bool>,
  /// Whether or not the window is visible on all workspaces
  pub visible_on_all_workspaces: Option<bool>,
  /// Whether or not the window is maximized.
  pub maximized: Option<bool>,
  /// Whether or not the window is maximizable
  pub maximizable: Option<bool>,
  /// Whether or not the window is minimizable
  pub minimizable: Option<bool>,
  /// Whether or not the window is focused
  pub focused: Option<bool>,
  /// Whether or not the window is transparent
  pub transparent: Option<bool>,
  /// The fullscreen state of the window.
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

#[napi]
pub struct BrowserWindow {
  is_child_window: bool,
  window: Option<Window>,
  /// Unique identifier for this window
  id: u32,
  /// IPC client for communicating with eventloop process (only in IPC mode)
  ipc_client: Option<Rc<RefCell<Option<ipc::IpcClient>>>>,
}

#[napi]
impl BrowserWindow {
  pub fn new(
    event_loop: &EventLoop<()>,
    options: Option<BrowserWindowOptions>,
    child: bool,
    window_id: u32,
  ) -> Result<Self> {
    let options = options.unwrap_or_default();

    let mut window = WindowBuilder::new();

    if let Some(resizable) = options.resizable {
      window = window.with_resizable(resizable);
    }

    if let Some(width) = options.width {
      window = window.with_inner_size(PhysicalSize::new(width, options.height.unwrap()));
    }

    if let Some(x) = options.x {
      window = window.with_position(LogicalPosition::new(x, options.y.unwrap()));
    }

    if let Some(visible) = options.visible {
      window = window.with_visible(visible);
    }

    if let Some(decorations) = options.decorations {
      window = window.with_decorations(decorations);
    }

    if let Some(always_on_top) = options.always_on_top {
      window = window.with_always_on_top(always_on_top);
    }

    if let Some(always_on_bottom) = options.always_on_bottom {
      window = window.with_always_on_bottom(always_on_bottom);
    }

    if let Some(visible_on_all_workspaces) = options.visible_on_all_workspaces {
      window = window.with_visible_on_all_workspaces(visible_on_all_workspaces);
    }

    if let Some(maximized) = options.maximized {
      window = window.with_maximized(maximized);
    }

    if let Some(maximizable) = options.maximizable {
      window = window.with_maximizable(maximizable);
    }

    if let Some(minimizable) = options.minimizable {
      window = window.with_minimizable(minimizable);
    }

    if let Some(focused) = options.focused {
      window = window.with_focused(focused);
    }

    if let Some(transparent) = options.transparent {
      window = window.with_transparent(transparent);
      #[cfg(target_os = "windows")]
      {
        use tao::platform::windows::WindowBuilderExtWindows;
        window = window.with_undecorated_shadow(false);
      }
    }

    if let Some(fullscreen) = options.fullscreen {
      let fs = match fullscreen {
        // Some(FullscreenType::Exclusive) => Some(Fullscreen::Exclusive()),
        FullscreenType::Borderless => Some(Fullscreen::Borderless(None)),
        _ => None,
      };

      window = window.with_fullscreen(fs);
    }

    if let Some(title) = options.title {
      window = window.with_title(&title);
    }

    let window = window.build(event_loop).map_err(|e| {
      napi::Error::new(
        napi::Status::GenericFailure,
        format!("Failed to create window: {}", e),
      )
    })?;

    Ok(Self {
      window: Some(window),
      is_child_window: child,
      id: window_id,
      ipc_client: None,
    })
  }

  /// Crea un BrowserWindow proxy que se comunica vía IPC con el proceso del eventloop
  pub fn new_ipc_proxy(window_id: u32, ipc_client: Rc<RefCell<Option<ipc::IpcClient>>>) -> Self {
    Self {
      window: None,
      is_child_window: false,
      id: window_id,
      ipc_client: Some(ipc_client),
    }
  }

  /// Verifica si esta ventana está en modo IPC
  fn is_ipc_mode(&self) -> bool {
    self.ipc_client.is_some()
  }

  #[napi]
  /// Creates a webview on this window.
  pub fn create_webview(&mut self, env: Env, options: Option<WebviewOptions>) -> Result<JsWebview> {
    if self.is_ipc_mode() {
      // Modo IPC: enviar solicitud al proceso del eventloop
      let ipc_client = self.ipc_client.as_ref().unwrap();
      let client_ref = ipc_client.borrow();
      let client = client_ref.as_ref().ok_or_else(|| {
        napi::Error::new(napi::Status::GenericFailure, "IPC client not initialized")
      })?;

      let options_json = serde_json::to_value(options.unwrap_or_default()).map_err(|e| {
        napi::Error::new(
          napi::Status::GenericFailure,
          format!("Failed to serialize options: {}", e),
        )
      })?;

      let request = ipc::IpcRequest::CreateWebview {
        window_id: self.id,
        options: options_json,
      };

      client.send_request(request).map_err(|e| {
        napi::Error::new(
          napi::Status::GenericFailure,
          format!("Failed to send IPC request: {}", e),
        )
      })?;

      // Retornar un JsWebview proxy
      Ok(JsWebview::new_ipc_proxy(
        self.id,
        self.ipc_client.clone().unwrap(),
      ))
    } else {
      // Modo tradicional: crear webview directamente
      let window = self
        .window
        .as_ref()
        .ok_or_else(|| napi::Error::new(napi::Status::GenericFailure, "Window not initialized"))?;
      let webview = JsWebview::create(&env, window, options.unwrap_or_default())?;
      Ok(webview)
    }
  }

  #[napi(getter)]
  /// Whether or not the window is a child window.
  pub fn is_child(&self) -> bool {
    self.is_child_window
  }

  #[napi(getter)]
  /// Gets the unique identifier for this window.
  pub fn id(&self) -> u32 {
    self.id
  }

  #[napi]
  /// Destroys the window by hiding it permanently.
  /// This is a more permanent close than hide(), as it indicates the window
  /// should not be reopened. Use this when you want to close a specific window
  /// (like a login window) permanently.
  pub fn destroy(&self) {
    if self.is_ipc_mode() {
      // Modo IPC: enviar solicitud
      if let Some(ipc_client) = &self.ipc_client {
        let client_ref = ipc_client.borrow();
        if let Some(client) = client_ref.as_ref() {
          let _ = client.send_request_async(ipc::IpcRequest::SetWindowVisible {
            window_id: self.id,
            visible: false,
          });
        }
      }
    } else if let Some(window) = &self.window {
      window.set_visible(false);
    }
  }

  #[napi]
  /// Whether the window is focused.
  pub fn is_focused(&self) -> bool {
    if self.is_ipc_mode() {
      // En modo IPC, retornamos true por defecto
      true
    } else if let Some(window) = &self.window {
      window.is_focused()
    } else {
      false
    }
  }

  #[napi]
  /// Whether the window is visible.
  pub fn is_visible(&self) -> bool {
    if self.is_ipc_mode() {
      // En modo IPC, retornamos true por defecto
      true
    } else if let Some(window) = &self.window {
      window.is_visible()
    } else {
      false
    }
  }

  #[napi]
  /// Whether the window is decorated.
  pub fn is_decorated(&self) -> bool {
    if self.is_ipc_mode() {
      // En modo IPC, retornamos true por defecto
      true
    } else if let Some(window) = &self.window {
      window.is_decorated()
    } else {
      false
    }
  }

  #[napi]
  /// Whether the window is closable.
  pub fn is_closable(&self) -> bool {
    if self.is_ipc_mode() {
      // En modo IPC, retornamos true por defecto
      true
    } else if let Some(window) = &self.window {
      window.is_closable()
    } else {
      false
    }
  }

  #[napi]
  /// Whether the window is maximizable.
  pub fn is_maximizable(&self) -> bool {
    if self.is_ipc_mode() {
      // En modo IPC, retornamos true por defecto
      true
    } else if let Some(window) = &self.window {
      window.is_maximizable()
    } else {
      false
    }
  }

  #[napi]
  /// Whether the window is minimizable.
  pub fn is_minimizable(&self) -> bool {
    if self.is_ipc_mode() {
      // En modo IPC, retornamos true por defecto
      true
    } else if let Some(window) = &self.window {
      window.is_minimizable()
    } else {
      false
    }
  }

  #[napi]
  /// Whether the window is maximized.
  pub fn is_maximized(&self) -> bool {
    if self.is_ipc_mode() {
      // En modo IPC, retornamos false por defecto
      false
    } else if let Some(window) = &self.window {
      window.is_maximized()
    } else {
      false
    }
  }

  #[napi]
  /// Whether the window is minimized.
  pub fn is_minimized(&self) -> bool {
    if self.is_ipc_mode() {
      // En modo IPC, retornamos false por defecto
      false
    } else if let Some(window) = &self.window {
      window.is_minimized()
    } else {
      false
    }
  }

  #[napi]
  /// Whether the window is resizable.
  pub fn is_resizable(&self) -> bool {
    if self.is_ipc_mode() {
      // En modo IPC, retornamos true por defecto
      true
    } else if let Some(window) = &self.window {
      window.is_resizable()
    } else {
      false
    }
  }

  #[napi]
  /// Sets the window title.
  pub fn set_title(&self, title: String) {
    if self.is_ipc_mode() {
      // Modo IPC: enviar solicitud
      if let Some(ipc_client) = &self.ipc_client {
        let client_ref = ipc_client.borrow();
        if let Some(client) = client_ref.as_ref() {
          let _ = client.send_request_async(ipc::IpcRequest::SetWindowTitle {
            window_id: self.id,
            title,
          });
        }
      }
    } else if let Some(window) = &self.window {
      window.set_title(&title);
    }
  }

  #[napi(getter)]
  /// Sets the window title.
  pub fn get_title(&self) -> String {
    if self.is_ipc_mode() {
      // En modo IPC, retornamos un valor por defecto
      "WebviewJS".to_string()
    } else if let Some(window) = &self.window {
      window.title()
    } else {
      "Unknown".to_string()
    }
  }

  #[napi]
  /// Sets closable.
  pub fn set_closable(&self, closable: bool) {
    if !self.is_ipc_mode() {
      if let Some(window) = &self.window {
        window.set_closable(closable);
      }
    }
  }

  #[napi]
  /// Sets maximizable.
  pub fn set_maximizable(&self, maximizable: bool) {
    if !self.is_ipc_mode() {
      if let Some(window) = &self.window {
        window.set_maximizable(maximizable);
      }
    }
  }

  #[napi]
  /// Sets minimizable.
  pub fn set_minimizable(&self, minimizable: bool) {
    if !self.is_ipc_mode() {
      if let Some(window) = &self.window {
        window.set_minimizable(minimizable);
      }
    }
  }

  #[napi]
  /// Sets resizable.
  pub fn set_resizable(&self, resizable: bool) {
    if !self.is_ipc_mode() {
      if let Some(window) = &self.window {
        window.set_resizable(resizable);
      }
    }
  }

  #[napi(getter)]
  /// Gets the window theme.
  pub fn get_theme(&self) -> Theme {
    if self.is_ipc_mode() {
      // En modo IPC, retornamos System por defecto
      Theme::System
    } else if let Some(window) = &self.window {
      match window.theme() {
        tao::window::Theme::Light => Theme::Light,
        tao::window::Theme::Dark => Theme::Dark,
        _ => Theme::System,
      }
    } else {
      Theme::System
    }
  }

  #[napi]
  /// Sets the window theme.
  pub fn set_theme(&self, theme: Theme) {
    if !self.is_ipc_mode() {
      if let Some(window) = &self.window {
        let theme = match theme {
          Theme::Light => Some(tao::window::Theme::Light),
          Theme::Dark => Some(tao::window::Theme::Dark),
          _ => None,
        };
        window.set_theme(theme);
      }
    }
  }

  #[napi]
  /// Sets the window icon.
  pub fn set_window_icon(
    &self,
    icon: Either<Vec<u8>, String>,
    width: u32,
    height: u32,
  ) -> Result<()> {
    if !self.is_ipc_mode() {
      if let Some(window) = &self.window {
        #[cfg(target_os = "windows")]
        {
          use tao::platform::windows::IconExtWindows;

          let ico = match icon {
            Either::A(bytes) => Icon::from_rgba(bytes, width, height),
            Either::B(path) => Icon::from_path(&path, PhysicalSize::new(width, height).into()),
          };

          let parsed = ico.map_err(|e| {
            napi::Error::new(
              napi::Status::GenericFailure,
              format!("Failed to set window icon: {}", e),
            )
          })?;

          window.set_window_icon(Some(parsed));
        }
      }
    }

    Ok(())
  }

  #[napi]
  /// Removes the window icon.
  pub fn remove_window_icon(&self) {
    if !self.is_ipc_mode() {
      if let Some(window) = &self.window {
        window.set_window_icon(None);
      }
    }
  }

  #[napi]
  /// Modifies the window's visibility.
  /// If `false`, this will hide all the window. If `true`, this will show the window.
  pub fn set_visible(&self, visible: bool) {
    if self.is_ipc_mode() {
      // Modo IPC: enviar solicitud
      if let Some(ipc_client) = &self.ipc_client {
        let client_ref = ipc_client.borrow();
        if let Some(client) = client_ref.as_ref() {
          let _ = client.send_request_async(ipc::IpcRequest::SetWindowVisible {
            window_id: self.id,
            visible,
          });
        }
      }
    } else if let Some(window) = &self.window {
      window.set_visible(visible);
    }
  }

  #[napi]
  /// Modifies the window's progress bar.
  pub fn set_progress_bar(&self, state: JsProgressBar) {
    if !self.is_ipc_mode() {
      if let Some(window) = &self.window {
        let progress_state = match state.state {
          Some(JsProgressBarState::Normal) => Some(tao::window::ProgressState::Normal),
          Some(JsProgressBarState::Indeterminate) => {
            Some(tao::window::ProgressState::Indeterminate)
          }
          Some(JsProgressBarState::Paused) => Some(tao::window::ProgressState::Paused),
          Some(JsProgressBarState::Error) => Some(tao::window::ProgressState::Error),
          _ => None,
        };

        let progress_value = state.progress.map(|value| value as u64);

        let progress = ProgressBarState {
          progress: progress_value,
          state: progress_state,
          desktop_filename: None,
        };

        window.set_progress_bar(progress);
      }
    }
  }

  #[napi]
  /// Maximizes the window.
  pub fn set_maximized(&self, value: bool) {
    if !self.is_ipc_mode() {
      if let Some(window) = &self.window {
        window.set_maximized(value);
      }
    }
  }

  #[napi]
  /// Minimizes the window.
  pub fn set_minimized(&self, value: bool) {
    if !self.is_ipc_mode() {
      if let Some(window) = &self.window {
        window.set_minimized(value);
      }
    }
  }

  #[napi]
  /// Bring the window to front and focus.
  pub fn focus(&self) {
    if !self.is_ipc_mode() {
      if let Some(window) = &self.window {
        window.set_focus();
      }
    }
  }

  #[napi]
  /// Get available monitors.
  pub fn get_available_monitors(&self) -> Vec<Monitor> {
    if self.is_ipc_mode() {
      vec![]
    } else if let Some(window) = &self.window {
      window
        .available_monitors()
        .map(|m| Monitor {
          name: m.name(),
          scale_factor: m.scale_factor(),
          size: Dimensions {
            width: m.size().width,
            height: m.size().height,
          },
          position: Position {
            x: m.position().x,
            y: m.position().y,
          },
          video_modes: m
            .video_modes()
            .map(|v| JsVideoMode {
              size: Dimensions {
                width: v.size().width,
                height: v.size().height,
              },
              bit_depth: v.bit_depth(),
              refresh_rate: v.refresh_rate(),
            })
            .collect(),
        })
        .collect()
    } else {
      vec![]
    }
  }

  #[napi]
  /// Get the current monitor.
  pub fn get_current_monitor(&self) -> Option<Monitor> {
    if self.is_ipc_mode() {
      None
    } else if let Some(window) = &self.window {
      window.current_monitor().map(|monitor| Monitor {
        name: monitor.name(),
        scale_factor: monitor.scale_factor(),
        size: Dimensions {
          width: monitor.size().width,
          height: monitor.size().height,
        },
        position: Position {
          x: monitor.position().x,
          y: monitor.position().y,
        },
        video_modes: monitor
          .video_modes()
          .map(|v| JsVideoMode {
            size: Dimensions {
              width: v.size().width,
              height: v.size().height,
            },
            bit_depth: v.bit_depth(),
            refresh_rate: v.refresh_rate(),
          })
          .collect(),
      })
    } else {
      None
    }
  }

  #[napi]
  /// Get the primary monitor.
  pub fn get_primary_monitor(&self) -> Option<Monitor> {
    if self.is_ipc_mode() {
      None
    } else if let Some(window) = &self.window {
      window.primary_monitor().map(|monitor| Monitor {
        name: monitor.name(),
        scale_factor: monitor.scale_factor(),
        size: Dimensions {
          width: monitor.size().width,
          height: monitor.size().height,
        },
        position: Position {
          x: monitor.position().x,
          y: monitor.position().y,
        },
        video_modes: monitor
          .video_modes()
          .map(|v| JsVideoMode {
            size: Dimensions {
              width: v.size().width,
              height: v.size().height,
            },
            bit_depth: v.bit_depth(),
            refresh_rate: v.refresh_rate(),
          })
          .collect(),
      })
    } else {
      None
    }
  }

  #[napi]
  /// Get the monitor from the given point.
  pub fn get_monitor_from_point(&self, x: f64, y: f64) -> Option<Monitor> {
    if self.is_ipc_mode() {
      None
    } else if let Some(window) = &self.window {
      window.monitor_from_point(x, y).map(|monitor| Monitor {
        name: monitor.name(),
        scale_factor: monitor.scale_factor(),
        size: Dimensions {
          width: monitor.size().width,
          height: monitor.size().height,
        },
        position: Position {
          x: monitor.position().x,
          y: monitor.position().y,
        },
        video_modes: monitor
          .video_modes()
          .map(|v| JsVideoMode {
            size: Dimensions {
              width: v.size().width,
              height: v.size().height,
            },
            bit_depth: v.bit_depth(),
            refresh_rate: v.refresh_rate(),
          })
          .collect(),
      })
    } else {
      None
    }
  }

  #[napi]
  /// Prevents the window contents from being captured by other apps.
  pub fn set_content_protection(&self, enabled: bool) {
    if !self.is_ipc_mode() {
      if let Some(window) = &self.window {
        window.set_content_protection(enabled);
      }
    }
  }

  #[napi]
  /// Sets the window always on top.
  pub fn set_always_on_top(&self, enabled: bool) {
    if !self.is_ipc_mode() {
      if let Some(window) = &self.window {
        window.set_always_on_top(enabled);
      }
    }
  }

  #[napi]
  /// Sets always on bottom.
  pub fn set_always_on_bottom(&self, enabled: bool) {
    if !self.is_ipc_mode() {
      if let Some(window) = &self.window {
        window.set_always_on_bottom(enabled);
      }
    }
  }

  #[napi]
  /// Turn window decorations on or off.
  pub fn set_decorations(&self, enabled: bool) {
    if !self.is_ipc_mode() {
      if let Some(window) = &self.window {
        window.set_decorations(enabled);
      }
    }
  }

  #[napi(getter)]
  /// Gets the window's current fullscreen state.
  pub fn get_fullscreen(&self) -> Option<FullscreenType> {
    if self.is_ipc_mode() {
      None
    } else if let Some(window) = &self.window {
      match window.fullscreen() {
        None => None,
        Some(Fullscreen::Borderless(None)) => Some(FullscreenType::Borderless),
        _ => Some(FullscreenType::Exclusive),
      }
    } else {
      None
    }
  }

  #[napi]
  /// Sets the window to fullscreen or back.
  pub fn set_fullscreen(&self, fullscreen_type: Option<FullscreenType>) {
    if !self.is_ipc_mode() {
      if let Some(window) = &self.window {
        let monitor = window.current_monitor();

        if monitor.is_none() {
          return;
        };

        let video_mode = monitor.unwrap().video_modes().next();

        if video_mode.is_none() {
          return;
        };

        let fs = match fullscreen_type {
          Some(FullscreenType::Exclusive) => Some(Fullscreen::Exclusive(video_mode.unwrap())),
          Some(FullscreenType::Borderless) => Some(Fullscreen::Borderless(None)),
          _ => None,
        };

        window.set_fullscreen(fs);
      }
    }
  }

  #[napi]
  /// Closes the window by hiding it. Note: This hides the window rather than closing it completely,
  /// as tao requires the event loop to handle window closing. Use this when you want to
  /// close a specific window (like a login window) and potentially reopen it later.
  pub fn close(&self) {
    if self.is_ipc_mode() {
      // Modo IPC: enviar solicitud
      if let Some(ipc_client) = &self.ipc_client {
        let client_ref = ipc_client.borrow();
        if let Some(client) = client_ref.as_ref() {
          let _ = client.send_request_async(ipc::IpcRequest::SetWindowVisible {
            window_id: self.id,
            visible: false,
          });
        }
      }
    } else if let Some(window) = &self.window {
      window.set_visible(false);
    }
  }

  #[napi]
  /// Hides the window without destroying it.
  pub fn hide(&self) {
    if self.is_ipc_mode() {
      // Modo IPC: enviar solicitud
      if let Some(ipc_client) = &self.ipc_client {
        let client_ref = ipc_client.borrow();
        if let Some(client) = client_ref.as_ref() {
          let _ = client.send_request_async(ipc::IpcRequest::SetWindowVisible {
            window_id: self.id,
            visible: false,
          });
        }
      }
    } else if let Some(window) = &self.window {
      window.set_visible(false);
    }
  }

  #[napi]
  /// Shows the window if it was hidden.
  pub fn show(&self) {
    if self.is_ipc_mode() {
      // Modo IPC: enviar solicitud
      if let Some(ipc_client) = &self.ipc_client {
        let client_ref = ipc_client.borrow();
        if let Some(client) = client_ref.as_ref() {
          let _ = client.send_request_async(ipc::IpcRequest::SetWindowVisible {
            window_id: self.id,
            visible: true,
          });
        }
      }
    } else if let Some(window) = &self.window {
      window.set_visible(true);
    }
  }
}
