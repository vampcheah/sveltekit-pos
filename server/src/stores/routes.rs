//! stores 域路由（薄 handler）。写操作要 admin + `stores.write`（deny-by-default）。
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

use crate::{
    error::{AppError, AppResult},
    shared::auth::{require_permission, AdminActor},
    state::AppState,
    stores::{model::*, repo},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/stores", get(list).post(create))
        .route("/stores/{id}", get(get_one).patch(update))
}

async fn list(State(st): State<AppState>, _a: AdminActor) -> AppResult<Json<Vec<Store>>> {
    Ok(Json(repo::list(&st.db).await?))
}

async fn get_one(
    State(st): State<AppState>,
    _a: AdminActor,
    Path(id): Path<i64>,
) -> AppResult<Json<Store>> {
    repo::get(&st.db, id).await?.map(Json).ok_or(AppError::NotFound)
}

async fn create(
    State(st): State<AppState>,
    a: AdminActor,
    Json(body): Json<CreateStore>,
) -> AppResult<Json<Store>> {
    require_permission(&st, &a.0, "stores.write").await?;
    Ok(Json(repo::create(&st.db, &body).await?))
}

async fn update(
    State(st): State<AppState>,
    a: AdminActor,
    Path(id): Path<i64>,
    Json(body): Json<UpdateStore>,
) -> AppResult<Json<Store>> {
    require_permission(&st, &a.0, "stores.write").await?;
    repo::update(&st.db, id, &body).await?.map(Json).ok_or(AppError::NotFound)
}
