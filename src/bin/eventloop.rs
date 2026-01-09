//! Binario separado para ejecutar el eventloop en su propio proceso
//!
//! Este binario se ejecuta como un proceso independiente, lo que permite
//! que el EventLoop de tao se ejecute en el hilo principal de este proceso,
//! evitando las restricciones de Windows sobre crear EventLoops en hilos secundarios.

use std::collections::HashMap;
use std::env;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use tao::{
  dpi::{LogicalPosition, PhysicalSize},
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoopBuilder},
  platform::windows::EventLoopBuilderExtWindows,
  window::{Fullscreen, Window, WindowBuilder},
};
use wry::{Rect, WebViewBuilder};

/// Mensajes que se pueden enviar desde el proceso principal al proceso del eventloop
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum IpcRequest {
  /// Crear una nueva ventana del navegador
  CreateBrowserWindow {
    window_id: u32,
    options: serde_json::Value,
    is_child: bool,
  },
  /// Cerrar una ventana específica
  CloseWindow { window_id: u32 },
  /// Crear un webview en una ventana
  CreateWebview {
    window_id: u32,
    options: serde_json::Value,
  },
  /// Ejecutar JavaScript en un webview
  EvaluateScript { window_id: u32, script: String },
  /// Cargar una URL en un webview
  LoadUrl { window_id: u32, url: String },
  /// Cargar HTML en un webview
  LoadHtml { window_id: u32, html: String },
  /// Mostrar/Ocultar ventana
  SetWindowVisible { window_id: u32, visible: bool },
  /// Establecer título de ventana
  SetWindowTitle { window_id: u32, title: String },
  /// Solicitar salir de la aplicación
  Exit,
  /// Ping para verificar conexión
  Ping,
}

/// Mensajes que se envían desde el proceso del eventloop al proceso principal
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum IpcResponse {
  /// Respuesta de éxito
  Success {
    request_id: u64,
    data: Option<serde_json::Value>,
  },
  /// Respuesta de error
  Error { request_id: u64, message: String },
  /// Evento de la aplicación (cierre de ventana, etc.)
  ApplicationEvent {
    event_type: String,
    window_id: Option<u32>,
  },
  /// Respuesta a ping
  Pong,
}

/// Wrapper para mensajes con ID de solicitud
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct IpcMessage<T> {
  pub request_id: u64,
  pub payload: T,
}

/// Gestor de ventanas en el proceso del eventloop
struct WindowManager {
  /// Mapa de window_id -> Window
  windows: HashMap<u32, Window>,
  /// Mapa de tao window_id -> nuestro window_id
  tao_to_window_id: HashMap<tao::window::WindowId, u32>,
  /// Mapa de window_id -> WebView
  webviews: HashMap<u32, wry::WebView>,
}

impl WindowManager {
  fn new() -> Self {
    Self {
      windows: HashMap::new(),
      tao_to_window_id: HashMap::new(),
      webviews: HashMap::new(),
    }
  }

  fn add_window(&mut self, window_id: u32, window: Window) {
    let tao_id = window.id();
    self.windows.insert(window_id, window);
    self.tao_to_window_id.insert(tao_id, window_id);
  }

  fn create_window_options(
    window_id: u32,
    options: &serde_json::Value,
  ) -> Result<(WindowBuilder, u32), String> {
    // Deserializar opciones
    let resizable = options["resizable"].as_bool().unwrap_or(true);
    let title = options["title"].as_str().unwrap_or("WebviewJS");
    let width = options["width"].as_f64().unwrap_or(800.0);
    let height = options["height"].as_f64().unwrap_or(600.0);
    let x = options["x"].as_f64().unwrap_or(0.0);
    let y = options["y"].as_f64().unwrap_or(0.0);
    let visible = options["visible"].as_bool().unwrap_or(true);
    let decorations = options["decorations"].as_bool().unwrap_or(true);
    let transparent = options["transparent"].as_bool().unwrap_or(false);
    let always_on_top = options["always_on_top"].as_bool().unwrap_or(false);
    let always_on_bottom = options["always_on_bottom"].as_bool().unwrap_or(false);
    let maximized = options["maximized"].as_bool().unwrap_or(false);
    let maximizable = options["maximizable"].as_bool().unwrap_or(true);
    let minimizable = options["minimizable"].as_bool().unwrap_or(true);
    let focused = options["focused"].as_bool().unwrap_or(true);

    let mut window_builder = WindowBuilder::new();

    window_builder = window_builder.with_resizable(resizable);
    window_builder = window_builder.with_title(title);
    window_builder = window_builder.with_inner_size(PhysicalSize::new(width, height));
    window_builder = window_builder.with_position(LogicalPosition::new(x, y));
    window_builder = window_builder.with_visible(visible);
    window_builder = window_builder.with_decorations(decorations);
    window_builder = window_builder.with_transparent(transparent);
    window_builder = window_builder.with_always_on_top(always_on_top);
    window_builder = window_builder.with_always_on_bottom(always_on_bottom);
    window_builder = window_builder.with_maximized(maximized);
    window_builder = window_builder.with_maximizable(maximizable);
    window_builder = window_builder.with_minimizable(minimizable);
    window_builder = window_builder.with_focused(focused);

    #[cfg(target_os = "windows")]
    {
      use tao::platform::windows::WindowBuilderExtWindows;
      if transparent {
        window_builder = window_builder.with_undecorated_shadow(false);
      }
    }

    if let Some(fullscreen) = options["fullscreen"].as_str() {
      if fullscreen == "Borderless" {
        window_builder = window_builder.with_fullscreen(Some(Fullscreen::Borderless(None)));
      }
    }

    Ok((window_builder, window_id))
  }

