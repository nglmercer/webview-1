//! Wry functions
//!
//! This module contains all functions from the wry crate.

use napi::Result;
use napi_derive::napi;

/// Returns the version of the webview library.
#[napi]
pub fn webview_version() -> Result<(u32, u32, u32)> {
  let version = wry::webview_version();
  let (major, minor, patch) = version.split_at(2);
  Ok((
    major.parse().unwrap_or(0),
    minor.parse().unwrap_or(0),
    patch.parse().unwrap_or(0),
  ))
}
