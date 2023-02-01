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

// - Re-exports
pub use category::{Category, CategoryForCreate, CategoryForUpdate, CategoryFilter, CategoryBmc};
pub use project::{Project, ProjectForCreate, ProjectForUpdate, ProjectFilter, ProjectBmc};

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

#[derive(Serialize, TS, Clone)]
#[ts(export, export_to = "../src-frontend/src/bindings/")]
pub struct ModelMutateResultData {
    pub id: String,
}

impl From<String> for ModelMutateResultData {
    fn from(id: String) -> Self {
        Self { id }
    }
}
