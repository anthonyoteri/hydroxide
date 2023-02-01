#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::ipc::{
    create_category, create_project, delete_category, delete_project, get_category, get_project,
    list_categories, list_projects, update_category, update_project,
};

use crate::prelude::*;

use model::ProjectForCreate;
use std::sync::Arc;
use store::Store;
mod ctx;
mod error;
mod event;
mod ipc;
mod model;
mod prelude;
mod store;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let store = Store::new().await?;
    let store = Arc::new(store);

    tauri::Builder::default()
        .manage(store)
        .invoke_handler(tauri::generate_handler![
            // Category
            get_category,
            create_category,
            update_category,
            delete_category,
            list_categories,
            // Project
            get_project,
            create_project,
            update_project,
            delete_project,
            list_projects,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
