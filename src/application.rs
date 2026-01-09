//! Main application module
//!
//! This module contains the Application structure and its implementation,
//! which coordinates the event loop, events, and window creation.

use crate::browser_window::BrowserWindow;
use crate::event_loop::TaoEventLoop;
use crate::events::{AppState, EventHandler};
use crate::types::{ApplicationEvent, ApplicationOptions, BrowserWindowOptions, JsControlFlow};
use napi::{bindgen_prelude::FunctionRef, Env, Result};
use napi_derive::napi;

/// Represents a webview application
#[napi]
pub struct Application {
  /// The tao event loop
  event_loop: TaoEventLoop,
  /// The event handler
  event_handler: EventHandler,
  /// The application state
  app_state: AppState,
}

#[napi]
impl Application {
  /// Creates a new application
  #[napi(constructor)]
  pub fn new(env: Env, options: Option<ApplicationOptions>) -> Result<Self> {
    let options = options.unwrap_or(ApplicationOptions {
      control_flow: Some(JsControlFlow::Poll),
      wait_time: None,
      exit_code: None,
    });

    let event_loop = TaoEventLoop::new(options.clone());
    let event_handler = EventHandler::new(env);
    let app_state = AppState::new();

    Ok(Self {
      event_loop,
      event_handler,
      app_state,
    })
  }

  /// Sets the event callback
  #[napi]
  pub fn on_event(&mut self, handler: Option<FunctionRef<ApplicationEvent, ()>>) {
    self.event_handler.set_callback(handler);
  }

  /// Alias for on_event() - binds an event callback
  #[napi]
  pub fn bind(&mut self, handler: Option<FunctionRef<ApplicationEvent, ()>>) {
    self.event_handler.set_callback(handler);
  }

  /// Creates a new browser window
  #[napi]
  pub fn create_browser_window(
    &'static mut self,
    options: Option<BrowserWindowOptions>,
  ) -> Result<BrowserWindow> {
    self.create_window(options, false)
  }

  /// Creates a new browser window as a child window
  #[napi]
  pub fn create_child_browser_window(
    &'static mut self,
    options: Option<BrowserWindowOptions>,
  ) -> Result<BrowserWindow> {
    self.create_window(options, true)
  }

  /// Internal method to create a browser window
  fn create_window(
    &mut self,
    options: Option<BrowserWindowOptions>,
    is_child: bool,
  ) -> Result<BrowserWindow> {
    let event_loop = self.event_loop.event_loop();

    if event_loop.is_none() {
      return Err(napi::Error::new(
        napi::Status::GenericFailure,
        "Event loop is not initialized",
      ));
    }

    BrowserWindow::new(event_loop.unwrap(), options, is_child)
  }

  /// Exits the application gracefully
  #[napi]
  pub fn exit(&self) {
    self.app_state.request_exit();
  }

  /// Runs the application in blocking mode
  ///
  /// This method will block the current thread until the application terminates.
  /// Note: This will block the Node.js event loop.
  #[napi]
  pub fn run(&mut self) -> Result<()> {
    self
      .event_loop
      .run_blocking(self.event_handler.clone(), self.app_state.clone_state())
  }

  /// Runs the application with a worker thread (future)
  ///
  /// This method will allow the UI event loop to run on the main thread
  /// while a worker thread handles business logic without blocking Node.js.
  ///
  /// TODO: Implement this functionality
  #[napi]
  pub fn run_with_worker(
    &mut self,
    _worker_callback: napi::threadsafe_function::ThreadsafeFunction<String>,
  ) -> Result<()> {
    // This implementation will require:
    // 1. Create a worker thread in Rust
    // 2. Use napi_threadsafe_function for communication
    // 3. Coordinate the UI event loop with the worker
    unimplemented!("run_with_worker is not yet implemented")
  }

  /// Runs the application in detached mode (future)
  ///
  /// This method will allow the server to keep running after
  /// the window is closed.
  ///
  /// TODO: Implement this functionality
  #[napi]
  pub fn run_detached(&mut self, _keep_server_alive: bool) -> Result<()> {
    // This implementation will require:
    // 1. Separate the event loop lifecycle from the server
    // 2. Allow the worker thread to keep running after closing the window
    unimplemented!("run_detached is not yet implemented")
  }
}
