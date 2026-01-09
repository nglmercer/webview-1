//! Utility functions for the webview library
//!
//! This module contains helper functions and utilities used across
//! different modules to reduce code duplication.

use crate::types::{Dimensions, JsVideoMode, Monitor, Position, Theme};
use napi::{Error, Status};
use tao::window::Theme as TaoTheme;
use wry::Theme as WryTheme;

/// Converts a tao::monitor::MonitorHandle to our custom Monitor type
pub fn convert_monitor(monitor: tao::monitor::MonitorHandle) -> Monitor {
  Monitor {
    name: monitor.name(),
    scale_factor: monitor.scale_factor(),
    size: Dimensions {
      width: monitor.size().width,
      height: monitor.size().height,
    },
    position: Position {
      x: monitor.position().x,
      y: monitor.position().y,
    },
    video_modes: convert_video_modes(monitor.video_modes()),
  }
}

/// Converts tao::monitor::VideoMode iterator to our custom JsVideoMode type
pub fn convert_video_modes(
  modes: impl Iterator<Item = tao::monitor::VideoMode>,
) -> Vec<JsVideoMode> {
  modes
    .map(|v| JsVideoMode {
      size: Dimensions {
        width: v.size().width,
        height: v.size().height,
      },
      bit_depth: v.bit_depth(),
      refresh_rate: v.refresh_rate(),
    })
    .collect()
}

/// Creates an error with a format string and context
pub fn error_with_context(operation: &str, details: &str) -> Error {
  Error::new(
    Status::GenericFailure,
    format!("Failed to {}: {}", operation, details),
  )
}

/// Converts our custom Theme to tao::window::Theme
pub fn theme_to_tao(theme: Theme) -> Option<TaoTheme> {
  match theme {
    Theme::Light => Some(TaoTheme::Light),
    Theme::Dark => Some(TaoTheme::Dark),
    _ => None,
  }
}

/// Converts our custom Theme to wry::Theme
pub fn theme_to_wry(theme: Theme) -> WryTheme {
  match theme {
    Theme::Light => WryTheme::Light,
    Theme::Dark => WryTheme::Dark,
    _ => WryTheme::Auto,
  }
}

/// Converts tao::window::Theme to our custom Theme
pub fn tao_to_theme(theme: TaoTheme) -> Theme {
  match theme {
    TaoTheme::Light => Theme::Light,
    TaoTheme::Dark => Theme::Dark,
    _ => Theme::System,
  }
}
