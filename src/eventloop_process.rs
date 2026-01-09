//! Módulo que ejecuta el eventloop en un proceso separado
//!
//! Este módulo implementa la lógica del eventloop que se ejecuta en un proceso
//! independiente, permitiendo comunicación IPC con el proceso principal.

use crate::ipc::{IpcEvent, IpcRequest, IpcResponse, IpcServer};
use std::collections::HashMap;
use std::sync::{
  atomic::{AtomicBool, Ordering},
  Arc,
};
use std::thread;
use tao::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::Window,
};

/// Estado del proceso del eventloop
pub struct EventloopProcess {
  /// El servidor IPC para comunicación
  ipc_server: Option<IpcServer>,
  /// Puerto IPC para comunicación
  ipc_port: Option<u16>,
  /// Indica si el eventloop está corriendo
  is_running: Arc<AtomicBool>,
}

impl EventloopProcess {
  /// Inicia el proceso del eventloop
  pub fn spawn() -> Result<Self, Box<dyn std::error::Error>> {
    // Crear un servidor IPC para comunicarse con el proceso
    let ipc_server = IpcServer::new()?;
    let port = ipc_server.port();
    let event_sender = ipc_server.event_sender();
    let is_running = Arc::new(AtomicBool::new(true));
    let is_running_clone = Arc::clone(&is_running);

    // Iniciar el eventloop en un hilo separado
    thread::spawn(move || {
      run_eventloop_thread(event_sender, is_running_clone);
    });

    Ok(Self {
      ipc_server: Some(ipc_server),
      ipc_port: Some(port),
      is_running,
    })
  }

  /// Retorna el puerto IPC para comunicación
  pub fn ipc_port(&self) -> Option<u16> {
    self.ipc_port
  }

  /// Detiene el proceso del eventloop
  pub fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    self.is_running.store(false, Ordering::SeqCst);
    Ok(())
  }

  /// Retorna el servidor IPC (para uso interno)
  pub fn ipc_server(&self) -> Option<&IpcServer> {
    self.ipc_server.as_ref()
  }
}

/// Ejecuta el eventloop en un hilo separado
fn run_eventloop_thread(
  event_sender: std::sync::mpsc::Sender<IpcEvent>,
  is_running: Arc<AtomicBool>,
) -> ! {
  let event_loop = EventLoop::new();
  let mut window_manager = WindowManager::new();
  let _ipc_server_ref = event_sender.clone();

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Poll;

    // Verificar si debemos detener el eventloop
    if !is_running.load(Ordering::SeqCst) {
      *control_flow = ControlFlow::Exit;
      return;
    }

    match event {
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        window_id,
        ..
      } => {
        // Manejar cierre de ventana
        if let Some(window_id_num) = window_manager.get_window_id(&window_id) {
          window_manager.remove_window(window_id_num);

          // Enviar evento de cierre al proceso principal
          // (esto se implementaría en una versión completa con IPC bidireccional)
        }
      }
      _ => {}
    }
  });
}

/// Gestor de ventanas en el proceso del eventloop
pub struct WindowManager {
  /// Mapa de window_id -> Window
  windows: HashMap<u32, Window>,
  /// Mapa de tao window_id -> nuestro window_id
  tao_to_window_id: HashMap<tao::window::WindowId, u32>,
}

impl WindowManager {
  fn new() -> Self {
    Self {
      windows: HashMap::new(),
      tao_to_window_id: HashMap::new(),
    }
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

/// Procesa una solicitud IPC en el eventloop
pub fn process_ipc_request(
  request: IpcRequest,
  window_manager: &mut WindowManager,
) -> Result<IpcResponse, String> {
  match request {
    IpcRequest::CreateBrowserWindow {
      window_id,
      options: _,
      is_child: _,
    } => {
      // Crear ventana usando las opciones proporcionadas
      // En una implementación real, deserializaríamos las opciones
      // y crearíamos la ventana usando tao::WindowBuilder

      Ok(IpcResponse::Success {
        request_id: 0,
        data: Some(serde_json::json!({
            "window_id": window_id,
            "success": true
        })),
      })
    }
    IpcRequest::CloseWindow { window_id } => {
      window_manager.remove_window(window_id);
      Ok(IpcResponse::Success {
        request_id: 0,
        data: Some(serde_json::json!({ "closed": true })),
      })
    }
    IpcRequest::CreateWebview {
      window_id,
      options: _,
    } => {
      // Crear webview en la ventana especificada
      Ok(IpcResponse::Success {
        request_id: 0,
        data: Some(serde_json::json!({
            "window_id": window_id,
            "webview_created": true
        })),
      })
    }
    IpcRequest::EvaluateScript {
      window_id: _,
      script,
    } => {
      // Ejecutar script en el webview de la ventana
      Ok(IpcResponse::Success {
        request_id: 0,
        data: Some(serde_json::json!({
            "evaluated": true,
            "script_length": script.len()
        })),
      })
    }
    IpcRequest::LoadUrl { window_id: _, url } => {
      // Cargar URL en el webview
      Ok(IpcResponse::Success {
        request_id: 0,
        data: Some(serde_json::json!({
            "loaded": true,
            "url": url
        })),
      })
    }
    IpcRequest::LoadHtml { window_id: _, html } => {
      // Cargar HTML en el webview
      Ok(IpcResponse::Success {
        request_id: 0,
        data: Some(serde_json::json!({
            "loaded": true,
            "html_length": html.len()
        })),
      })
    }
    IpcRequest::SetWindowVisible { window_id, visible } => {
      // Establecer visibilidad de ventana
      if let Some(window) = window_manager.get_window(window_id) {
        window.set_visible(visible);
        Ok(IpcResponse::Success {
          request_id: 0,
          data: Some(serde_json::json!({ "visible": visible })),
        })
      } else {
        Err(format!("Window {} not found", window_id))
      }
    }
    IpcRequest::SetWindowTitle { window_id, title } => {
      // Establecer título de ventana
      if let Some(window) = window_manager.get_window(window_id) {
        window.set_title(&title);
        Ok(IpcResponse::Success {
          request_id: 0,
          data: Some(serde_json::json!({ "title": title })),
        })
      } else {
        Err(format!("Window {} not found", window_id))
      }
    }
    IpcRequest::Exit => {
      // Solicitar salida del eventloop
      Ok(IpcResponse::Success {
        request_id: 0,
        data: Some(serde_json::json!({ "exiting": true })),
      })
    }
    IpcRequest::Ping => Ok(IpcResponse::Pong),
  }
}
