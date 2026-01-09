use std::{cell::RefCell, rc::Rc};

use napi::{
  bindgen_prelude::FunctionRef,
  threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
  Env, Result,
};
use napi_derive::*;
use tao::dpi::{LogicalPosition, LogicalSize};
use wry::{http::Request, Rect, WebViewBuilder};

use crate::types::{HeaderData, IpcMessage, WebviewOptions};
use crate::utils::{error_with_context, theme_to_wry};
use wry::WebViewBuilderExtWindows;

/// Applies webview options to a WebViewBuilder
fn apply_webview_options<'a>(
  mut webview: WebViewBuilder<'a>,
  options: &'a WebviewOptions,
) -> WebViewBuilder<'a> {
  // Bounds
  webview = webview.with_bounds(Rect {
    position: LogicalPosition::new(options.x.unwrap_or(0.0), options.y.unwrap_or(0.0)).into(),
    size: LogicalSize::new(
      options.width.unwrap_or(800.0),
      options.height.unwrap_or(600.0),
    )
    .into(),
  });

  // Content
  if let Some(devtools) = options.enable_devtools {
    webview = webview.with_devtools(devtools);
  }

  if let Some(incognito) = options.incognito {
    webview = webview.with_incognito(incognito);
  }

  if let Some(transparent) = options.transparent {
    webview = webview.with_transparent(transparent);
  }

  if let Some(autoplay) = options.autoplay {
    webview = webview.with_autoplay(autoplay);
  }

  if let Some(clipboard) = options.clipboard {
    webview = webview.with_clipboard(clipboard);
  }

  if let Some(back_forward_navigation_gestures) = options.back_forward_navigation_gestures {
    webview = webview.with_back_forward_navigation_gestures(back_forward_navigation_gestures);
  }

  if let Some(hotkeys_zoom) = options.hotkeys_zoom {
    webview = webview.with_hotkeys_zoom(hotkeys_zoom);
  }

  // Windows-specific options
  #[cfg(target_os = "windows")]
  {
    if let Some(ref theme) = options.theme {
      webview = webview.with_theme(theme_to_wry(*theme));
    }
  }

  // User agent
  if let Some(ref user_agent) = options.user_agent {
    webview = webview.with_user_agent(user_agent);
  }

  // Content loading
  if let Some(ref html) = options.html {
    webview = webview.with_html(html);
  }

  if let Some(ref url) = options.url {
    webview = webview.with_url(url);
  }

  webview
}

#[napi(js_name = "Webview")]
pub struct JsWebview {
  /// The inner webview
  webview_inner: wry::WebView,
  /// The ipc handler fn
  ipc_state: Rc<RefCell<Option<FunctionRef<IpcMessage, ()>>>>,
}

#[napi]
impl JsWebview {
  pub fn create(env: &Env, window: &tao::window::Window, options: WebviewOptions) -> Result<Self> {
    let mut webview = apply_webview_options(WebViewBuilder::new(), &options);

    // Preload script
    if let Some(ref preload) = options.preload {
      webview = webview.with_initialization_script(preload);
    }

    let ipc_state = Rc::new(RefCell::new(None::<FunctionRef<IpcMessage, ()>>));
    let ipc_state_clone = ipc_state.clone();
    let env_copy = *env;

    let ipc_handler = move |req: Request<String>| {
      let borrowed = ipc_state_clone.borrow();
      if let Some(func) = borrowed.as_ref() {
        let on_ipc_msg = func.borrow_back(&env_copy);

        if on_ipc_msg.is_err() {
          return;
        }

        let on_ipc_msg = on_ipc_msg.unwrap();

        let body = req.body().as_bytes().to_vec().into();
        let headers = req
          .headers()
          .iter()
          .map(|(k, v)| HeaderData {
            key: k.as_str().to_string(),
            value: v.to_str().ok().map(|s| s.to_string()),
          })
          .collect::<Vec<_>>();

        let ipc_message = IpcMessage {
          body,
          headers,
          method: req.method().to_string(),
          uri: req.uri().to_string(),
        };

        let _ = on_ipc_msg.call(ipc_message);
      }
    };

    webview = webview.with_ipc_handler(ipc_handler);

    let webview_inner = build_webview(webview, window, options.child.unwrap_or(false))?;

    Ok(Self {
      webview_inner,
      ipc_state,
    })
  }
}

