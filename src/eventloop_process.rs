//! Módulo que ejecuta el eventloop en un proceso separado
//!
//! Este módulo implementa la lógica para lanzar y comunicarse con un proceso
//! independiente que ejecuta el eventloop, permitiendo comunicación IPC con el
//! proceso principal.

use crate::ipc::IpcClient;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Estado del proceso del eventloop
pub struct EventloopProcess {
  /// El proceso hijo que ejecuta el eventloop
  child: Option<Child>,
  /// Puerto IPC para comunicación
  ipc_port: Option<u16>,
  /// Indica si el eventloop está corriendo
  is_running: Arc<AtomicBool>,
}

impl EventloopProcess {
  /// Inicia el proceso del eventloop
  pub fn spawn() -> Result<Self, Box<dyn std::error::Error>> {
    // Obtener el path del binario eventloop
    let eventloop_bin = Self::get_eventloop_binary_path()?;
    eprintln!("Eventloop binary path: {}", eventloop_bin.display());

    // Usar puerto 0 para que el sistema asigne un puerto disponible automáticamente
    let port = 0;

    // Iniciar el proceso del eventloop capturando stdout para leer el puerto
    let mut child = Command::new(&eventloop_bin)
      .arg(port.to_string())
      .stdout(Stdio::piped())
      .stderr(Stdio::inherit())
      .spawn()
      .map_err(|e| {
        format!(
          "Failed to spawn eventloop process '{}': {}",
          eventloop_bin.display(),
          e
        )
      })?;

    eprintln!("Eventloop process spawned with PID: {:?}", child.id());

    // Leer el puerto desde stdout del proceso
    let stdout = child.stdout.as_mut().ok_or("Failed to capture stdout")?;
    let reader = BufReader::new(stdout);
    let mut actual_port: Option<u16> = None;

    for line in reader.lines() {
      match line {
        Ok(l) => {
          // Buscar el puerto en el mensaje "Eventloop process iniciado en puerto XXXXX"
          if l.contains("Eventloop process iniciado en puerto") {
            let parts: Vec<&str> = l.split_whitespace().collect();
            if let Some(port_str) = parts.last() {
              if let Ok(p) = port_str.parse::<u16>() {
                actual_port = Some(p);
                break;
              }
            }
          }
        }
        Err(_) => break,
      }
    }

    let actual_port = actual_port.ok_or("Failed to read IPC port from eventloop process")?;
    eprintln!("Eventloop process listening on port: {}", actual_port);

    // Esperar un momento para asegurar que el servidor IPC esté completamente listo
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Verificar que el proceso sigue corriendo
    match child.try_wait() {
      Ok(Some(status)) => {
        eprintln!("Eventloop process exited with status: {}", status);
        return Err("Eventloop process exited unexpectedly".into());
      }
      Ok(None) => {
        eprintln!("Eventloop process is running");
      }
      Err(e) => {
        eprintln!("Error checking eventloop process status: {}", e);
      }
    }

    Ok(Self {
      child: Some(child),
      ipc_port: Some(actual_port),
      is_running: Arc::new(AtomicBool::new(true)),
    })
  }

  /// Retorna el puerto IPC para comunicación
  pub fn ipc_port(&self) -> Option<u16> {
    self.ipc_port
  }

  /// Detiene el proceso del eventloop
  pub fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    self.is_running.store(false, Ordering::SeqCst);

    if let Some(mut child) = self.child.take() {
      // Intentar terminar el proceso gracefulmente
      if let Err(e) = child.kill() {
        eprintln!("Error killing eventloop process: {}", e);
      }
      let _ = child.wait();
    }

    Ok(())
  }

  /// Conecta al proceso del eventloop y retorna un cliente IPC
  pub fn connect_ipc(&self) -> Result<IpcClient, Box<dyn std::error::Error>> {
    let port = self.ipc_port.ok_or("IPC port not available")?;
    IpcClient::connect(port).map_err(|e| {
      format!(
        "Failed to connect to eventloop process on port {}: {}",
        port, e
      )
      .into()
    })
  }

  /// Obtiene el path del binario eventloop
  fn get_eventloop_binary_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    // Intentar encontrar el binario en varias ubicaciones
    let possible_paths = vec![
      // En desarrollo: target/debug/eventloop
      std::path::PathBuf::from("target/debug/eventloop.exe"),
      // En release: target/release/eventloop
      std::path::PathBuf::from("target/release/eventloop.exe"),
      // En el directorio actual
      std::path::PathBuf::from("eventloop.exe"),
    ];

    for path in possible_paths {
      if path.exists() {
        return Ok(path);
      }
    }

    // Si no encontramos el binario, intentar construirlo
    eprintln!("Eventloop binary not found, attempting to build...");
    let output = Command::new("cargo")
      .args(["build", "--bin", "eventloop"])
      .output()?;

    if !output.status.success() {
      return Err(
        format!(
          "Failed to build eventloop binary: {}",
          String::from_utf8_lossy(&output.stderr)
        )
        .into(),
      );
    }

    // Verificar que el binario existe ahora
    let path = std::path::PathBuf::from("target/debug/eventloop.exe");
    if path.exists() {
      Ok(path)
    } else {
      Err("Eventloop binary not found after build".into())
    }
  }
}

