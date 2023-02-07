//! All model and controller for the Project type
//!

use super::{base, ModelMutateResultData};
use crate::ctx::Ctx;
use crate::prelude::*;
use crate::store::{Createable, Filterable, Patchable};
use crate::utils::{map, XTakeVal};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;
use std::collections::BTreeMap;
use std::sync::Arc;
use surrealdb::sql::{Object, Value};
use ts_rs::TS;

#[derive(Serialize, TS, Debug, Clone)]
#[ts(export, export_to = "../src-frontend/src/bindings/")]
pub struct Category {
    pub id: String,
    pub name: String,
    pub description: String,
    pub ctime: String,
}

impl TryFrom<Object> for Category {
    type Error = Error;

    fn try_from(mut val: Object) -> Result<Self> {
        Ok(Self {
            id: val.x_take_val("id")?,
            name: val.x_take_val("name")?,
            description: val.x_take_val("description")?,
            ctime: val.x_take_val::<i64>("ctime")?.to_string(),
        })
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CategoryForCreate {
    pub name: String,
    pub description: Option<String>,
}

impl From<CategoryForCreate> for Value {
    fn from(val: CategoryForCreate) -> Self {
        let mut data = BTreeMap::new();
        data.insert("name".into(), val.name.into());
        data.insert("description".into(), val.description.into());
        data.into()
    }
}

impl Createable for CategoryForCreate {}

#[skip_serializing_none]
#[derive(Deserialize, Debug, Clone)]
pub struct CategoryForUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl From<CategoryForUpdate> for Value {
    fn from(val: CategoryForUpdate) -> Self {
        let mut data = BTreeMap::new();
        if let Some(name) = val.name {
            data.insert("name".into(), name.into());
        }
        if let Some(description) = val.description {
            data.insert("description".into(), description.into());
        }
        data.into()
    }
}

impl Patchable for CategoryForUpdate {}

#[derive(Deserialize, Debug, Clone)]
pub struct CategoryFilter {
    name: Option<String>,
}

impl From<CategoryFilter> for Value {
    fn from(val: CategoryFilter) -> Self {
        Value::Object(
            map![
                "name".into() => val.name.into(),
            ]
            .into(),
        )
    }
}

impl Filterable for CategoryFilter {}

#[derive(Deserialize, TS, Debug, Clone)]
#[ts(export, export_to = "../src-frontend/src/bindings/")]
pub struct CategoryForImport {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub num_records: usize,

    #[ts(type = "Date")]
    pub created: DateTime<Utc>,

    #[ts(type = "Date")]
    pub updated: DateTime<Utc>,
    pub user: String,
}

impl From<CategoryForImport> for CategoryForCreate {
    fn from(val: CategoryForImport) -> Self {
        Self {
            name: val.name,
            description: Some(val.description),
        }
    }
}

pub struct CategoryBmc;

impl CategoryBmc {
    const ENTITY: &'static str = "category";

    pub async fn get(ctx: Arc<Ctx>, id: &str) -> Result<Category> {
        base::get(ctx, Self::ENTITY, id).await
    }

    pub async fn create(ctx: Arc<Ctx>, data: CategoryForCreate) -> Result<ModelMutateResultData> {
        base::create(ctx, Self::ENTITY, data).await
    }

    pub async fn update(
        ctx: Arc<Ctx>,
        id: &str,
        data: CategoryForUpdate,
    ) -> Result<ModelMutateResultData> {
        base::update(ctx, Self::ENTITY, id, data).await
    }

    pub async fn delete(ctx: Arc<Ctx>, id: &str) -> Result<ModelMutateResultData> {
        base::delete(ctx, Self::ENTITY, id).await
    }

    pub async fn list(ctx: Arc<Ctx>, filter: Option<CategoryFilter>) -> Result<Vec<Category>> {
        base::list(ctx, Self::ENTITY, filter).await
    }
}
