//! `ipc` module and sub-modules are all Rust constructs necessary for the WebView to Rust Tauri IPC calls.
//!
//! At a high level it follows the "JSON-RPC 2.0" format
//!   - method_name - Will be the Tauri command function name
//!   - params - Tuari commands will have one params argument by design, called params (and state arguments)
//!   - response - Will be an IpcResponse with the JSON-RPC 2.0 result/error format back.
//!
//! The benefits of following the JSON-RPC 2.0 style is that it is simple, clean and allows to write the frontend to a
//! JSON-RPC 2.0 cloud backend easily.
//!
//! Notes:
//!   - This module re-exports the appropriate sub-module cnstructs as their heirarchy is irrelevant to callers.
//!

mod params;
mod project;
mod response;

// -- Re-exports
pub use params::*;
pub use project::*;
pub use response::*;
