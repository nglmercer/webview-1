#![deny(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! Webview N-API Bindings
//!
//! This library provides N-API bindings for using tao and wry
//! in Node.js applications. All methods, APIs, enums, and types are exported
//! directly for Node.js composition.

// Wry bindings
pub mod wry;

// Tao bindings
pub mod tao;