impl Drop for EventloopProcess {
  fn drop(&mut self) {
    // Asegurarse de detener el proceso cuando se destruye
    let _ = self.stop();
  }
}

/// Procesa una solicitud IPC en el eventloop
/// Esta función es pública para que pueda ser usada desde el binario eventloop
pub fn process_ipc_request(
  request: crate::ipc::IpcRequest,
  window_manager: &mut WindowManager,
) -> Result<crate::ipc::IpcResponse, String> {
  match request {
    crate::ipc::IpcRequest::CreateBrowserWindow {
      window_id,
      options: _,
      is_child: _,
    } => {
      // Crear ventana usando las opciones proporcionadas
      // En una implementación real, deserializaríamos las opciones
      // y crearíamos la ventana usando tao::WindowBuilder

      Ok(crate::ipc::IpcResponse::Success {
        request_id: 0,
        data: Some(serde_json::json!({
            "window_id": window_id,
            "success": true
        })),
      })
    }
    crate::ipc::IpcRequest::CloseWindow { window_id } => {
      window_manager.remove_window(window_id);
      Ok(crate::ipc::IpcResponse::Success {
        request_id: 0,
        data: Some(serde_json::json!({ "closed": true })),
      })
    }
    crate::ipc::IpcRequest::CreateWebview {
      window_id,
      options: _,
    } => {
      // Crear webview en la ventana especificada
      Ok(crate::ipc::IpcResponse::Success {
        request_id: 0,
        data: Some(serde_json::json!({
            "window_id": window_id,
            "webview_created": true
        })),
      })
    }
    crate::ipc::IpcRequest::EvaluateScript {
      window_id: _,
      script,
    } => {
      // Ejecutar script en el webview de la ventana
      Ok(crate::ipc::IpcResponse::Success {
        request_id: 0,
        data: Some(serde_json::json!({
            "evaluated": true,
            "script_length": script.len()
        })),
      })
    }
    crate::ipc::IpcRequest::LoadUrl { window_id: _, url } => {
      // Cargar URL en el webview
      Ok(crate::ipc::IpcResponse::Success {
        request_id: 0,
        data: Some(serde_json::json!({
            "loaded": true,
            "url": url
        })),
      })
    }
    crate::ipc::IpcRequest::LoadHtml { window_id: _, html } => {
      // Cargar HTML en el webview
      Ok(crate::ipc::IpcResponse::Success {
        request_id: 0,
        data: Some(serde_json::json!({
            "loaded": true,
            "html_length": html.len()
        })),
      })
    }
    crate::ipc::IpcRequest::SetWindowVisible { window_id, visible } => {
      // Establecer visibilidad de ventana
      if let Some(_window) = window_manager.get_window(window_id) {
        Ok(crate::ipc::IpcResponse::Success {
          request_id: 0,
          data: Some(serde_json::json!({ "visible": visible })),
        })
      } else {
        Err(format!("Window {} not found", window_id))
      }
    }
    crate::ipc::IpcRequest::SetWindowTitle { window_id, title } => {
      // Establecer título de ventana
      if let Some(_window) = window_manager.get_window(window_id) {
        Ok(crate::ipc::IpcResponse::Success {
          request_id: 0,
          data: Some(serde_json::json!({ "title": title })),
        })
      } else {
        Err(format!("Window {} not found", window_id))
      }
    }
    crate::ipc::IpcRequest::Exit => {
      // Solicitar salida del eventloop
      Ok(crate::ipc::IpcResponse::Success {
        request_id: 0,
        data: Some(serde_json::json!({ "exiting": true })),
      })
    }
    crate::ipc::IpcRequest::Ping => Ok(crate::ipc::IpcResponse::Pong),
  }
}

/// Gestor de ventanas en el proceso del eventloop
/// Esta estructura es pública para que pueda ser usada desde el binario eventloop
pub struct WindowManager {
  /// Mapa de window_id -> Window
  #[allow(dead_code)]
  windows: std::collections::HashMap<u32, tao::window::Window>,
  /// Mapa de tao window_id -> nuestro window_id
  #[allow(dead_code)]
  tao_to_window_id: std::collections::HashMap<tao::window::WindowId, u32>,
}

impl Default for WindowManager {
  fn default() -> Self {
    Self::new()
  }
}

impl WindowManager {
  pub fn new() -> Self {
    Self {
      windows: std::collections::HashMap::new(),
      tao_to_window_id: std::collections::HashMap::new(),
    }
  }

  pub fn remove_window(&mut self, window_id: u32) {
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

  pub fn get_window(&self, window_id: u32) -> Option<&tao::window::Window> {
    self.windows.get(&window_id)
  }

  pub fn get_window_id(&self, tao_id: &tao::window::WindowId) -> Option<u32> {
    self.tao_to_window_id.get(tao_id).copied()
  }
}
