//! Tauri IPC command s to bridge Project Frontend Model Controller to Backend Model Controller
//!

use super::{CreateParams, DeleteParams, GetParams, IpcResponse, ListParams, UpdateParams};
use crate::ctx::Ctx;
use crate::model::{
    ModelMutateResultData, TimeRecord, TimeRecordBmc, TimeRecordFilter, TimeRecordForCreate, TimeRecordForUpdate,
};
use crate::prelude::*;
use tauri::{command, AppHandle, Wry};

#[command]
pub async fn get_time_record(app: AppHandle<Wry>, params: GetParams) -> IpcResponse<TimeRecord> {
    match Ctx::from_app(app) {
        Ok(ctx) => TimeRecordBmc::get(ctx, &params.id).await.into(),
        Err(_) => Err(Error::CtxFail).into(),
    }
}

#[command]
pub async fn create_time_record(
    app: AppHandle<Wry>,
    params: CreateParams<TimeRecordForCreate>,
) -> IpcResponse<ModelMutateResultData> {
    match Ctx::from_app(app) {
        Ok(ctx) => TimeRecordBmc::create(ctx, params.data).await.into(),
        Err(_) => Err(Error::CtxFail).into(),
    }
}

#[command]
pub async fn update_time_record(
    app: AppHandle<Wry>,
    params: UpdateParams<TimeRecordForUpdate>,
) -> IpcResponse<ModelMutateResultData> {
    match Ctx::from_app(app) {
        Ok(ctx) => TimeRecordBmc::update(ctx, &params.id, params.data)
            .await
            .into(),
        Err(_) => Err(Error::CtxFail).into(),
    }
}

#[command]
pub async fn delete_time_record(
    app: AppHandle<Wry>,
    params: DeleteParams,
) -> IpcResponse<ModelMutateResultData> {
    match Ctx::from_app(app) {
        Ok(ctx) => TimeRecordBmc::delete(ctx, &params.id).await.into(),
        Err(_) => Err(Error::CtxFail).into(),
    }
}

#[command]
pub async fn list_time_records(
    app: AppHandle<Wry>,
    params: ListParams<TimeRecordFilter>,
) -> IpcResponse<Vec<TimeRecord>> {
    match Ctx::from_app(app) {
        Ok(ctx) => TimeRecordBmc::list(ctx, params.filter).await.into(),
        Err(_) => Err(Error::CtxFail).into(),
    }
}
