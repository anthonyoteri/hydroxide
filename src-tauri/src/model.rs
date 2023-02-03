//! Model module and sub-modules contain all of the model types and
//! backend model controllers for the application.
//!
//! The application code call the model controllers, and the model controller
//! calls the store and fire model events as appropriate.
//!

use crate::ctx::Ctx;
use crate::event::HubEvent;
use serde::Serialize;
use ts_rs::TS;

mod base;
mod category;
mod project;
mod time_record;

// - Re-exports
pub use category::{
    Category, CategoryBmc, CategoryFilter, CategoryForCreate, CategoryForImport, CategoryForUpdate,
};
pub use project::{
    Project, ProjectBmc, ProjectFilter, ProjectForCreate, ProjectForImport,
    ProjectForImportWithContext, ProjectForUpdate,
};
pub use time_record::{
    TimeRecord, TimeRecordBmc, TimeRecordFilter, TimeRecordForCreate, TimeRecordForImport,
    TimeRecordForImportWithContext, TimeRecordForUpdate,
};

fn fire_model_event<D>(ctx: &Ctx, entity: &str, action: &str, data: D)
where
    D: Serialize + Clone,
{
    ctx.emit_hub_event(HubEvent {
        hub: "Model".to_string(),
        topic: entity.to_string(),
        label: Some(action.to_string()),
        data: Some(data),
    })
}

#[derive(Serialize, TS, Clone, Debug)]
#[ts(export, export_to = "../src-frontend/src/bindings/")]
pub struct ModelMutateResultData {
    pub id: String,
}

impl From<String> for ModelMutateResultData {
    fn from(id: String) -> Self {
        Self { id }
    }
}
