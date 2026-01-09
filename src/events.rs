//! Application event handling
//!
//! This module provides functionality to process and handle
//! events from the tao event loop and communicate them to JavaScript.

use crate::types::{ApplicationEvent, WebviewApplicationEvent};
use napi::bindgen_prelude::FunctionRef;
use napi::Env;
use std::cell::RefCell;
use std::rc::Rc;

/// Application event handler
#[derive(Clone)]
pub struct EventHandler {
  /// The JavaScript callback for events
  callback: Rc<RefCell<Option<FunctionRef<ApplicationEvent, ()>>>>,
  /// The N-API environment
  env: Env,
}

impl EventHandler {
  /// Creates a new event handler
  pub fn new(env: Env) -> Self {
    Self {
      callback: Rc::new(RefCell::new(None)),
      env,
    }
  }

  /// Sets the JavaScript callback
  pub fn set_callback(&self, handler: Option<FunctionRef<ApplicationEvent, ()>>) {
    *self.callback.borrow_mut() = handler;
  }

  /// Gets a reference to the callback
  pub fn get_callback(&self) -> &Rc<RefCell<Option<FunctionRef<ApplicationEvent, ()>>>> {
    &self.callback
  }

  /// Gets the N-API environment
  pub fn env(&self) -> Env {
    self.env
  }

  /// Emits an application event
  fn emit_event(&self, event: WebviewApplicationEvent) {
    let callback = self.callback.borrow();
    if let Some(callback) = callback.as_ref() {
      if let Ok(on_event) = callback.borrow_back(&self.env) {
        let _ = on_event.call(ApplicationEvent { event });
      }
    }
  }

  /// Emits a window close event
  pub fn emit_window_close(&self) {
    self.emit_event(WebviewApplicationEvent::WindowCloseRequested);
  }

  /// Emits an application close event
  pub fn emit_application_close(&self) {
    self.emit_event(WebviewApplicationEvent::ApplicationCloseRequested);
  }
}

/// Application execution state
#[derive(Clone)]
pub struct AppState {
  /// Indicates whether the application should exit
  should_exit: Rc<RefCell<bool>>,
}

impl AppState {
  /// Creates a new application state
  pub fn new() -> Self {
    Self {
      should_exit: Rc::new(RefCell::new(false)),
    }
  }

  /// Requests the application to exit
  pub fn request_exit(&self) {
    *self.should_exit.borrow_mut() = true;
  }

  /// Checks if exit has been requested
  pub fn should_exit(&self) -> bool {
    *self.should_exit.borrow()
  }

  /// Clones the state to share between threads
  pub fn clone_state(&self) -> Self {
    Self {
      should_exit: self.should_exit.clone(),
    }
  }
}

impl Default for AppState {
  fn default() -> Self {
    Self::new()
  }
}
