use crate::prelude::*;
use crate::{ctx::Ctx, model::TimeRecordForImportWithContext};
use std::collections::BTreeMap;
use std::sync::Arc;
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

async fn _import_data(ctx: Arc<Ctx>, params: ImportParams) -> Result<()> {
    let mut tx = ctx.transaction(true).await?;

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
        if let Err(err) = TimeRecordBmc::delete(ctx.clone(), &r.id).await {
            log::error!("Error deleting record {}: {}", &r.id, &err);
            tx.cancel().await?;
            return Err(err);
        }
    }
    for p in existing_projects {
        if let Err(err) = ProjectBmc::delete(ctx.clone(), &p.id).await {
            log::error!("Error deleting project {}: {}", &p.id, &err);
            tx.cancel().await?;
            return Err(err);
        };
    }
    for c in existing_categories {
        if let Err(err) = CategoryBmc::delete(ctx.clone(), &c.id).await {
            log::error!("Error deleting category {}: {}", &c.id, &err);
            tx.cancel().await?;
            return Err(err);
        };
    }

    // -- Map old IDs from Hydra to new ID format.
    let mut category_id_map: BTreeMap<usize, String> = BTreeMap::new();
    let mut project_id_map: BTreeMap<usize, String> = BTreeMap::new();

    // -- Restore categories.
    for value in params.categories {
        let category_for_create = CategoryForCreate::from(value.clone());

        match CategoryBmc::create(ctx.clone(), category_for_create).await {
            Ok(response) => {
                category_id_map.insert(value.id, response.id);
            }
            Err(err) => {
                log::error!("Error creating category: {}", &err);
                tx.cancel().await?;
                return Err(err);
            }
        }
    }

    // -- Restore projects.
    for value in params.projects {
        let value_with_ctx = ProjectForImportWithContext {
            data: value.clone(),
            ctx: category_id_map.clone(),
        };

        match ProjectForCreate::try_from(value_with_ctx) {
            Ok(project_for_create) => {
                match ProjectBmc::create(ctx.clone(), project_for_create).await {
                    Ok(response) => {
                        project_id_map.insert(value.id, response.id);
                    }
                    Err(err) => {
                        log::error!("Error creating project: {}", &err);
                        tx.cancel().await?;
                        return Err(err);
                    }
                }
            }
            Err(err) => {
                log::error!("Unable to find category for project: {}", &err);
                tx.cancel().await?;
                return Err(err);
            }
        }
    }

    // -- Restore Time Records.
    for value in params.time_records {
        let value_with_ctx = TimeRecordForImportWithContext {
            data: value.clone(),
            ctx: project_id_map.clone(),
        };

        match TimeRecordForCreate::try_from(value_with_ctx) {
            Ok(record_for_create) => {
                if let Err(err) =
                    TimeRecordBmc::create(ctx.clone(), record_for_create.clone()).await
                {
                    log::error!(
                        "Error creating time record {:?}: {}",
                        &record_for_create,
                        &err
                    );
                    tx.cancel().await?;
                    return Err(err);
                }
            }
            Err(err) => {
                log::error!("Unable to find project for time record: {}", &err);
                tx.cancel().await?;
                return Err(err);
            }
        }
    }

    Ok(tx.commit().await?)
}

#[command]
pub async fn import_data(app: AppHandle<Wry>, params: ImportParams) -> IpcResponse<()> {
    log::trace!("import_data( params: {:?} )", params);

    match Ctx::from_app(app) {
        Ok(ctx) => _import_data(ctx, params).await.into(),
        Err(_) => Err(Error::CtxFail).into(),
    }
}
