use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use napi::{
  bindgen_prelude::FunctionRef,
  threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
  Env, Result,
};
use napi_derive::*;
use tao::dpi::{LogicalPosition, LogicalSize};
use wry::{http::Request, Rect, WebViewBuilder};

use crate::{ipc, HeaderData, IpcMessage};

/// Represents the theme of the window.
#[napi(js_name = "Theme")]
#[derive(serde_derive::Serialize)]
pub enum JsTheme {
  /// The light theme.
  Light,
  /// The dark theme.
  Dark,
  /// The system theme.
  System,
}

// Export Theme as well for use in other modules
pub use JsTheme as Theme;

#[napi(object)]
#[derive(serde_derive::Serialize)]
pub struct WebviewOptions {
  /// The URL to load.
  pub url: Option<String>,
  /// The HTML content to load.
  pub html: Option<String>,
  /// The width of the window.
  pub width: Option<f64>,
  /// The height of the window.
  pub height: Option<f64>,
  /// The x position of the window.
  pub x: Option<f64>,
  /// The y position of the window.
  pub y: Option<f64>,
  /// Whether to enable devtools. Default is `true`.
  pub enable_devtools: Option<bool>,
  /// Whether the window is incognito. Default is `false`.
  pub incognito: Option<bool>,
  /// The default user agent.
  pub user_agent: Option<String>,
  /// Whether the webview should be built as a child.
  pub child: Option<bool>,
  /// The preload script to inject.
  pub preload: Option<String>,
  /// Whether the window is transparent. Default is `false`.
  pub transparent: Option<bool>,
  /// The default theme.
  pub theme: Option<JsTheme>,
  /// Whether the window is zoomable via hotkeys or gestures.
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

#[napi(js_name = "Webview")]
pub struct JsWebview {
  /// The inner webview.
  webview_inner: Option<wry::WebView>,
  /// The ipc handler fn
  ipc_state: Rc<RefCell<Option<FunctionRef<IpcMessage, ()>>>>,
  /// Window ID for IPC mode
  window_id: u32,
  /// IPC client for communicating with eventloop process (only in IPC mode)
  ipc_client: Option<Rc<RefCell<Option<ipc::IpcClient>>>>,
}

#[napi]
impl JsWebview {
  pub fn create(env: &Env, window: &tao::window::Window, options: WebviewOptions) -> Result<Self> {
    // let mut webview = if options.child.unwrap_or(false) {
    //   WebViewBuilder::new_as_child(window)
    // } else {
    //   WebViewBuilder::new(window)
    // };
    let mut webview = WebViewBuilder::new();

    if let Some(devtools) = options.enable_devtools {
      webview = webview.with_devtools(devtools);
    }

    webview = webview.with_bounds(Rect {
      position: LogicalPosition::new(options.x.unwrap_or(0.0), options.y.unwrap_or(0.0)).into(),
      size: LogicalSize::new(
        options.width.unwrap_or(800.0),
        options.height.unwrap_or(600.0),
      )
      .into(),
    });

    if let Some(incognito) = options.incognito {
      webview = webview.with_incognito(incognito);
    }

    if let Some(preload) = options.preload {
      webview = webview.with_initialization_script(&preload);
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

    #[cfg(target_os = "windows")]
    {
      use wry::WebViewBuilderExtWindows;

      if let Some(theme) = options.theme {
        let theme = match theme {
          JsTheme::Light => wry::Theme::Light,
          JsTheme::Dark => wry::Theme::Dark,
          _ => wry::Theme::Auto,
        };

        webview = webview.with_theme(theme)
      }
    }

    if let Some(user_agent) = options.user_agent {
      webview = webview.with_user_agent(&user_agent);
    }

    if let Some(html) = options.html {
      webview = webview.with_html(&html);
    }

    if let Some(url) = options.url {
      webview = webview.with_url(&url);
    }

    let ipc_state = Rc::new(RefCell::new(None::<FunctionRef<IpcMessage, ()>>));
    let ipc_state_clone = ipc_state.clone();

    let env = env.clone();
    let ipc_handler = move |req: Request<String>| {
      let callback: &RefCell<Option<FunctionRef<IpcMessage, ()>>> = ipc_state_clone.borrow();
      let callback = callback.borrow();
      if let Some(func) = callback.as_ref() {
        let on_ipc_msg = func.borrow_back(&env);

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

        match on_ipc_msg.call(ipc_message) {
          _ => {}
        };
      }
    };

    webview = webview.with_ipc_handler(ipc_handler);

    let handle_build_error = |e| {
      napi::Error::new(
        napi::Status::GenericFailure,
        format!("Failed to create webview: {}", e),
      )
    };

    #[cfg(not(target_os = "linux"))]
    let webview = {
      if options.child.unwrap_or(false) {
        webview.build_as_child(&window).map_err(handle_build_error)
      } else {
        webview.build(&window).map_err(handle_build_error)
      }
    }?;

    #[cfg(target_os = "linux")]
    let webview = {
      if options.child.unwrap_or(false) {
        webview
          .build_as_child(window.gtk_window())
          .map_err(handle_build_error)
      } else {
        webview
          .build(window.gtk_window())
          .map_err(handle_build_error)
      }
    };

    Ok(Self {
      webview_inner: Some(webview),
      ipc_state,
      window_id: 0,
      ipc_client: None,
    })
  }

  /// Crea un JsWebview proxy que se comunica vía IPC con el proceso del eventloop
  pub fn new_ipc_proxy(window_id: u32, ipc_client: Rc<RefCell<Option<ipc::IpcClient>>>) -> Self {
    Self {
      webview_inner: None,
      ipc_state: Rc::new(RefCell::new(None::<FunctionRef<IpcMessage, ()>>)),
      window_id,
      ipc_client: Some(ipc_client),
    }
  }

  /// Verifica si este webview está en modo IPC
  fn is_ipc_mode(&self) -> bool {
    self.ipc_client.is_some()
  }

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
    if self.is_ipc_mode() {
      // En modo IPC, no soportamos print por ahora
      Ok(())
    } else if let Some(webview) = &self.webview_inner {
      webview.print().map_err(|e| {
        napi::Error::new(
          napi::Status::GenericFailure,
          format!("Failed to print: {}", e),
        )
      })
    } else {
      Err(napi::Error::new(
        napi::Status::GenericFailure,
        "Webview not initialized",
      ))
    }
  }

