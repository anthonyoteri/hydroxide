//! All model and controller for the Project type
//!

use super::{base, ModelMutateResultData};
use crate::ctx::Ctx;
use crate::prelude::*;
use crate::store::{Createable, Filterable, Patchable};
use crate::utils::{map, XTakeVal};

use serde::{Deserialize, Serialize};
use serde_with_macros::skip_serializing_none;
use std::collections::BTreeMap;
use std::sync::Arc;
use surrealdb::sql::{Object, Value};

#[derive(Serialize, Debug, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub ctime: String,
}

impl TryFrom<Object> for Project {
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
pub struct ProjectForCreate {
    pub name: String,
    pub description: String,
}

impl From<ProjectForCreate> for Value {
    fn from(val: ProjectForCreate) -> Self {
        let mut data = BTreeMap::new();
        data.insert("name".into(), val.name.into());
        data.insert("description".into(), val.description.into());
        data.into()
    }
}

impl Createable for ProjectForCreate {}

#[skip_serializing_none]
#[derive(Deserialize, Debug, Clone)]
pub struct ProjectForUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl From<ProjectForUpdate> for Value {
    fn from(val: ProjectForUpdate) -> Self {
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

impl Patchable for ProjectForUpdate {}

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
        base::create(ctx, Self::ENTITY, data).await
    }

    pub async fn update(
        ctx: Arc<Ctx>,
        id: &str,
        data: ProjectForUpdate,
    ) -> Result<ModelMutateResultData> {
        base::update(ctx, Self::ENTITY, id, data).await
    }

    pub async fn delete(ctx: Arc<Ctx>, id: &str) -> Result<ModelMutateResultData> {
        base::delete(ctx, Self::ENTITY, id).await
    }

    pub async fn list(ctx: Arc<Ctx>, filter: Option<ProjectFilter>) -> Result<Vec<Project>> {
        base::list(ctx, Self::ENTITY, filter).await
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::store::Store;

    /// Validate that we can create and retrieve a `Project` from the `Store`
    #[tokio::test]
    async fn test_create_and_fetch_project() -> Result<()> {
        let store = Store::new().await?;

        let pc = ProjectForCreate {
            name: "project 1".into(),
            description: "description of project 1".into(),
        };

        let id = store
            .create::<ProjectForCreate>("project", pc.clone())
            .await?;

        let returned: Project = store.get(&id).await?.try_into()?;

        assert_eq!(returned.id, id);
        assert_eq!(returned.name, pc.name);
        assert_eq!(returned.description, pc.description);

        Ok(())
    }

    /// Validate that we can create and then delete a `Project` from the
    /// `Store`
    #[tokio::test]
    async fn test_create_and_delete_project() -> Result<()> {
        let store = Store::new().await?;

        let pc = ProjectForCreate {
            name: "project 1".into(),
            description: "description of project 1".into(),
        };

        let id = store
            .create::<ProjectForCreate>("project", pc.clone())
            .await?;

        let deleted = store.delete(&id).await?;

        assert_eq!(deleted, id);
        assert!(store.get(&id).await.is_err());

        Ok(())
    }

    /// Verify that we can create and update a `Project` from the
    /// `Store`.  The update will replace all fields and then validate
    /// that all fields are changed.
    #[tokio::test]
    async fn test_create_and_update_project() -> Result<()> {
        let store = Store::new().await?;

        let pc = ProjectForCreate {
            name: "project 1".into(),
            description: "description of project 1".into(),
        };

        let id = store
            .create::<ProjectForCreate>("project", pc.clone())
            .await?;

        let pu = ProjectForUpdate {
            name: Some(pc.name.to_uppercase()),
            description: Some(pc.description.to_uppercase()),
        };
        let id = store.merge::<ProjectForUpdate>(&id, pu.clone()).await?;

        let returned: Project = store.get(&id).await?.try_into()?;

        assert_eq!(returned.id, id);
        assert_eq!(returned.name, pu.name.unwrap());
        assert_eq!(returned.description, pu.description.unwrap());

        Ok(())
    }

    /// Validate that we can create and then patch (Partial Update) a
    /// `Project` in the `Store`.  In this test, only the `name` field
    /// should be updated, the remaining fields should remain at their
    /// original values.
    #[tokio::test]
    async fn test_create_and_partial_update_project() -> Result<()> {
        let store = Store::new().await?;

        let pc = ProjectForCreate {
            name: "project 1".into(),
            description: "description of project 1".into(),
        };

        let id = store
            .create::<ProjectForCreate>("project", pc.clone())
            .await?;

        let pu = ProjectForUpdate {
            name: Some(pc.name.to_uppercase()),
            description: None,
        };
        let id = store.merge::<ProjectForUpdate>(&id, pu.clone()).await?;

        let returned: Project = store.get(&id).await?.try_into()?;

        assert_eq!(returned.id, id);
        assert_eq!(returned.name, pu.name.unwrap());
        assert_eq!(returned.description, pc.description);

        Ok(())
    }

    /// Test that listing `Projects` with and without a `ProjectFilter`
    /// works.  In the first part of the test, we validate that we can
    /// apply a filter, and we get back the desired element(s), in the
    /// second part, we simply verify that when no filter is passed, all
    /// elements are returned.
    #[tokio::test]
    async fn test_create_list_and_filter_projects() -> Result<()> {
        let store = Store::new().await?;

        let ps = (0..20).into_iter().map(|k| ProjectForCreate {
            name: f!("Project {k}"),
            description: f!("Some description of project {k}"),
        });

        for project in ps {
            store
                .create::<ProjectForCreate>("project", project.clone())
                .await?;
        }

        let filter = ProjectFilter {
            name: Some("Project 3".to_string()),
        };
        let results = store.select("project", Some(filter.into())).await?;

        assert_eq!(results.len(), 1);

        let projects: Vec<Project> = results
            .into_iter()
            .map(|o| o.try_into())
            .collect::<Result<_>>()?;

        println!("Results are {:?}", projects);

        assert_eq!(projects.first().unwrap().name, "Project 3");

        let results = store.select("project", None).await?;
        assert_eq!(results.len(), 20);

        Ok(())
    }
}
