//! members 域路由。写要 `members.write`；积分/储值是"能薅钱"操作，独立高权。
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use rust_decimal::Decimal;

use crate::{
    error::{AppError, AppResult},
    members::{model::*, repo},
    shared::{
        audit,
        auth::{require_permission, AdminActor},
    },
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/members", get(list).post(create))
        .route("/members/{id}", get(get_one).patch(update))
        .route("/members/{id}/points", post(adjust_points))
        .route("/members/{id}/topup", post(topup))
}

async fn list(State(st): State<AppState>, a: AdminActor) -> AppResult<Json<Vec<MemberView>>> {
    require_permission(&st, &a.0, "members.write").await?;
    Ok(Json(repo::list(&st.db).await?))
}

async fn get_one(State(st): State<AppState>, a: AdminActor, Path(id): Path<i64>) -> AppResult<Json<MemberView>> {
    require_permission(&st, &a.0, "members.write").await?;
    repo::get(&st.db, id).await?.map(Json).ok_or(AppError::NotFound)
}

async fn create(State(st): State<AppState>, a: AdminActor, Json(body): Json<CreateMember>) -> AppResult<Json<MemberView>> {
    require_permission(&st, &a.0, "members.write").await?;
    Ok(Json(repo::create(&st.db, &body).await?))
}

async fn update(State(st): State<AppState>, a: AdminActor, Path(id): Path<i64>, Json(body): Json<UpdateMember>) -> AppResult<Json<MemberView>> {
    require_permission(&st, &a.0, "members.write").await?;
    repo::update(&st.db, id, &body).await?.map(Json).ok_or(AppError::NotFound)
}

async fn adjust_points(State(st): State<AppState>, a: AdminActor, Path(id): Path<i64>, Json(body): Json<PointsOp>) -> AppResult<Json<MemberView>> {
    require_permission(&st, &a.0, "members.points.write").await?;
    let v = repo::apply_ledger(&st.db, id, "adjust", body.points_delta, Decimal::ZERO, body.note.as_deref(), a.0.session.actor_id).await?;
    audit::record(&st, &a.0, "members.points", "member", id, Some(serde_json::json!({ "points_delta": body.points_delta }))).await;
    Ok(Json(v))
}

async fn topup(State(st): State<AppState>, a: AdminActor, Path(id): Path<i64>, Json(body): Json<TopupOp>) -> AppResult<Json<MemberView>> {
    require_permission(&st, &a.0, "members.balance.write").await?;
    if body.amount <= Decimal::ZERO {
        return Err(AppError::BadRequest("充值金额须为正".into()));
    }
    let v = repo::apply_ledger(&st.db, id, "topup", 0, body.amount, body.note.as_deref(), a.0.session.actor_id).await?;
    audit::record(&st, &a.0, "members.topup", "member", id, Some(serde_json::json!({ "amount": body.amount }))).await;
    Ok(Json(v))
}
