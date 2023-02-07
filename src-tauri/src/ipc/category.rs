//! Tauri IPC command s to bridge Project Frontend Model Controller to Backend Model Controller
//!

use super::{CreateParams, DeleteParams, GetParams, IpcResponse, ListParams, UpdateParams};
use crate::ctx::Ctx;
use crate::model::{
    Category, CategoryBmc, CategoryFilter, CategoryForCreate, CategoryForUpdate,
    ModelMutateResultData,
};
use crate::prelude::*;
use tauri::{command, AppHandle, Wry};

#[command]
pub async fn get_category(app: AppHandle<Wry>, params: GetParams) -> IpcResponse<Category> {
    log::trace!("get_category( params: {:?} )", params);

    match Ctx::from_app(app) {
        Ok(ctx) => CategoryBmc::get(ctx, &params.id).await.into(),
        Err(_) => Err(Error::CtxFail).into(),
    }
}

#[command]
pub async fn create_category(
    app: AppHandle<Wry>,
    params: CreateParams<CategoryForCreate>,
) -> IpcResponse<ModelMutateResultData> {
    log::trace!("create_category( params: {:?} )", params);

    match Ctx::from_app(app) {
        Ok(ctx) => CategoryBmc::create(ctx, params.data).await.into(),
        Err(_) => Err(Error::CtxFail).into(),
    }
}

#[command]
pub async fn update_category(
    app: AppHandle<Wry>,
    params: UpdateParams<CategoryForUpdate>,
) -> IpcResponse<ModelMutateResultData> {
    log::trace!("update_category( params: {:?} )", params);

    match Ctx::from_app(app) {
        Ok(ctx) => CategoryBmc::update(ctx, &params.id, params.data)
            .await
            .into(),
        Err(_) => Err(Error::CtxFail).into(),
    }
}

#[command]
pub async fn delete_category(
    app: AppHandle<Wry>,
    params: DeleteParams,
) -> IpcResponse<ModelMutateResultData> {
    log::trace!("delete_category( params: {:?} )", params);

    match Ctx::from_app(app) {
        Ok(ctx) => CategoryBmc::delete(ctx, &params.id).await.into(),
        Err(_) => Err(Error::CtxFail).into(),
    }
}

#[command]
pub async fn list_categories(
    app: AppHandle<Wry>,
    params: ListParams<CategoryFilter>,
) -> IpcResponse<Vec<Category>> {
    log::trace!("list_categories( params: {:?} )", params);
    
    match Ctx::from_app(app) {
        Ok(ctx) => CategoryBmc::list(ctx, params.filter).await.into(),
        Err(_) => Err(Error::CtxFail).into(),
    }
}
