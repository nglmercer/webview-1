#![deny(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};

use browser_window::{BrowserWindow, BrowserWindowOptions};
use napi::bindgen_prelude::*;
use napi::Result;
use napi_derive::napi;
use tao::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
};

pub mod browser_window;
pub mod eventloop_process;
pub mod ipc;
pub mod webview;

/// Contador global para IDs de ventana
static WINDOW_ID_COUNTER: AtomicU32 = AtomicU32::new(1);

#[napi]
/// TODO
pub enum WebviewApplicationEvent {
  /// Window close event.
  WindowCloseRequested,
  /// Application close event.
  ApplicationCloseRequested,
}

#[napi(object)]
pub struct HeaderData {
  /// The key of the header.
  pub key: String,
  /// The value of the header.
  pub value: Option<String>,
}

#[napi(object)]
pub struct IpcMessage {
  /// The body of the message.
  pub body: Buffer,
  /// The HTTP method of the message.
  pub method: String,
  /// The http headers of the message.
  pub headers: Vec<HeaderData>,
  /// The URI of the message.
  pub uri: String,
}

#[napi]
/// Returns the version of the webview.
pub fn get_webview_version() -> Result<String> {
  wry::webview_version().map_err(|e| {
    napi::Error::new(
      napi::Status::GenericFailure,
      format!("Failed to get webview version: {}", e),
    )
  })
}

#[napi(js_name = "ControlFlow")]
/// Represents the control flow of the application.
pub enum JsControlFlow {
  /// The application will continue running.
  Poll,
  /// The application will wait until the specified time.
  WaitUntil,
  /// The application will exit.
  Exit,
  /// The application will exit with the given exit code.
  ExitWithCode,
}

#[napi(object)]
/// Represents the options for creating an application.
pub struct ApplicationOptions {
  /// The control flow of the application. Default is `Poll`.
  pub control_flow: Option<JsControlFlow>,
  /// The waiting time in ms for the application (only applicable if control flow is set to `WaitUntil`).
  pub wait_time: Option<i32>,
  /// The exit code of the application. Only applicable if control flow is set to `ExitWithCode`.
  pub exit_code: Option<i32>,
  /// Whether to prevent the window from closing. Default is `false`.
  pub prevent_close: Option<bool>,
}

#[napi(object)]
/// Represents an event for the application.
pub struct ApplicationEvent {
  /// The event type.
  pub event: WebviewApplicationEvent,
}

#[napi]
/// Represents an application.
pub struct Application {
  /// The event loop (deprecated - kept for backward compatibility).
  event_loop: Option<EventLoop<()>>,
  /// The options for creating the application.
  options: ApplicationOptions,
  /// The event handler for the application.
  handler: Rc<RefCell<Option<FunctionRef<ApplicationEvent, ()>>>>,
  /// The env
  env: Env,
  /// Whether the application should exit
  should_exit: Rc<RefCell<bool>>,
  /// Set of open window IDs
  open_windows: Rc<RefCell<HashSet<u32>>>,
  /// IPC client for communicating with the eventloop process
  ipc_client: Rc<RefCell<Option<ipc::IpcClient>>>,
  /// Whether to use IPC mode (non-blocking)
  use_ipc: bool,
  /// Pointer to the eventloop process (only used in IPC mode)
  _eventloop_process: Option<*mut eventloop_process::EventloopProcess>,
}

#[napi]
impl Application {
  #[napi(constructor)]
  /// Creates a new application.
  pub fn new(env: Env, options: Option<ApplicationOptions>) -> Result<Self> {
    let event_loop = EventLoop::new();

    Ok(Self {
      event_loop: Some(event_loop),
      options: options.unwrap_or(ApplicationOptions {
        control_flow: Some(JsControlFlow::Poll),
        wait_time: None,
        exit_code: None,
        prevent_close: None,
      }),
      handler: Rc::new(RefCell::new(None::<FunctionRef<ApplicationEvent, ()>>)),
      env,
      should_exit: Rc::new(RefCell::new(false)),
      open_windows: Rc::new(RefCell::new(HashSet::new())),
      ipc_client: Rc::new(RefCell::new(None)),
      use_ipc: false,
      _eventloop_process: None,
    })
  }

