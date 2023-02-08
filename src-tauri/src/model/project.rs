//! All model and controller for the Project type
//!

use super::{base, ModelMutateResultData};
use crate::ctx::Ctx;
use crate::prelude::*;
use crate::store::{Createable, Filterable, Patchable};
use crate::utils::{map, XTake, XTakeVal};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;
use std::collections::BTreeMap;
use std::sync::Arc;
use surrealdb::sql::{Object, Value};
use ts_rs::TS;

#[derive(Serialize, TS, Debug, Clone)]
#[ts(export, export_to = "../src-frontend/src/bindings/")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: Option<String>,
    pub ctime: String,
}

impl TryFrom<Object> for Project {
    type Error = Error;

    fn try_from(mut val: Object) -> Result<Self> {
        Ok(Self {
            id: val.x_take_val("id")?,
            name: val.x_take_val("name")?,
            category: val.x_take_val("category")?,
            description: val.x_take("description")?,
            ctime: val.x_take_val::<i64>("ctime")?.to_string(),
        })
    }
}

#[derive(Deserialize, TS, Debug, Clone)]
#[ts(export, export_to = "../src-frontend/src/bindings/")]
pub struct ProjectForCreate {
    pub name: String,
    pub category: String,
    pub description: Option<String>,
}

impl From<ProjectForCreate> for Value {
    fn from(val: ProjectForCreate) -> Self {
        let mut data = BTreeMap::new();
        data.insert("name".into(), val.name.into());
        data.insert("category".into(), val.category.into());

        if let Some(desc) = val.description {
            data.insert("description".into(), desc.into());
        }
        data.into()
    }
}

impl Createable for ProjectForCreate {}

#[skip_serializing_none]
#[derive(Deserialize, TS, Debug, Clone)]
#[ts(export, export_to = "../src-frontend/src/bindings/")]
pub struct ProjectForUpdate {
    pub name: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
}

impl From<ProjectForUpdate> for Value {
    fn from(val: ProjectForUpdate) -> Self {
        let mut data = BTreeMap::new();
        if let Some(name) = val.name {
            data.insert("name".into(), name.into());
        }
        if let Some(category) = val.category {
            data.insert("category".into(), category.into());
        }
        if let Some(description) = val.description {
            data.insert("description".into(), description.into());
        }
        data.into()
    }
}

impl Patchable for ProjectForUpdate {}

#[derive(Deserialize, Debug, Clone)]
pub struct ProjectForImport {
    pub id: usize,
    pub category: usize,
    pub name: String,
    pub description: String,
    pub num_records: usize,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

pub struct ProjectForImportWithContext {
    pub data: ProjectForImport,
    pub ctx: BTreeMap<usize, String>,
}

impl TryFrom<ProjectForImportWithContext> for ProjectForCreate {
    type Error = Error;

    fn try_from(val: ProjectForImportWithContext) -> Result<Self> {
        if let Some(category) = val.ctx.get(&val.data.category) {
            return Ok(Self {
                name: val.data.name,
                category: category.clone(),
                description: Some(val.data.description.clone()),
            });
        }
        Err(Error::InvalidState(format!(
            "No such category with external reference ID {}",
            val.data.category
        )))
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProjectFilter {
    name: Option<String>,
}

impl From<ProjectFilter> for Value {
    fn from(val: ProjectFilter) -> Self {
        Value::Object(
            map![
                "name".into() => val.name.into(),
            ]
            .into(),
        )
    }
}

impl Filterable for ProjectFilter {}

pub struct ProjectBmc;

impl ProjectBmc {
    const ENTITY: &'static str = "project";

    pub async fn get(ctx: Arc<Ctx>, id: &str) -> Result<Project> {
        base::get(ctx, Self::ENTITY, id).await
    }

    pub async fn create(ctx: Arc<Ctx>, data: ProjectForCreate) -> Result<ModelMutateResultData> {
        log::info!("Creating project {}", &data.name);

        base::create(ctx, Self::ENTITY, data).await
    }

    pub async fn update(
        ctx: Arc<Ctx>,
        id: &str,
        data: ProjectForUpdate,
    ) -> Result<ModelMutateResultData> {
        if let Some(name) = &data.name {
            log::info!("Updating project with name {}", name);
        }
        base::merge(ctx, Self::ENTITY, id, data).await
    }

    pub async fn delete(ctx: Arc<Ctx>, id: &str) -> Result<ModelMutateResultData> {
        log::info!("Deleting project with id {}", id);

        base::delete(ctx, Self::ENTITY, id).await
    }

    pub async fn list(ctx: Arc<Ctx>, filter: Option<ProjectFilter>) -> Result<Vec<Project>> {
        base::list(ctx, Self::ENTITY, filter).await
    }
}