  fn create_webview(&mut self, window_id: u32, options: &serde_json::Value) -> Result<(), String> {
    let window = self
      .windows
      .get(&window_id)
      .ok_or_else(|| format!("Window {} not found", window_id))?;

    // Deserializar opciones de webview
    let enable_devtools = options["enable_devtools"].as_bool().unwrap_or(true);
    let incognito = options["incognito"].as_bool().unwrap_or(false);
    let transparent = options["transparent"].as_bool().unwrap_or(false);
    let autoplay = options["autoplay"].as_bool().unwrap_or(true);
    let clipboard = options["clipboard"].as_bool().unwrap_or(true);
    let back_forward_navigation_gestures = options["back_forward_navigation_gestures"]
      .as_bool()
      .unwrap_or(true);
    let hotkeys_zoom = options["hotkeys_zoom"].as_bool().unwrap_or(true);
    let user_agent = options["user_agent"].as_str().unwrap_or("WebviewJS");
    let preload = options["preload"].as_str();
    let url = options["url"].as_str();
    let html = options["html"].as_str();
    let width = options["width"].as_f64().unwrap_or(800.0);
    let height = options["height"].as_f64().unwrap_or(600.0);
    let x = options["x"].as_f64().unwrap_or(0.0);
    let y = options["y"].as_f64().unwrap_or(0.0);

    let mut webview_builder = WebViewBuilder::new();

    webview_builder = webview_builder.with_devtools(enable_devtools);
    webview_builder = webview_builder.with_incognito(incognito);
    webview_builder = webview_builder.with_transparent(transparent);
    webview_builder = webview_builder.with_autoplay(autoplay);
    webview_builder = webview_builder.with_clipboard(clipboard);
    webview_builder =
      webview_builder.with_back_forward_navigation_gestures(back_forward_navigation_gestures);
    webview_builder = webview_builder.with_hotkeys_zoom(hotkeys_zoom);
    webview_builder = webview_builder.with_user_agent(user_agent);

    webview_builder = webview_builder.with_bounds(Rect {
      position: LogicalPosition::new(x, y).into(),
      size: tao::dpi::LogicalSize::new(width, height).into(),
    });

    #[cfg(target_os = "windows")]
    {
      use wry::WebViewBuilderExtWindows;
      if let Some(theme) = options["theme"].as_str() {
        let theme = match theme {
          "Light" => wry::Theme::Light,
          "Dark" => wry::Theme::Dark,
          _ => wry::Theme::Auto,
        };
        webview_builder = webview_builder.with_theme(theme);
      }
    }

    if let Some(preload) = preload {
      webview_builder = webview_builder.with_initialization_script(preload);
    }

    if let Some(url) = url {
      webview_builder = webview_builder.with_url(url);
    }

    if let Some(html) = html {
      webview_builder = webview_builder.with_html(html);
    }

    let webview = webview_builder
      .build(window)
      .map_err(|e| format!("Failed to create webview: {}", e))?;

    self.webviews.insert(window_id, webview);

    Ok(())
  }

  fn get_webview(&self, window_id: u32) -> Option<&wry::WebView> {
    self.webviews.get(&window_id)
  }

  fn remove_window(&mut self, window_id: u32) {
    if let Some(_window) = self.windows.remove(&window_id) {
      // Encontrar y remover el mapeo de tao_id
      let tao_id_to_remove: Vec<_> = self
        .tao_to_window_id
        .iter()
        .filter(|(_, &id)| id == window_id)
        .map(|(&tao_id, _)| tao_id)
        .collect();

      for tao_id in tao_id_to_remove {
        self.tao_to_window_id.remove(&tao_id);
      }
    }
  }

