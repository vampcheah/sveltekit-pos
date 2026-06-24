//! orders 域路由。结账：cashier 直接可下单（核心职能），admin 需 orders.create。
//! store 作用域取自会话（§3 铁律 3）；退款需 orders.refund。
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};

use crate::{
    error::{AppError, AppResult},
    orders::{model::*, repo, service},
    shared::{
        audit,
        auth::{require_permission, AdminActor, CurrentActor},
    },
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/orders", get(list).post(checkout))
        .route("/orders/{id}", get(get_one))
        .route("/orders/{id}/refund", post(refund))
}

async fn checkout(State(st): State<AppState>, actor: CurrentActor, Json(req): Json<CheckoutReq>) -> AppResult<Json<OrderView>> {
    let (cashier_id, store_id) = match actor.session.actor_type.as_str() {
        "cashier" => {
            let store = actor.session.store_id.ok_or_else(|| AppError::BadRequest("收银员未绑定门店".into()))?;
            (Some(actor.session.actor_id), store)
        }
        _ => {
            require_permission(&st, &actor, "orders.create").await?;
            let store = req.store_id.ok_or_else(|| AppError::BadRequest("需指定 store_id".into()))?;
            (None, store)
        }
    };
    Ok(Json(service::checkout(&st.db, cashier_id, store_id, &req).await?))
}

async fn list(State(st): State<AppState>, actor: CurrentActor) -> AppResult<Json<Vec<OrderRow>>> {
    let scope = (actor.session.actor_type == "cashier").then_some(actor.session.store_id).flatten();
    Ok(Json(repo::list(&st.db, scope).await?))
}

async fn get_one(State(st): State<AppState>, actor: CurrentActor, Path(id): Path<i64>) -> AppResult<Json<OrderView>> {
    let view = repo::get_view(&st.db, id).await?.ok_or(AppError::NotFound)?;
    if actor.session.actor_type == "cashier" && Some(view.order.store_id) != actor.session.store_id {
        return Err(AppError::NotFound); // 防跨店读
    }
    Ok(Json(view))
}

async fn refund(State(st): State<AppState>, a: AdminActor, Path(id): Path<i64>, Json(req): Json<RefundReq>) -> AppResult<Json<OrderView>> {
    require_permission(&st, &a.0, "orders.refund").await?;
    let view = service::refund(&st.db, None, id, &req).await?;
    audit::record(&st, &a.0, "orders.refund", "order", id, Some(serde_json::json!({ "refund_total": view.order.total }))).await;
    Ok(Json(view))
}
