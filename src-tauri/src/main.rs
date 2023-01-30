#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::ipc::{create_project, delete_project, get_project, list_projects, update_project};

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

    // -- For DEV ONLY
    seed_store(store.clone()).await?;

    tauri::Builder::default()
        .manage(store)
        .invoke_handler(tauri::generate_handler![
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

/// Only use while developing.
async fn seed_store(store: Arc<Store>) -> Result<()> {
    let ps = ["A", "B"].into_iter().map(|k| {
        (
            k,
            ProjectForCreate {
                name: f!("Project {k}"),
                description: f!("Some description of project {k}"),
            },
        )
    });

    for (k, project) in ps {
        let _project_id = store.create::<ProjectForCreate>("project", project).await?;
    }

    Ok(())
}
