//! Tao type aliases
//!
//! This module contains all type aliases from the tao crate.

use napi::Result as NapiResult;

/// Result type for tao operations.
pub type Result<T> = NapiResult<T>;

/// Unique identifier for a window.
pub type WindowId = u64;

/// Device identifier.
pub type DeviceId = u32;
