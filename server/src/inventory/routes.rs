//! inventory 域路由。读对登录者开放；warehouse 建=stores.write；调整=stock.adjust；调拨=stock.transfer。
use axum::{
    extract::{Query, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    error::AppResult,
    inventory::{model::*, repo},
    shared::{
        audit,
        auth::{require_permission, AdminActor, CurrentActor},
    },
    state::AppState,
};

#[derive(Deserialize)]
struct Wh {
    warehouse_id: Option<i64>,
    store_id: Option<i64>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/warehouses", get(list_warehouses).post(create_warehouse))
        .route("/stock", get(list_stock))
        .route("/stock/low", get(low_stock))
        .route("/stock/adjust", post(adjust))
        .route("/stock/transfer", post(transfer))
}

async fn list_warehouses(State(st): State<AppState>, _a: CurrentActor, Query(q): Query<Wh>) -> AppResult<Json<Vec<Warehouse>>> {
    Ok(Json(repo::list_warehouses(&st.db, q.store_id).await?))
}

async fn create_warehouse(State(st): State<AppState>, a: AdminActor, Json(body): Json<CreateWarehouse>) -> AppResult<Json<Warehouse>> {
    require_permission(&st, &a.0, "stores.write").await?;
    Ok(Json(repo::create_warehouse(&st.db, &body).await?))
}

async fn list_stock(State(st): State<AppState>, _a: CurrentActor, Query(q): Query<Wh>) -> AppResult<Json<Vec<StockRow>>> {
    Ok(Json(repo::list_stock(&st.db, q.warehouse_id).await?))
}

async fn low_stock(State(st): State<AppState>, a: AdminActor) -> AppResult<Json<Vec<StockRow>>> {
    require_permission(&st, &a.0, "reports.read").await?;
    Ok(Json(repo::low_stock(&st.db).await?))
}

async fn adjust(State(st): State<AppState>, a: AdminActor, Json(body): Json<AdjustReq>) -> AppResult<Json<serde_json::Value>> {
    require_permission(&st, &a.0, "stock.adjust").await?;
    let pid = body.product_id;
    repo::adjust(&st.db, &body, a.0.session.actor_id).await?;
    audit::record(&st, &a.0, "stock.adjust", "product", pid, Some(json!({ "delta": body.quantity_delta }))).await;
    Ok(Json(json!({ "ok": true })))
}

async fn transfer(State(st): State<AppState>, a: AdminActor, Json(body): Json<TransferReq>) -> AppResult<Json<serde_json::Value>> {
    require_permission(&st, &a.0, "stock.transfer").await?;
    repo::transfer(&st.db, &body, a.0.session.actor_id).await?;
    Ok(Json(json!({ "ok": true })))
}