  #[napi]
  /// Creates a new application in non-blocking mode using IPC.
  /// This allows the eventloop to run in a separate process, preventing
  /// the JavaScript thread from being blocked.
  pub fn new_non_blocking(env: Env, options: Option<ApplicationOptions>) -> Result<Self> {
    // Iniciar el proceso del eventloop
    let eventloop_process = eventloop_process::EventloopProcess::spawn().map_err(|e| {
      napi::Error::new(
        napi::Status::GenericFailure,
        format!("Failed to spawn eventloop process: {}", e),
      )
    })?;

    let _port = eventloop_process.ipc_port().ok_or_else(|| {
      napi::Error::new(
        napi::Status::GenericFailure,
        "Failed to get IPC port from eventloop process",
      )
    })?;

    // Conectar al proceso del eventloop
    let ipc_client = eventloop_process.connect_ipc().map_err(|e| {
      napi::Error::new(
        napi::Status::GenericFailure,
        format!("Failed to connect to eventloop process: {}", e),
      )
    })?;

    // Convertir el proceso a Box y guardarlo en un Rc para mantenerlo vivo
    let eventloop_process = Box::new(eventloop_process);
    let eventloop_process_ptr = Box::into_raw(eventloop_process);

    Ok(Self {
      event_loop: None, // No eventloop directo en modo IPC
      options: options.unwrap_or(ApplicationOptions {
        control_flow: Some(JsControlFlow::Poll),
        wait_time: None,
        exit_code: None,
        prevent_close: None,
      }),
      handler: Rc::new(RefCell::new(None::<FunctionRef<ApplicationEvent, ()>>)),
      env,
      should_exit: Rc::new(RefCell::new(false)),
      open_windows: Rc::new(RefCell::new(HashSet::new())),
      ipc_client: Rc::new(RefCell::new(Some(ipc_client))),
      use_ipc: true,
      _eventloop_process: Some(eventloop_process_ptr),
    })
  }

  #[napi]
  /// Sets the event handler callback.
  pub fn on_event(&mut self, handler: Option<FunctionRef<ApplicationEvent, ()>>) {
    *self.handler.borrow_mut() = handler;
  }

  #[napi]
  /// Creates a new browser window.
  pub fn create_browser_window(
    &'static mut self,
    options: Option<BrowserWindowOptions>,
  ) -> Result<BrowserWindow> {
    if self.use_ipc {
      // Modo IPC: enviar solicitud al proceso del eventloop
      let window_id = WINDOW_ID_COUNTER.fetch_add(1, Ordering::SeqCst);

      let ipc_client_ref = self.ipc_client.borrow();
      let client = ipc_client_ref.as_ref().ok_or_else(|| {
        napi::Error::new(napi::Status::GenericFailure, "IPC client not initialized")
      })?;

      let options_json = serde_json::to_value(options.unwrap_or_default()).map_err(|e| {
        napi::Error::new(
          napi::Status::GenericFailure,
          format!("Failed to serialize options: {}", e),
        )
      })?;

      let request = ipc::IpcRequest::CreateBrowserWindow {
        window_id,
        options: options_json,
        is_child: false,
      };

      client.send_request(request).map_err(|e| {
        napi::Error::new(
          napi::Status::GenericFailure,
          format!("Failed to send IPC request: {}", e),
        )
      })?;

      // Registrar ventana en open_windows
      self.open_windows.borrow_mut().insert(window_id);

      // Retornar un BrowserWindow proxy que usa IPC
      let ipc_client_for_proxy = self.ipc_client.clone();
      Ok(BrowserWindow::new_ipc_proxy(
        window_id,
        ipc_client_for_proxy,
      ))
    } else {
      // Modo tradicional: usar eventloop directo
      let event_loop = self.event_loop.as_ref();

      if event_loop.is_none() {
        return Err(napi::Error::new(
          napi::Status::GenericFailure,
          "Event loop is not initialized",
        ));
      }

      let window_id = WINDOW_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
      let window = BrowserWindow::new(event_loop.unwrap(), options, false, window_id)?;

      // Register window in open_windows set
      self.open_windows.borrow_mut().insert(window_id);

      Ok(window)
    }
  }

  #[napi]
  /// Creates a new browser window as a child window.
  pub fn create_child_browser_window(
    &'static mut self,
    options: Option<BrowserWindowOptions>,
  ) -> Result<BrowserWindow> {
    if self.use_ipc {
      // Modo IPC: enviar solicitud al proceso del eventloop
      let window_id = WINDOW_ID_COUNTER.fetch_add(1, Ordering::SeqCst);

      let ipc_client_ref = self.ipc_client.borrow();
      let client = ipc_client_ref.as_ref().ok_or_else(|| {
        napi::Error::new(napi::Status::GenericFailure, "IPC client not initialized")
      })?;

      let options_json = serde_json::to_value(options.unwrap_or_default()).map_err(|e| {
        napi::Error::new(
          napi::Status::GenericFailure,
          format!("Failed to serialize options: {}", e),
        )
      })?;

      let request = ipc::IpcRequest::CreateBrowserWindow {
        window_id,
        options: options_json,
        is_child: true,
      };

      client.send_request(request).map_err(|e| {
        napi::Error::new(
          napi::Status::GenericFailure,
          format!("Failed to send IPC request: {}", e),
        )
      })?;

      // Registrar ventana en open_windows
      self.open_windows.borrow_mut().insert(window_id);

      // Retornar un BrowserWindow proxy que usa IPC
      let ipc_client_for_proxy = self.ipc_client.clone();
      Ok(BrowserWindow::new_ipc_proxy(
        window_id,
        ipc_client_for_proxy,
      ))
    } else {
      // Modo tradicional: usar eventloop directo
      let event_loop = self.event_loop.as_ref();

      if event_loop.is_none() {
        return Err(napi::Error::new(
          napi::Status::GenericFailure,
          "Event loop is not initialized",
        ));
      }

      let window_id = WINDOW_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
      let window = BrowserWindow::new(event_loop.unwrap(), options, true, window_id)?;

      // Register window in open_windows set
      self.open_windows.borrow_mut().insert(window_id);

      Ok(window)
    }
  }

