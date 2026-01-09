//! Tao event loop abstraction
//!
//! This module provides an interface to handle the tao event loop,
//! facilitating the implementation of different execution strategies
//! (blocking, with worker, etc.).

use crate::events::{AppState, EventHandler};
use crate::types::ApplicationOptions;
use napi::Result;
use tao::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
};

/// Tao event loop wrapper
pub struct TaoEventLoop {
  /// The tao event loop
  event_loop: Option<EventLoop<()>>,
  /// The application options
  options: ApplicationOptions,
}

impl TaoEventLoop {
  /// Creates a new tao event loop
  pub fn new(options: ApplicationOptions) -> Self {
    let event_loop = EventLoop::new();
    Self {
      event_loop: Some(event_loop),
      options,
    }
  }

  /// Gets a reference to the tao event loop
  pub fn event_loop(&self) -> Option<&EventLoop<()>> {
    self.event_loop.as_ref()
  }

  /// Consumes the event loop and returns the tao instance
  pub fn take_event_loop(&mut self) -> Option<EventLoop<()>> {
    self.event_loop.take()
  }

  /// Runs the event loop in blocking mode (current implementation)
  ///
  /// This method blocks the current thread until the application terminates.
  /// This is the current implementation and has the problem of blocking the Node.js event loop.
  pub fn run_blocking(&mut self, event_handler: EventHandler, app_state: AppState) -> Result<()> {
    let ctrl = self.map_control_flow();

    if let Some(event_loop) = self.take_event_loop() {
      let _handler = event_handler.get_callback().clone();
      let _env = event_handler.env();
      let should_exit = app_state.clone_state();

      event_loop.run(move |event, _, control_flow| {
        *control_flow = ctrl;

        // Check if exit was requested
        if should_exit.should_exit() {
          event_handler.emit_application_close();
          *control_flow = ControlFlow::Exit;
          return;
        }

        // Handle window events
        if let Event::WindowEvent {
          event: WindowEvent::CloseRequested,
          ..
        } = event
        {
          event_handler.emit_window_close();
          *control_flow = ControlFlow::Exit;
        }
      });
    }

    Ok(())
  }

  /// Maps the control flow from JavaScript to tao
  fn map_control_flow(&self) -> ControlFlow {
    match self.options.control_flow {
      None => ControlFlow::Poll,
      Some(crate::types::JsControlFlow::Poll) => ControlFlow::Poll,
      Some(crate::types::JsControlFlow::WaitUntil) => {
        let wait_time = self.options.wait_time.unwrap_or(0);
        ControlFlow::WaitUntil(
          std::time::Instant::now() + std::time::Duration::from_millis(wait_time as u64),
        )
      }
      Some(crate::types::JsControlFlow::Exit) => ControlFlow::Exit,
      Some(crate::types::JsControlFlow::ExitWithCode) => {
        let exit_code = self.options.exit_code.unwrap_or(0);
        ControlFlow::ExitWithCode(exit_code)
      }
    }
  }

  /// Runs the event loop with a worker thread (future)
  ///
  /// This method is designed to allow the UI event loop to run
  /// on the main thread while a worker thread handles
  /// business logic without blocking Node.js.
  ///
  /// TODO: Implement this functionality
  #[allow(dead_code)]
  pub fn run_with_worker(
    &mut self,
    _event_handler: EventHandler,
    _app_state: AppState,
  ) -> Result<()> {
    // This implementation will require:
    // 1. Create a worker thread in Rust
    // 2. Use napi_threadsafe_function for communication
    // 3. Coordinate the UI event loop with the worker
    unimplemented!("run_with_worker is not yet implemented")
  }

  /// Runs the event loop in detached mode (future)
  ///
  /// This method allows the server to keep running after
  /// the window is closed.
  ///
  /// TODO: Implement this functionality
  #[allow(dead_code)]
  pub fn run_detached(
    &mut self,
    _event_handler: EventHandler,
    _app_state: AppState,
    _keep_server_alive: bool,
  ) -> Result<()> {
    // This implementation will require:
    // 1. Separate the event loop lifecycle from the server
    // 2. Allow the worker thread to keep running after closing the window
    unimplemented!("run_detached is not yet implemented")
  }
}

impl Default for TaoEventLoop {
  fn default() -> Self {
    Self::new(ApplicationOptions {
      control_flow: Some(crate::types::JsControlFlow::Poll),
      wait_time: None,
      exit_code: None,
    })
  }
}