  fn get_window(&self, window_id: u32) -> Option<&Window> {
    self.windows.get(&window_id)
  }

  fn get_window_id(&self, tao_id: &tao::window::WindowId) -> Option<u32> {
    self.tao_to_window_id.get(tao_id).copied()
  }
}

/// Resultado de procesar una solicitud IPC
enum IpcRequestResult {
  Success(IpcResponse),
  CreateWindow(Box<WindowBuilder>, u32),
  Error(String),
}

/// Procesa una solicitud IPC en el eventloop
fn process_ipc_request(
  request: IpcRequest,
  window_manager: &mut WindowManager,
) -> IpcRequestResult {
  match request {
    IpcRequest::CreateBrowserWindow {
      window_id,
      options,
      is_child: _,
    } => match WindowManager::create_window_options(window_id, &options) {
      Ok((window_builder, wid)) => IpcRequestResult::CreateWindow(Box::new(window_builder), wid),
      Err(e) => IpcRequestResult::Error(e),
    },
    IpcRequest::CloseWindow { window_id } => {
      window_manager.remove_window(window_id);
      IpcRequestResult::Success(IpcResponse::Success {
        request_id: 0,
        data: Some(serde_json::json!({ "closed": true })),
      })
    }
    IpcRequest::CreateWebview { window_id, options } => {
      match window_manager.create_webview(window_id, &options) {
        Ok(()) => IpcRequestResult::Success(IpcResponse::Success {
          request_id: 0,
          data: Some(serde_json::json!({
              "window_id": window_id,
              "webview_created": true
          })),
        }),
        Err(e) => IpcRequestResult::Error(e),
      }
    }
    IpcRequest::EvaluateScript { window_id, script } => {
      if let Some(webview) = window_manager.get_webview(window_id) {
        match webview.evaluate_script(&script) {
          Ok(()) => IpcRequestResult::Success(IpcResponse::Success {
            request_id: 0,
            data: Some(serde_json::json!({
                "evaluated": true,
                "script_length": script.len()
            })),
          }),
          Err(e) => IpcRequestResult::Error(format!("Failed to evaluate script: {}", e)),
        }
      } else {
        IpcRequestResult::Error(format!("Webview {} not found", window_id))
      }
    }
    IpcRequest::LoadUrl { window_id, url } => {
      if let Some(webview) = window_manager.get_webview(window_id) {
        match webview.load_url(&url) {
          Ok(()) => IpcRequestResult::Success(IpcResponse::Success {
            request_id: 0,
            data: Some(serde_json::json!({
                "loaded": true,
                "url": url
            })),
          }),
          Err(e) => IpcRequestResult::Error(format!("Failed to load URL: {}", e)),
        }
      } else {
        IpcRequestResult::Error(format!("Webview {} not found", window_id))
      }
    }
    IpcRequest::LoadHtml { window_id, html } => {
      if let Some(webview) = window_manager.get_webview(window_id) {
        match webview.load_html(&html) {
          Ok(()) => IpcRequestResult::Success(IpcResponse::Success {
            request_id: 0,
            data: Some(serde_json::json!({
                "loaded": true,
                "html_length": html.len()
            })),
          }),
          Err(e) => IpcRequestResult::Error(format!("Failed to load HTML: {}", e)),
        }
      } else {
        IpcRequestResult::Error(format!("Webview {} not found", window_id))
      }
    }
    IpcRequest::SetWindowVisible { window_id, visible } => {
      if let Some(window) = window_manager.get_window(window_id) {
        window.set_visible(visible);
        IpcRequestResult::Success(IpcResponse::Success {
          request_id: 0,
          data: Some(serde_json::json!({ "visible": visible })),
        })
      } else {
        IpcRequestResult::Error(format!("Window {} not found", window_id))
      }
    }
    IpcRequest::SetWindowTitle { window_id, title } => {
      if let Some(window) = window_manager.get_window(window_id) {
        window.set_title(&title);
        IpcRequestResult::Success(IpcResponse::Success {
          request_id: 0,
          data: Some(serde_json::json!({ "title": title })),
        })
      } else {
        IpcRequestResult::Error(format!("Window {} not found", window_id))
      }
    }
    IpcRequest::Exit => IpcRequestResult::Success(IpcResponse::Success {
      request_id: 0,
      data: Some(serde_json::json!({ "exiting": true })),
    }),
    IpcRequest::Ping => IpcRequestResult::Success(IpcResponse::Pong),
  }
}

