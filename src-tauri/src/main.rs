#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(clippy::module_name_repetitions)]

use crate::ipc::{
    create_category, create_project, create_time_record, delete_category, delete_project,
    delete_time_record, get_category, get_project, get_time_record, import_data, list_categories,
    list_projects, list_time_records, update_category, update_project, update_time_record,
};

use crate::prelude::*;

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
    // -- Determine if we are in debug or release mode
    let is_release = !cfg!(debug_assertions);

    env_logger::init();

    if !is_release {
        log::info!("****************************************");
        log::info!("* Hydroxide is currently in DEBUG Mode *");
        log::info!("****************************************");
    }

    let data_dir = tauri::api::path::local_data_dir().unwrap();
    let db_file = if is_release {
        format!("file://{}/hydroxide/hydra.db", data_dir.display())
    } else {
        format!("file://{}/hydroxide/hydra-devel.db", data_dir.display())
    };

    log::info!("Using db_file {db_file}");

    let store = Store::new(&db_file, "hydroxide", "hydroxide").await?;
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
            // Time Record
            get_time_record,
            create_time_record,
            update_time_record,
            delete_time_record,
            list_time_records,
            // Import from Hydra
            import_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