/// Builds the webview either as a child or as a regular webview
fn build_webview(
  webview: WebViewBuilder,
  window: &tao::window::Window,
  as_child: bool,
) -> Result<wry::WebView> {
  if as_child {
    webview
      .build_as_child(window)
      .map_err(|e| error_with_context("create child webview", &e.to_string()))
  } else {
    webview
      .build(window)
      .map_err(|e| error_with_context("create webview", &e.to_string()))
  }
}

#[napi]
impl JsWebview {
  #[napi(constructor)]
  pub fn new() -> Result<Self> {
    Err(napi::Error::new(
      napi::Status::GenericFailure,
      "Webview constructor is not directly supported",
    ))
  }

  #[napi]
  /// Sets the IPC handler callback.
  pub fn on_ipc_message(&mut self, handler: Option<FunctionRef<IpcMessage, ()>>) {
    *self.ipc_state.borrow_mut() = handler;
  }

  #[napi]
  /// Launch a print modal for this window's contents.
  pub fn print(&self) -> Result<()> {
    self
      .webview_inner
      .print()
      .map_err(|e| error_with_context("print", &e.to_string()))
  }

  #[napi]
  /// Set webview zoom level.
  pub fn zoom(&self, scale_factor: f64) -> Result<()> {
    self
      .webview_inner
      .zoom(scale_factor)
      .map_err(|e| error_with_context("zoom", &e.to_string()))
  }

  #[napi]
  /// Hides or shows the webview.
  pub fn set_webview_visibility(&self, visible: bool) -> Result<()> {
    self
      .webview_inner
      .set_visible(visible)
      .map_err(|e| error_with_context("set webview visibility", &e.to_string()))
  }

  #[napi]
  /// Whether the devtools is opened.
  pub fn is_devtools_open(&self) -> bool {
    self.webview_inner.is_devtools_open()
  }

  #[napi]
  /// Opens the devtools.
  pub fn open_devtools(&self) {
    self.webview_inner.open_devtools();
  }

  #[napi]
  /// Closes the devtools.
  pub fn close_devtools(&self) {
    self.webview_inner.close_devtools();
  }

  #[napi]
  /// Loads the given URL.
  pub fn load_url(&self, url: String) -> Result<()> {
    self
      .webview_inner
      .load_url(&url)
      .map_err(|e| error_with_context("load URL", &e.to_string()))
  }

  #[napi]
  /// Loads the given HTML content.
  pub fn load_html(&self, html: String) -> Result<()> {
    self
      .webview_inner
      .load_html(&html)
      .map_err(|e| error_with_context("load HTML", &e.to_string()))
  }

  #[napi]
  /// Evaluates the given JavaScript code.
  pub fn evaluate_script(&self, js: String) -> Result<()> {
    self
      .webview_inner
      .evaluate_script(&js)
      .map_err(|e| error_with_context("evaluate script", &e.to_string()))
  }

  #[napi]
  pub fn evaluate_script_with_callback(
    &self,
    js: String,
    callback: ThreadsafeFunction<String>,
  ) -> Result<()> {
    self
      .webview_inner
      .evaluate_script_with_callback(&js, move |val| {
        callback.call(Ok(val), ThreadsafeFunctionCallMode::Blocking);
      })
      .map_err(|e| error_with_context("evaluate script with callback", &e.to_string()))
  }

  #[napi]
  /// Reloads the webview.
  pub fn reload(&self) -> Result<()> {
    self
      .webview_inner
      .reload()
      .map_err(|e| error_with_context("reload", &e.to_string()))
  }
}
