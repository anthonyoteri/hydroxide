use super::{fire_model_event, ModelMutateResultData};
use crate::ctx::Ctx;
use crate::prelude::*;
use crate::store::{Createable, Filterable, Patchable, UpdateType};
use std::sync::Arc;
use surrealdb::sql::Object;

pub async fn get<E>(ctx: Arc<Ctx>, _entity: &'static str, id: &str) -> Result<E>
where
    E: TryFrom<Object, Error = Error>,
{
    ctx.get_store().get(id).await?.try_into()
}

pub async fn create<D>(
    ctx: Arc<Ctx>,
    entity: &'static str,
    data: D,
) -> Result<ModelMutateResultData>
where
    D: Createable,
{
    log::trace!("model::create( entity: {:?}, data: {:?} )", entity, data);

    let id = ctx.get_store().create(entity, data).await?;
    let result_data = ModelMutateResultData::from(id);

    fire_model_event(&ctx, entity, "create", result_data.clone());

    Ok(result_data)
}

pub async fn update<D>(
    ctx: Arc<Ctx>,
    entity: &'static str,
    id: &str,
    data: D,
) -> Result<ModelMutateResultData>
where
    D: Patchable,
{
    log::trace!(
        "model::update( entity: {:?}, id: {:?}, data: {:?} )",
        entity,
        id,
        data
    );

    let id = ctx.get_store().merge(id, data, UpdateType::Replace).await?;

    let result_data = ModelMutateResultData::from(id);
    fire_model_event(&ctx, entity, "update", result_data.clone());

    Ok(result_data)
}

pub async fn merge<D>(
    ctx: Arc<Ctx>,
    entity: &'static str,
    id: &str,
    data: D,
) -> Result<ModelMutateResultData>
where
    D: Patchable,
{
    log::trace!(
        "model::merge( entity: {:?}, id: {:?}, data: {:?} )",
        entity,
        id,
        data
    );

    let id = ctx.get_store().merge(id, data, UpdateType::Merge).await?;

    let result_data = ModelMutateResultData::from(id);
    fire_model_event(&ctx, entity, "update", result_data.clone());

    Ok(result_data)
}

pub async fn delete(
    ctx: Arc<Ctx>,
    entity: &'static str,
    id: &str,
) -> Result<ModelMutateResultData> {
    log::trace!("model::delete( entity: {:?}, id: {:?} )", entity, id);

    let id = ctx.get_store().delete(id).await?;
    let result_data = ModelMutateResultData::from(id);

    fire_model_event(&ctx, entity, "delete", result_data.clone());

    Ok(result_data)
}

pub async fn list<E, F>(ctx: Arc<Ctx>, entity: &'static str, filter: Option<F>) -> Result<Vec<E>>
where
    E: TryFrom<Object, Error = Error>,
    F: Filterable + std::fmt::Debug,
{
    log::trace!("model::list( entity: {:?}, filter: {:?} )", entity, filter);

    let objects = ctx
        .get_store()
        .select(entity, filter.map(std::convert::Into::into))
        .await?;

    objects
        .into_iter()
        .map(std::convert::TryInto::try_into)
        .collect::<Result<_>>()
}
