//! Wry traits
//!
//! This module contains all traits from the wry crate.

use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::wry::structs::WebView;

/// Extension trait for WebView on Unix platforms.
#[napi]
impl WebView {
  /// Gets the GTK widget for the webview (Unix only).
  #[napi]
  pub fn gtk_widget(&self) -> Result<u64> {
    #[cfg(any(
      target_os = "linux",
      target_os = "dragonfly",
      target_os = "freebsd",
      target_os = "netbsd",
      target_os = "openbsd"
    ))]
    {
      use wry::WebViewExtUnix;
      let widget = self.inner.lock().unwrap().gtk_widget();
      Ok(widget as u64)
    }

    #[cfg(not(any(
      target_os = "linux",
      target_os = "dragonfly",
      target_os = "freebsd",
      target_os = "netbsd",
      target_os = "openbsd"
    )))]
    {
      Err(napi::Error::new(
        napi::Status::GenericFailure,
        "Unix-specific method not available on this platform".to_string(),
      ))
    }
  }
}
