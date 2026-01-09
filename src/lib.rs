#![deny(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! Webview N-API Bindings
//!
//! This library provides N-API bindings for using tao and wry
//! in Node.js applications.

use napi::Result;
use napi_derive::napi;

// Application modules
pub mod application;
pub mod browser_window;
pub mod event_loop;
pub mod events;
pub mod types;
pub mod webview;

// Private modules
mod utils;

/// Returns the webview version
#[napi]
pub fn get_webview_version() -> Result<String> {
  wry::webview_version().map_err(|e| {
    napi::Error::new(
      napi::Status::GenericFailure,
      format!("Failed to get webview version: {}", e),
    )
  })
}
