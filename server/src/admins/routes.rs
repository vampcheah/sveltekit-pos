//! admins 域路由。全部要 admin + `admins.write`。
//! 改密/禁用/改角色 → 踢线（撤会话），保证撤权实时生效。
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};

use crate::{
    admins::{model::*, repo},
    error::{AppError, AppResult},
    shared::{
        auth::{require_permission, AdminActor},
        password, session,
    },
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/admins", get(list).post(create))
        .route("/admins/{id}", get(get_one).patch(update))
        .route("/admins/{id}/reset-password", post(reset_password))
        .route("/admins/{id}/disable", post(disable))
        .route("/admins/{id}/enable", post(enable))
        .route("/admins/{id}/unlock", post(unlock))
}

async fn guard(st: &AppState, a: &AdminActor) -> AppResult<()> {
    require_permission(st, &a.0, "admins.write").await
}

async fn list(State(st): State<AppState>, a: AdminActor) -> AppResult<Json<Vec<AdminView>>> {
    guard(&st, &a).await?;
    Ok(Json(repo::list(&st.db).await?))
}

async fn get_one(State(st): State<AppState>, a: AdminActor, Path(id): Path<i64>) -> AppResult<Json<AdminView>> {
    guard(&st, &a).await?;
    repo::get(&st.db, id).await?.map(Json).ok_or(AppError::NotFound)
}

async fn create(State(st): State<AppState>, a: AdminActor, Json(body): Json<CreateAdmin>) -> AppResult<Json<AdminView>> {
    guard(&st, &a).await?;
    let hash = password::hash(&body.password)?;
    Ok(Json(repo::create(&st.db, &body, &hash).await?))
}

async fn update(State(st): State<AppState>, a: AdminActor, Path(id): Path<i64>, Json(body): Json<UpdateAdmin>) -> AppResult<Json<AdminView>> {
    guard(&st, &a).await?;
    let role_changed = body.role_id.is_some();
    let v = repo::update(&st.db, id, &body).await?.ok_or(AppError::NotFound)?;
    if role_changed {
        session::revoke_all(&st, "admin", id).await?; // 新角色须重新登录生效
    }
    Ok(Json(v))
}

async fn reset_password(State(st): State<AppState>, a: AdminActor, Path(id): Path<i64>, Json(body): Json<ResetPassword>) -> AppResult<Json<Value>> {
    guard(&st, &a).await?;
    let hash = password::hash(&body.new_password)?;
    if !repo::set_password(&st.db, id, &hash).await? {
        return Err(AppError::NotFound);
    }
    session::revoke_all(&st, "admin", id).await?;
    Ok(Json(json!({ "ok": true })))
}

async fn disable(State(st): State<AppState>, a: AdminActor, Path(id): Path<i64>) -> AppResult<Json<Value>> {
    guard(&st, &a).await?;
    if !repo::set_status(&st.db, id, true).await? {
        return Err(AppError::NotFound);
    }
    session::revoke_all(&st, "admin", id).await?; // 禁用即踢线
    Ok(Json(json!({ "ok": true })))
}

async fn enable(State(st): State<AppState>, a: AdminActor, Path(id): Path<i64>) -> AppResult<Json<Value>> {
    guard(&st, &a).await?;
    if !repo::set_status(&st.db, id, false).await? {
        return Err(AppError::NotFound);
    }
    Ok(Json(json!({ "ok": true })))
}

async fn unlock(State(st): State<AppState>, a: AdminActor, Path(id): Path<i64>) -> AppResult<Json<Value>> {
    guard(&st, &a).await?;
    if !repo::unlock(&st.db, id).await? {
        return Err(AppError::NotFound);
    }
    Ok(Json(json!({ "ok": true })))
}
