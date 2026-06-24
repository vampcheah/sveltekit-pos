//! cashiers 域路由。写要 `cashiers.write`；重置PIN要 `cashiers.reset_pin`。
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};

use crate::{
    cashiers::{model::*, repo},
    error::{AppError, AppResult},
    shared::{
        auth::{require_permission, AdminActor},
        password, session,
    },
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/cashiers", get(list).post(create))
        .route("/cashiers/{id}", get(get_one).patch(update))
        .route("/cashiers/{id}/reset-pin", post(reset_pin))
        .route("/cashiers/{id}/disable", post(disable))
        .route("/cashiers/{id}/enable", post(enable))
        .route("/cashiers/{id}/unlock", post(unlock))
}

async fn w(st: &AppState, a: &AdminActor) -> AppResult<()> {
    require_permission(st, &a.0, "cashiers.write").await
}

async fn list(State(st): State<AppState>, a: AdminActor) -> AppResult<Json<Vec<CashierView>>> {
    w(&st, &a).await?;
    Ok(Json(repo::list(&st.db).await?))
}

async fn get_one(State(st): State<AppState>, a: AdminActor, Path(id): Path<i64>) -> AppResult<Json<CashierDetail>> {
    w(&st, &a).await?;
    let cashier = repo::get(&st.db, id).await?.ok_or(AppError::NotFound)?;
    let store_ids = repo::store_ids(&st.db, id).await?;
    Ok(Json(CashierDetail { cashier, store_ids }))
}

async fn create(State(st): State<AppState>, a: AdminActor, Json(body): Json<CreateCashier>) -> AppResult<Json<CashierView>> {
    w(&st, &a).await?;
    let pin_hash = password::hash(&body.pin)?;
    Ok(Json(repo::create(&st.db, &body, &pin_hash).await?))
}

async fn update(State(st): State<AppState>, a: AdminActor, Path(id): Path<i64>, Json(body): Json<UpdateCashier>) -> AppResult<Json<CashierView>> {
    w(&st, &a).await?;
    repo::update(&st.db, id, &body).await?.map(Json).ok_or(AppError::NotFound)
}

async fn reset_pin(State(st): State<AppState>, a: AdminActor, Path(id): Path<i64>, Json(body): Json<ResetPin>) -> AppResult<Json<Value>> {
    require_permission(&st, &a.0, "cashiers.reset_pin").await?;
    let pin_hash = password::hash(&body.new_pin)?;
    if !repo::set_pin(&st.db, id, &pin_hash).await? {
        return Err(AppError::NotFound);
    }
    session::revoke_all(&st, "cashier", id).await?;
    Ok(Json(json!({ "ok": true })))
}

async fn disable(State(st): State<AppState>, a: AdminActor, Path(id): Path<i64>) -> AppResult<Json<Value>> {
    w(&st, &a).await?;
    if !repo::set_status(&st.db, id, true).await? {
        return Err(AppError::NotFound);
    }
    session::revoke_all(&st, "cashier", id).await?;
    Ok(Json(json!({ "ok": true })))
}

async fn enable(State(st): State<AppState>, a: AdminActor, Path(id): Path<i64>) -> AppResult<Json<Value>> {
    w(&st, &a).await?;
    if !repo::set_status(&st.db, id, false).await? {
        return Err(AppError::NotFound);
    }
    Ok(Json(json!({ "ok": true })))
}

async fn unlock(State(st): State<AppState>, a: AdminActor, Path(id): Path<i64>) -> AppResult<Json<Value>> {
    w(&st, &a).await?;
    if !repo::unlock(&st.db, id).await? {
        return Err(AppError::NotFound);
    }
    Ok(Json(json!({ "ok": true })))
}