  #[napi]
  /// Set webview zoom level.
  pub fn zoom(&self, scale_factor: f64) -> Result<()> {
    if self.is_ipc_mode() {
      // En modo IPC, no soportamos zoom por ahora
      Ok(())
    } else if let Some(webview) = &self.webview_inner {
      webview.zoom(scale_factor).map_err(|e| {
        napi::Error::new(
          napi::Status::GenericFailure,
          format!("Failed to zoom: {}", e),
        )
      })
    } else {
      Err(napi::Error::new(
        napi::Status::GenericFailure,
        "Webview not initialized",
      ))
    }
  }

  #[napi]
  /// Hides or shows the webview.
  pub fn set_webview_visibility(&self, visible: bool) -> Result<()> {
    if self.is_ipc_mode() {
      // En modo IPC, no soportamos esto por ahora
      Ok(())
    } else if let Some(webview) = &self.webview_inner {
      webview.set_visible(visible).map_err(|e| {
        napi::Error::new(
          napi::Status::GenericFailure,
          format!("Failed to set webview visibility: {}", e),
        )
      })
    } else {
      Err(napi::Error::new(
        napi::Status::GenericFailure,
        "Webview not initialized",
      ))
    }
  }

  #[napi]
  /// Whether devtools is opened.
  pub fn is_devtools_open(&self) -> bool {
    if self.is_ipc_mode() {
      false
    } else if let Some(webview) = &self.webview_inner {
      webview.is_devtools_open()
    } else {
      false
    }
  }

  #[napi]
  /// Opens devtools.
  pub fn open_devtools(&self) {
    if !self.is_ipc_mode() {
      if let Some(webview) = &self.webview_inner {
        webview.open_devtools();
      }
    }
  }

  #[napi]
  /// Closes devtools.
  pub fn close_devtools(&self) {
    if !self.is_ipc_mode() {
      if let Some(webview) = &self.webview_inner {
        webview.close_devtools();
      }
    }
  }

  #[napi]
  /// Loads the given URL.
  pub fn load_url(&self, url: String) -> Result<()> {
    if self.is_ipc_mode() {
      // Modo IPC: enviar solicitud
      if let Some(ipc_client) = &self.ipc_client {
        let borrowed: std::cell::Ref<'_, Option<ipc::IpcClient>> = (**ipc_client).borrow();
        if let Some(client) = borrowed.as_ref() {
          client
            .send_request(ipc::IpcRequest::LoadUrl {
              window_id: self.window_id,
              url,
            })
            .map_err(|e| {
              napi::Error::new(
                napi::Status::GenericFailure,
                format!("Failed to send IPC request: {}", e),
              )
            })?;
          Ok(())
        } else {
          Err(napi::Error::new(
            napi::Status::GenericFailure,
            "IPC client not initialized",
          ))
        }
      } else {
        Err(napi::Error::new(
          napi::Status::GenericFailure,
          "IPC client not initialized",
        ))
      }
    } else if let Some(webview) = &self.webview_inner {
      webview.load_url(&url).map_err(|e| {
        napi::Error::new(
          napi::Status::GenericFailure,
          format!("Failed to load URL: {}", e),
        )
      })
    } else {
      Err(napi::Error::new(
        napi::Status::GenericFailure,
        "Webview not initialized",
      ))
    }
  }

