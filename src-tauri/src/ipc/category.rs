//! Tauri IPC command s to bridge Project Frontend Model Controller to Backend Model Controller
//!

use super::{CreateParams, DeleteParams, GetParams, IpcResponse, ListParams, UpdateParams};
use crate::ctx::Ctx;
use crate::model::{
    ModelMutateResultData, Category, CategoryBmc, CategoryFilter, CategoryForCreate, CategoryForUpdate,
};
use crate::prelude::*;
use tauri::{command, AppHandle, Wry};

#[command]
pub async fn get_category(app: AppHandle<Wry>, params: GetParams) -> IpcResponse<Category> {
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
    println!("Update category called with {params:?}");
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
    match Ctx::from_app(app) {
        Ok(ctx) => CategoryBmc::list(ctx, params.filter).await.into(),
        Err(_) => Err(Error::CtxFail).into(),
    }
}
