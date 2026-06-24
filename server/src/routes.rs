//! 顶层路由：/health + /api/v1（各域 router 后续在此挂载）。
use axum::{extract::State, routing::get, Json, Router};
use serde_json::{json, Value};

use crate::{
    admins, auth, cashiers, catalog, error::AppResult, inventory, members, orders, promotions,
    rbac, reports, settings, shifts, state::AppState, stores,
};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .nest("/api/v1", api_v1())
        .with_state(state)
}

fn api_v1() -> Router<AppState> {
    // 各域 router 在其阶段挂载到此
    Router::new()
        .route("/ping", get(|| async { "pong" }))
        .merge(auth::routes::router())
        .merge(stores::routes::router())
        .merge(admins::routes::router())
        .merge(rbac::router())
        .merge(cashiers::routes::router())
        .merge(members::routes::router())
        .merge(settings::router())
        .merge(catalog::routes::router())
        .merge(inventory::routes::router())
        .merge(orders::routes::router())
        .merge(promotions::router())
        .merge(reports::router())
        .merge(shifts::router())
}

/// 探活：同时验证 db + redis 可达。
async fn health(State(st): State<AppState>) -> AppResult<Json<Value>> {
    sqlx::query("SELECT 1").execute(&st.db).await?;
    let mut redis = st.redis.clone();
    let pong: String = redis::cmd("PING").query_async(&mut redis).await?;
    Ok(Json(json!({ "status": "ok", "db": "up", "redis": pong })))
}
