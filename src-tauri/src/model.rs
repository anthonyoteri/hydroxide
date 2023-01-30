//! Model module and sub-modules contain all of the model types and
//! backend model controllers for the application.
//!
//! The application code call the model controllers, and the model controller
//! calls the store and fire model events as appropriate.
//!

use crate::ctx::Ctx;
use crate::event::HubEvent;
use serde::Serialize;

mod base;
mod project;

// - Re-exports
pub use project::*;

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

#[derive(Serialize, Clone)]
pub struct ModelMutateResultData {
    pub id: String,
}

impl From<String> for ModelMutateResultData {
    fn from(id: String) -> Self {
        Self { id }
    }
}
