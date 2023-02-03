//! All model and controller for the `TiemRecord` type
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
pub struct TimeRecord {
    pub id: String,
    pub project: String,

    #[ts(type = "Date")]
    pub start_time: DateTime<Utc>,

    #[ts(type = "Date")]
    pub stop_time: Option<DateTime<Utc>>,

    pub approved: bool,
}

impl TryFrom<Object> for TimeRecord {
    type Error = Error;

    fn try_from(mut val: Object) -> Result<Self> {
        Ok(Self {
            id: val.x_take_val("id")?,
            project: val.x_take_val("project")?,
            start_time: val.x_take_val("start_time")?,
            stop_time: val.x_take("stop_time")?,
            approved: val.x_take_val("approved")?,
        })
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct TimeRecordForCreate {
    pub project: String,
    pub start_time: DateTime<Utc>,
    pub stop_time: Option<DateTime<Utc>>,
    pub approved: Option<bool>,
}

impl From<TimeRecordForCreate> for Value {
    fn from(val: TimeRecordForCreate) -> Self {
        let mut data = BTreeMap::new();
        data.insert("project".into(), val.project.into());
        data.insert("start_time".into(), val.start_time.into());

        if let Some(stop_time) = val.stop_time {
            data.insert("stop_time".into(), stop_time.into());
        }

        if let Some(approved) = val.approved {
            data.insert("approved".into(), approved.into());
        } else {
            let approved = val.start_time <= chrono::Utc::now();
            data.insert("approved".into(), approved.into());
        }

        if let Some(approved) = val.approved {
            data.insert("approved".into(), approved.into());
        }

        data.into()
    }
}

impl Createable for TimeRecordForCreate {}

#[skip_serializing_none]
#[derive(Deserialize, Debug, Clone)]
pub struct TimeRecordForUpdate {
    pub project: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub stop_time: Option<DateTime<Utc>>,
    pub approved: Option<bool>,
}

impl From<TimeRecordForUpdate> for Value {
    fn from(val: TimeRecordForUpdate) -> Self {
        let mut data = BTreeMap::new();

        if let Some(project) = val.project {
            data.insert("project".into(), project.into());
        }

        if let Some(start_time) = val.start_time {
            data.insert("start_time".into(), start_time.into());
        }

        if let Some(stop_time) = val.stop_time {
            data.insert("stop_time".into(), stop_time.into());
        }

        if let Some(approved) = val.approved {
            data.insert("approved".into(), approved.into());
        }

        data.into()
    }
}

impl Patchable for TimeRecordForUpdate {}

#[derive(Deserialize, TS, Debug, Clone)]
#[ts(export, export_to = "../src-frontend/src/bindings/")]
pub struct TimeRecordForImport {
    pub id: usize,
    pub project: usize,
    #[ts(type = "Date")]
    pub start_time: DateTime<Utc>,
    #[ts(type = "Date")]
    pub stop_time: Option<DateTime<Utc>>,
    pub total_seconds: u64,
    pub approved: bool,
}

pub struct TimeRecordForImportWithContext {
    pub data: TimeRecordForImport,
    pub ctx: BTreeMap<usize, String>,
}

impl TryFrom<TimeRecordForImportWithContext> for TimeRecordForCreate {
    type Error = Error;

    fn try_from(val: TimeRecordForImportWithContext) -> Result<Self> {
        if let Some(project) = val.ctx.get(&val.data.project) {
            return Ok(Self {
                project: project.clone(),
                start_time: val.data.start_time,
                stop_time: val.data.stop_time,
                approved: Some(val.data.approved),
            });
        }
        Err(Error::InvalidState(format!(
            "No such project with reference ID {}",
            val.data.project
        )))
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct TimeRecordFilter {
    project: Option<String>,
}

impl From<TimeRecordFilter> for Value {
    fn from(val: TimeRecordFilter) -> Self {
        Value::Object(
            map![
                "project".into() => val.project.into(),
            ]
            .into(),
        )
    }
}

impl Filterable for TimeRecordFilter {}

pub struct TimeRecordBmc;

impl TimeRecordBmc {
    const ENTITY: &'static str = "time_record";

    pub async fn get(ctx: Arc<Ctx>, id: &str) -> Result<TimeRecord> {
        base::get(ctx, Self::ENTITY, id).await
    }

    pub async fn create(ctx: Arc<Ctx>, data: TimeRecordForCreate) -> Result<ModelMutateResultData> {
        base::create(ctx, Self::ENTITY, data).await
    }

    pub async fn update(
        ctx: Arc<Ctx>,
        id: &str,
        data: TimeRecordForUpdate,
    ) -> Result<ModelMutateResultData> {
        base::update(ctx, Self::ENTITY, id, data).await
    }

    pub async fn delete(ctx: Arc<Ctx>, id: &str) -> Result<ModelMutateResultData> {
        base::delete(ctx, Self::ENTITY, id).await
    }

    pub async fn list(ctx: Arc<Ctx>, filter: Option<TimeRecordFilter>) -> Result<Vec<TimeRecord>> {
        base::list(ctx, Self::ENTITY, filter).await
    }
}