  #[napi]
  /// Loads the given HTML content.
  pub fn load_html(&self, html: String) -> Result<()> {
    if self.is_ipc_mode() {
      // Modo IPC: enviar solicitud
      if let Some(ipc_client) = &self.ipc_client {
        let borrowed: std::cell::Ref<'_, Option<ipc::IpcClient>> = (**ipc_client).borrow();
        if let Some(client) = borrowed.as_ref() {
          client
            .send_request(ipc::IpcRequest::LoadHtml {
              window_id: self.window_id,
              html,
            })
            .map_err(|e| {
              napi::Error::new(
                napi::Status::GenericFailure,
                format!("Failed to send IPC request: {}", e),
              )
            })?;
          Ok(())
        } else {
          Err(napi::Error::new(
            napi::Status::GenericFailure,
            "IPC client not initialized",
          ))
        }
      } else {
        Err(napi::Error::new(
          napi::Status::GenericFailure,
          "IPC client not initialized",
        ))
      }
    } else if let Some(webview) = &self.webview_inner {
      webview.load_html(&html).map_err(|e| {
        napi::Error::new(
          napi::Status::GenericFailure,
          format!("Failed to load HTML: {}", e),
        )
      })
    } else {
      Err(napi::Error::new(
        napi::Status::GenericFailure,
        "Webview not initialized",
      ))
    }
  }

  #[napi]
  /// Evaluates the given JavaScript code.
  pub fn evaluate_script(&self, js: String) -> Result<()> {
    if self.is_ipc_mode() {
      // Modo IPC: enviar solicitud
      if let Some(ipc_client) = &self.ipc_client {
        let borrowed: std::cell::Ref<'_, Option<ipc::IpcClient>> = (**ipc_client).borrow();
        if let Some(client) = borrowed.as_ref() {
          client
            .send_request(ipc::IpcRequest::EvaluateScript {
              window_id: self.window_id,
              script: js,
            })
            .map_err(|e| {
              napi::Error::new(
                napi::Status::GenericFailure,
                format!("Failed to send IPC request: {}", e),
              )
            })?;
          Ok(())
        } else {
          Err(napi::Error::new(
            napi::Status::GenericFailure,
            "IPC client not initialized",
          ))
        }
      } else {
        Err(napi::Error::new(
          napi::Status::GenericFailure,
          "IPC client not initialized",
        ))
      }
    } else if let Some(webview) = &self.webview_inner {
      webview
        .evaluate_script(&js)
        .map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("{}", e)))
    } else {
      Err(napi::Error::new(
        napi::Status::GenericFailure,
        "Webview not initialized",
      ))
    }
  }

  #[napi]
  pub fn evaluate_script_with_callback(
    &self,
    js: String,
    callback: ThreadsafeFunction<String>,
  ) -> Result<()> {
    if self.is_ipc_mode() {
      // En modo IPC, no soportamos callbacks por ahora
      Err(napi::Error::new(
        napi::Status::GenericFailure,
        "evaluate_script_with_callback not supported in IPC mode",
      ))
    } else if let Some(webview) = &self.webview_inner {
      webview
        .evaluate_script_with_callback(&js, move |val| {
          callback.call(Ok(val), ThreadsafeFunctionCallMode::Blocking);
        })
        .map_err(|e| napi::Error::new(napi::Status::GenericFailure, format!("{}", e)))
    } else {
      Err(napi::Error::new(
        napi::Status::GenericFailure,
        "Webview not initialized",
      ))
    }
  }

  #[napi]
  /// Reloads the webview.
  pub fn reload(&self) -> Result<()> {
    if self.is_ipc_mode() {
      // En modo IPC, no soportamos reload por ahora
      Ok(())
    } else if let Some(webview) = &self.webview_inner {
      webview.reload().map_err(|e| {
        napi::Error::new(
          napi::Status::GenericFailure,
          format!("Failed to reload: {}", e),
        )
      })
    } else {
      Err(napi::Error::new(
        napi::Status::GenericFailure,
        "Webview not initialized",
      ))
    }
  }
}