fn main() {
  // Obtener el puerto de los argumentos de línea de comandos
  let args: Vec<String> = env::args().collect();
  let port = if args.len() > 1 {
    args[1]
      .parse::<u16>()
      .expect("El puerto debe ser un número válido")
  } else {
    0 // Usar puerto aleatorio si no se especifica
  };

  // Crear listener TCP
  let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)) {
    Ok(l) => l,
    Err(e) => {
      eprintln!("Error al crear listener TCP: {}", e);
      std::process::exit(1);
    }
  };

  listener.set_nonblocking(true).ok();
  let actual_port = listener.local_addr().unwrap().port();
  println!("Eventloop process iniciado en puerto {}", actual_port);

  // Crear el EventLoop en el hilo principal de este proceso
  // En Windows, usamos any_thread para evitar restricciones
  #[cfg(target_os = "windows")]
  let event_loop = EventLoopBuilder::new().with_any_thread(true).build();

  #[cfg(not(target_os = "windows"))]
  let event_loop = EventLoop::new();

  let mut window_manager = WindowManager::new();
  let mut streams: Vec<TcpStream> = Vec::new();
  let mut buffer = vec![0u8; 8192];
  let should_exit = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

  // Ejecutar el eventloop
  event_loop.run(move |event, event_loop_target, control_flow| {
    *control_flow = ControlFlow::Poll;

    // Verificar si se solicitó salir
    if should_exit.load(std::sync::atomic::Ordering::SeqCst) {
      *control_flow = ControlFlow::Exit;
      return;
    }

    // Aceptar nuevas conexiones
    match listener.accept() {
      Ok((stream, _)) => {
        stream.set_nodelay(true).ok();
        stream.set_nonblocking(true).ok();
        streams.push(stream);
      }
      Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
      Err(_) => {}
    }

    // Leer de streams existentes y procesar solicitudes
    let mut streams_to_remove = Vec::new();
    for (idx, stream) in streams.iter_mut().enumerate() {
      match stream.read(&mut buffer) {
        Ok(0) => {
          // Conexión cerrada
          streams_to_remove.push(idx);
        }
        Ok(n) => {
          let data = &buffer[..n];
          if let Ok(message) = serde_json::from_slice::<IpcMessage<IpcRequest>>(data) {
            let request_id = message.request_id;
            let request = message.payload;

            // Procesar la solicitud
            let result = process_ipc_request(request, &mut window_manager);

            let (response, should_exit_flag) = match result {
              IpcRequestResult::Success(resp) => (resp, false),
              IpcRequestResult::Error(e) => (
                IpcResponse::Error {
                  request_id,
                  message: e,
                },
                false,
              ),
              IpcRequestResult::CreateWindow(window_builder, window_id) => {
                // Crear la ventana
                match window_builder.build(event_loop_target) {
                  Ok(window) => {
                    window_manager.add_window(window_id, window);
                    (
                      IpcResponse::Success {
                        request_id: 0,
                        data: Some(serde_json::json!({
                            "window_id": window_id,
                            "success": true
                        })),
                      },
                      false,
                    )
                  }
                  Err(e) => (
                    IpcResponse::Error {
                      request_id,
                      message: format!("Failed to create window: {}", e),
                    },
                    false,
                  ),
                }
              }
            };

            // Si se solicitó salir, actualizar el flag
            if should_exit_flag {
              should_exit.store(true, std::sync::atomic::Ordering::SeqCst);
            }

            // Enviar respuesta
            if let Ok(resp_data) = serde_json::to_vec(&IpcMessage {
              request_id,
              payload: response,
            }) {
              let _ = stream.write_all(&resp_data);
            }
          }
        }
        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
        Err(_) => {
          streams_to_remove.push(idx);
        }
      }
    }

    // Remover streams desconectados (en orden inverso para mantener índices válidos)
    for idx in streams_to_remove.into_iter().rev() {
      streams.remove(idx);
    }

    // Manejar eventos de ventana
    if let Event::WindowEvent {
      event: WindowEvent::CloseRequested,
      window_id,
      ..
    } = event
    {
      // Manejar cierre de ventana
      if let Some(window_id_num) = window_manager.get_window_id(&window_id) {
        window_manager.remove_window(window_id_num);

        // Enviar evento de cierre al proceso principal
        let response = IpcResponse::ApplicationEvent {
          event_type: "WindowCloseRequested".to_string(),
          window_id: Some(window_id_num),
        };
        if let Ok(resp_data) = serde_json::to_vec(&IpcMessage {
          request_id: 0,
          payload: response,
        }) {
          for stream in streams.iter_mut() {
            let _ = stream.write_all(&resp_data);
          }
        }
      }
    }
  });
}
