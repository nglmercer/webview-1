use napi::{Either, Env, Result};
use napi_derive::*;
use tao::{
  dpi::{LogicalPosition, PhysicalSize},
  event_loop::EventLoop,
  window::{Fullscreen, ProgressBarState, Window, WindowBuilder},
};

use crate::types::{
  BrowserWindowOptions, FullscreenType, JsProgressBar, JsProgressBarState, Theme, WebviewOptions,
};
use crate::utils::{convert_monitor, error_with_context, tao_to_theme, theme_to_tao};
use crate::webview::JsWebview;

#[napi]
pub struct BrowserWindow {
  is_child_window: bool,
  window: Window,
}

/// Applies browser window options to a WindowBuilder
fn apply_window_options(
  mut window: WindowBuilder,
  options: &BrowserWindowOptions,
) -> WindowBuilder {
  // Size
  if let Some(width) = options.width {
    window = window.with_inner_size(PhysicalSize::new(width, options.height.unwrap()));
  }

  // Position
  if let Some(x) = options.x {
    window = window.with_position(LogicalPosition::new(x, options.y.unwrap()));
  }

  // Window properties
  if let Some(resizable) = options.resizable {
    window = window.with_resizable(resizable);
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

  // Fullscreen
  if let Some(ref fullscreen) = options.fullscreen {
    let fs = match fullscreen {
      FullscreenType::Borderless => Some(Fullscreen::Borderless(None)),
      _ => None,
    };
    window = window.with_fullscreen(fs);
  }

  // Title
  if let Some(ref title) = options.title {
    window = window.with_title(title);
  }

  window
}

#[napi]
impl BrowserWindow {
  pub fn new(
    event_loop: &EventLoop<()>,
    options: Option<BrowserWindowOptions>,
    child: bool,
  ) -> Result<Self> {
    let options = options.unwrap_or_default();
    let window = apply_window_options(WindowBuilder::new(), &options);

    let window = window
      .build(event_loop)
      .map_err(|e| error_with_context("create window", &e.to_string()))?;

    Ok(Self {
      window,
      is_child_window: child,
    })
  }

  #[napi]
  /// Creates a webview on this window.
  pub fn create_webview(&mut self, env: Env, options: Option<WebviewOptions>) -> Result<JsWebview> {
    let webview = JsWebview::create(&env, &self.window, options.unwrap_or_default())?;
    Ok(webview)
  }

  #[napi(getter)]
  /// Whether or not the window is a child window.
  pub fn is_child(&self) -> bool {
    self.is_child_window
  }

  #[napi]
  /// Whether the window is focused.
  pub fn is_focused(&self) -> bool {
    self.window.is_focused()
  }

  #[napi]
  /// Whether the window is visible.
  pub fn is_visible(&self) -> bool {
    self.window.is_visible()
  }

  #[napi]
  /// Whether the window is decorated.
  pub fn is_decorated(&self) -> bool {
    self.window.is_decorated()
  }

  #[napi]
  /// Whether the window is closable.
  pub fn is_closable(&self) -> bool {
    self.window.is_closable()
  }

  #[napi]
  /// Whether the window is maximizable.
  pub fn is_maximizable(&self) -> bool {
    self.window.is_maximizable()
  }

  #[napi]
  /// Whether the window is minimizable.
  pub fn is_minimizable(&self) -> bool {
    self.window.is_minimizable()
  }

  #[napi]
  /// Whether the window is maximized.
  pub fn is_maximized(&self) -> bool {
    self.window.is_maximized()
  }

  #[napi]
  /// Whether the window is minimized.
  pub fn is_minimized(&self) -> bool {
    self.window.is_minimized()
  }

  #[napi]
  /// Whether the window is resizable.
  pub fn is_resizable(&self) -> bool {
    self.window.is_resizable()
  }

  #[napi]
  /// Sets the window title.
  pub fn set_title(&self, title: String) {
    self.window.set_title(&title);
  }

  #[napi(getter)]
  /// Sets the window title.
  pub fn get_title(&self) -> String {
    self.window.title()
  }

  #[napi]
  /// Sets closable.
  pub fn set_closable(&self, closable: bool) {
    self.window.set_closable(closable);
  }

  #[napi]
  /// Sets maximizable.
  pub fn set_maximizable(&self, maximizable: bool) {
    self.window.set_maximizable(maximizable);
  }

  #[napi]
  /// Sets minimizable.
  pub fn set_minimizable(&self, minimizable: bool) {
    self.window.set_minimizable(minimizable);
  }

  #[napi]
  /// Sets resizable.
  pub fn set_resizable(&self, resizable: bool) {
    self.window.set_resizable(resizable);
  }

  #[napi(getter)]
  /// Gets the window theme.
  pub fn get_theme(&self) -> Theme {
    tao_to_theme(self.window.theme())
  }

  #[napi]
  /// Sets the window theme.
  pub fn set_theme(&self, theme: Theme) {
    self.window.set_theme(theme_to_tao(theme));
  }

  #[napi]
  /// Sets the window icon.
  #[allow(unused_variables)]
  pub fn set_window_icon(
    &self,
    icon: Either<Vec<u8>, String>,
    width: u32,
    height: u32,
  ) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
      use tao::platform::windows::IconExtWindows;
      use tao::window::Icon;

      let ico = match icon {
        Either::A(bytes) => Icon::from_rgba(bytes, width, height),
        Either::B(path) => Icon::from_path(&path, PhysicalSize::new(width, height).into()),
      };

      let parsed = ico.map_err(|e| error_with_context("set window icon", &e.to_string()))?;

      self.window.set_window_icon(Some(parsed));
    }

    Ok(())
  }

  #[napi]
  /// Removes the window icon.
  pub fn remove_window_icon(&self) {
    self.window.set_window_icon(None);
  }

  #[napi]
  /// Modifies the window's visibility.
  /// If `false`, this will hide all the window. If `true`, this will show the window.
  pub fn set_visible(&self, visible: bool) {
    self.window.set_visible(visible);
  }

  #[napi]
  /// Modifies the window's progress bar.
  pub fn set_progress_bar(&self, state: JsProgressBar) {
    let progress_state = match state.state {
      Some(JsProgressBarState::Normal) => Some(tao::window::ProgressState::Normal),
      Some(JsProgressBarState::Indeterminate) => Some(tao::window::ProgressState::Indeterminate),
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

    self.window.set_progress_bar(progress);
  }

  #[napi]
  /// Maximizes the window.
  pub fn set_maximized(&self, value: bool) {
    self.window.set_maximized(value);
  }

  #[napi]
  /// Minimizes the window.
  pub fn set_minimized(&self, value: bool) {
    self.window.set_minimized(value);
  }

  #[napi]
  /// Bring the window to front and focus.
  pub fn focus(&self) {
    self.window.set_focus();
  }

  #[napi]
  /// Get available monitors.
  pub fn get_available_monitors(&self) -> Vec<crate::types::Monitor> {
    self
      .window
      .available_monitors()
      .map(convert_monitor)
      .collect()
  }

  #[napi]
  /// Get the current monitor.
  pub fn get_current_monitor(&self) -> Option<crate::types::Monitor> {
    self.window.current_monitor().map(convert_monitor)
  }

  #[napi]
  /// Get the primary monitor.
  pub fn get_primary_monitor(&self) -> Option<crate::types::Monitor> {
    self.window.primary_monitor().map(convert_monitor)
  }

  #[napi]
  /// Get the monitor from the given point.
  pub fn get_monitor_from_point(&self, x: f64, y: f64) -> Option<crate::types::Monitor> {
    self.window.monitor_from_point(x, y).map(convert_monitor)
  }

  #[napi]
  /// Prevents the window contents from being captured by other apps.
  pub fn set_content_protection(&self, enabled: bool) {
    self.window.set_content_protection(enabled);
  }

  #[napi]
  /// Sets the window always on top.
  pub fn set_always_on_top(&self, enabled: bool) {
    self.window.set_always_on_top(enabled);
  }

  #[napi]
  /// Sets always on bottom.
  pub fn set_always_on_bottom(&self, enabled: bool) {
    self.window.set_always_on_bottom(enabled);
  }

  #[napi]
  /// Turn window decorations on or off.
  pub fn set_decorations(&self, enabled: bool) {
    self.window.set_decorations(enabled);
  }

  #[napi(getter)]
  /// Gets the window's current fullscreen state.
  pub fn get_fullscreen(&self) -> Option<FullscreenType> {
    match self.window.fullscreen() {
      None => None,
      Some(Fullscreen::Borderless(None)) => Some(FullscreenType::Borderless),
      _ => Some(FullscreenType::Exclusive),
    }
  }

  #[napi]
  /// Sets the window to fullscreen or back.
  pub fn set_fullscreen(&self, fullscreen_type: Option<FullscreenType>) {
    let monitor = self.window.current_monitor();

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

    self.window.set_fullscreen(fs);
  }

  #[napi]
  /// Closes the window by hiding it. Note: This hides the window rather than closing it completely,
  /// as tao requires the event loop to handle window closing. Use this when you want to
  /// close a specific window (like a login window) and potentially reopen it later.
  pub fn close(&self) {
    self.set_visible(false);
  }

  #[napi]
  /// Hides the window without destroying it.
  pub fn hide(&self) {
    self.set_visible(false);
  }

  #[napi]
  /// Shows the window if it was hidden.
  pub fn show(&self) {
    self.set_visible(true);
  }
}