  #[napi]
  /// Closes a specific window by ID.
  pub fn close_window(&self, window_id: u32) {
    if self.use_ipc {
      // Modo IPC: enviar solicitud al proceso del eventloop
      let ipc_client = self.ipc_client.borrow();
      if let Some(client) = ipc_client.as_ref() {
        let request = ipc::IpcRequest::CloseWindow { window_id };
        let _ = client.send_request_async(request);
      }
    }

    // Remover del set local
    self.open_windows.borrow_mut().remove(&window_id);
  }

  #[napi]
  /// Exits the application gracefully. This will trigger the close event and clean up resources.
  pub fn exit(&self) {
    if self.use_ipc {
      // Modo IPC: enviar solicitud de salida al proceso del eventloop
      let ipc_client = self.ipc_client.borrow();
      if let Some(client) = ipc_client.as_ref() {
        let request = ipc::IpcRequest::Exit;
        let _ = client.send_request_async(request);
      }

      // Esperar un momento para que el proceso del eventloop se cierre
      std::thread::sleep(std::time::Duration::from_millis(500));

      // Cerrar el proceso del eventloop
      if let Some(ptr) = self._eventloop_process {
        unsafe {
          if !ptr.is_null() {
            let _ = Box::from_raw(ptr).stop();
          }
        }
      }
    }

    *self.should_exit.borrow_mut() = true;
  }

  #[napi]
  /// Runs the application. This method will block the current thread in traditional mode,
  /// but is NON-BLOCKING in IPC mode (when using new_non_blocking()).
  ///
  /// IMPORTANT: In traditional mode, this method is BLOCKING and will prevent JavaScript
  /// from executing. All setTimeout/setInterval callbacks must be scheduled BEFORE calling
  /// this method.
  ///
  /// In IPC mode (created with new_non_blocking()), this method returns immediately and
  /// the eventloop runs in a separate process. You can continue executing JavaScript code.
  pub fn run(&mut self) -> Result<()> {
    if self.use_ipc {
      // Modo IPC: el eventloop ya está corriendo en proceso separado
      // Solo retornamos inmediatamente - NO BLOQUEANTE
      Ok(())
    } else {
      // Modo tradicional: ejecutar eventloop en el hilo actual - BLOQUEANTE
      let ctrl = match self.options.control_flow {
        None => ControlFlow::Poll,
        Some(JsControlFlow::Poll) => ControlFlow::Poll,
        Some(JsControlFlow::WaitUntil) => {
          let wait_time = self.options.wait_time.unwrap_or(0);
          ControlFlow::WaitUntil(
            std::time::Instant::now() + std::time::Duration::from_millis(wait_time as u64),
          )
        }
        Some(JsControlFlow::Exit) => ControlFlow::Exit,
        Some(JsControlFlow::ExitWithCode) => {
          let exit_code = self.options.exit_code.unwrap_or(0);
          ControlFlow::ExitWithCode(exit_code)
        }
      };

      let prevent_close = self.options.prevent_close.unwrap_or(false);
      let open_windows = self.open_windows.clone();

      if let Some(event_loop) = self.event_loop.take() {
        let handler = self.handler.clone();
        let env = self.env;
        let should_exit = self.should_exit.clone();

        event_loop.run(move |event, _, control_flow| {
          *control_flow = ctrl;

          // Check if exit was requested
          if *should_exit.borrow() {
            let callback = handler.borrow();
            if let Some(callback) = callback.as_ref() {
              if let Ok(on_exit) = callback.borrow_back(&env) {
                let _ = on_exit.call(ApplicationEvent {
                  event: WebviewApplicationEvent::ApplicationCloseRequested,
                });
              }
            }
            *control_flow = ControlFlow::Exit;
            return;
          }

          if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
          } = event
          {
            let callback = handler.borrow();
            if let Some(callback) = callback.as_ref() {
              if let Ok(callback_fn) = callback.borrow_back(&env) {
                let _ = callback_fn.call(ApplicationEvent {
                  event: WebviewApplicationEvent::WindowCloseRequested,
                });
              }
            }

            // Check if all windows are closed and prevent_close is false
            if !prevent_close && open_windows.borrow().is_empty() {
              *control_flow = ControlFlow::Exit;
            }
          }
        });
      }

      Ok(())
    }
  }
}

impl Drop for Application {
  fn drop(&mut self) {
    // Asegurarse de cerrar el proceso del eventloop en modo IPC
    if self.use_ipc {
      if let Some(ptr) = self._eventloop_process {
        unsafe {
          if !ptr.is_null() {
            let _ = Box::from_raw(ptr).stop();
          }
        }
      }
    }
  }
}
