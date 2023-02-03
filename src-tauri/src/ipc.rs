//! `ipc` module and sub-modules are all Rust constructs necessary for the `WebView` to Rust Tauri IPC calls.
//!
//! At a high level it follows the "JSON-RPC 2.0" format
//!   - `method_name` - Will be the Tauri command function name
//!   - params - Tuari commands will have one params argument by design, called params (and state arguments)
//!   - response - Will be an `IpcResponse` with the JSON-RPC 2.0 result/error format back.
//!
//! The benefits of following the JSON-RPC 2.0 style is that it is simple, clean and allows to write the frontend to a
//! JSON-RPC 2.0 cloud backend easily.
//!
//! Notes:
//!   - This module re-exports the appropriate sub-module cnstructs as their heirarchy is irrelevant to callers.
//!

mod category;
mod import;
mod params;
mod project;
mod response;
mod time_record;

// -- Re-exports
pub use category::{create_category, update_category, get_category, list_categories, delete_category};
pub use params::{CreateParams, UpdateParams, GetParams, ListParams, DeleteParams};
pub use project::{create_project, update_project, get_project, list_projects, delete_project};
pub use response::*;
pub use time_record::{create_time_record, update_time_record, get_time_record, list_time_records, delete_time_record};
pub use import::import_data;

