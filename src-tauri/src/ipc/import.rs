use crate::prelude::*;
use crate::{ctx::Ctx, model::TimeRecordForImportWithContext};
use std::collections::BTreeMap;
use tauri::{command, AppHandle, Wry};

use serde::Deserialize;

use crate::model::{
    CategoryBmc, CategoryForCreate, CategoryForImport, ProjectBmc, ProjectForCreate,
    ProjectForImport, ProjectForImportWithContext, TimeRecordBmc, TimeRecordForCreate,
    TimeRecordForImport,
};

use super::IpcResponse;

#[derive(Deserialize, Debug)]
pub struct ImportParams {
    pub categories: Vec<CategoryForImport>,
    pub projects: Vec<ProjectForImport>,
    pub time_records: Vec<TimeRecordForImport>,
}

#[command]
pub async fn import_data(app: AppHandle<Wry>, params: ImportParams) -> IpcResponse<()> {
    log::trace!("import_data( params: {:?} )", params);

    match Ctx::from_app(app) {
        Ok(ctx) => {
            let existing_records = TimeRecordBmc::list(ctx.clone(), None)
                .await
                .unwrap_or(Vec::new());
            let existing_projects = ProjectBmc::list(ctx.clone(), None)
                .await
                .unwrap_or(Vec::new());
            let existing_categories = CategoryBmc::list(ctx.clone(), None)
                .await
                .unwrap_or(Vec::new());

            // -- Delete all existing records.
            for r in existing_records {
                if TimeRecordBmc::delete(ctx.clone(), &r.id).await.is_err() {
                    println!("Error deleting record {}", &r.id);
                };
            }
            for p in existing_projects {
                if ProjectBmc::delete(ctx.clone(), &p.id).await.is_err() {
                    println!("Error deleting project {}", &p.id);
                };
            }
            for c in existing_categories {
                if CategoryBmc::delete(ctx.clone(), &c.id).await.is_err() {
                    println!("Error deleting category {}", &c.id);
                };
            }

            // -- Map old IDs from Hydra to new ID format.
            let mut category_id_map: BTreeMap<usize, String> = BTreeMap::new();
            let mut project_id_map: BTreeMap<usize, String> = BTreeMap::new();

            // -- Restore categories.
            for value in params.categories {
                let category_for_create = CategoryForCreate::from(value.clone());
                if let Ok(response) = CategoryBmc::create(ctx.clone(), category_for_create).await {
                    category_id_map.insert(value.id, response.id);
                }
            }

            // -- Restore projects.
            for value in params.projects {
                let value_with_ctx = ProjectForImportWithContext {
                    data: value.clone(),
                    ctx: category_id_map.clone(),
                };

                if let Ok(project_for_create) = ProjectForCreate::try_from(value_with_ctx) {
                    if let Ok(response) = ProjectBmc::create(ctx.clone(), project_for_create).await
                    {
                        project_id_map.insert(value.id, response.id);
                    }
                }
            }

            // -- Restore Time Records.
            for value in params.time_records {
                let value_with_ctx = TimeRecordForImportWithContext {
                    data: value.clone(),
                    ctx: project_id_map.clone(),
                };

                if let Ok(record_for_create) = TimeRecordForCreate::try_from(value_with_ctx) {
                    if TimeRecordBmc::create(ctx.clone(), record_for_create.clone())
                        .await
                        .is_err()
                    {
                        println!("Error creating time record {:?}", &record_for_create);
                    }
                }
            }

            Ok(()).into()
        }
        Err(_) => Err(Error::CtxFail).into(),
    }
}
