//! shifts 域：交接班（开班备用金/闭班点钞算差额）+ 差异审批 + 审计日志查询。
use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    error::{AppError, AppResult},
    shared::auth::{require_permission, AdminActor, CashierActor, CurrentActor},
    state::AppState,
};

#[derive(Serialize, sqlx::FromRow)]
struct Shift {
    id: i64,
    store_id: i64,
    cashier_id: i64,
    opened_at: DateTime<Utc>,
    closed_at: Option<DateTime<Utc>>,
    opening_cash: Decimal,
    closing_cash: Option<Decimal>,
    expected_cash: Option<Decimal>,
    variance: Option<Decimal>,
    status: String,
}

#[derive(Deserialize)]
struct OpenReq {
    opening_cash: Decimal,
}
#[derive(Deserialize)]
struct CloseReq {
    closing_cash: Decimal,
    variance_reason: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/shifts", get(list))
        .route("/shifts/open", post(open))
        .route("/shifts/close", post(close))
        .route("/shifts/{id}/approve", post(approve))
        .route("/audit-logs", get(audit_logs))
}

async fn list(State(st): State<AppState>, actor: CurrentActor) -> AppResult<Json<Vec<Shift>>> {
    // cashier 仅本店；admin 全部
    let scope = (actor.session.actor_type == "cashier").then_some(actor.session.store_id).flatten();
    Ok(Json(sqlx::query_as::<_, Shift>(
        "SELECT id, store_id, cashier_id, opened_at, closed_at, opening_cash, closing_cash, expected_cash, variance, status \
         FROM shifts WHERE ($1::bigint IS NULL OR store_id = $1) ORDER BY id DESC LIMIT 100",
    ).bind(scope).fetch_all(&st.db).await?))
}

async fn open(State(st): State<AppState>, a: CashierActor, Json(b): Json<OpenReq>) -> AppResult<Json<Shift>> {
    let cashier_id = a.0.session.actor_id;
    let store_id = a.0.session.store_id.ok_or_else(|| AppError::BadRequest("未绑定门店".into()))?;
    // 一个收银员同时只能有一个开班
    let open_exists: Option<(i64,)> = sqlx::query_as("SELECT id FROM shifts WHERE cashier_id = $1 AND status = 'open'")
        .bind(cashier_id).fetch_optional(&st.db).await?;
    if open_exists.is_some() {
        return Err(AppError::BadRequest("已有开班，请先闭班".into()));
    }
    Ok(Json(sqlx::query_as::<_, Shift>(
        "INSERT INTO shifts (store_id, cashier_id, opening_cash) VALUES ($1,$2,$3) \
         RETURNING id, store_id, cashier_id, opened_at, closed_at, opening_cash, closing_cash, expected_cash, variance, status",
    ).bind(store_id).bind(cashier_id).bind(b.opening_cash).fetch_one(&st.db).await?))
}

async fn close(State(st): State<AppState>, a: CashierActor, Json(b): Json<CloseReq>) -> AppResult<Json<Shift>> {
    let cashier_id = a.0.session.actor_id;
    let sh: Option<(i64, DateTime<Utc>, Decimal)> = sqlx::query_as(
        "SELECT id, opened_at, opening_cash FROM shifts WHERE cashier_id = $1 AND status = 'open'",
    ).bind(cashier_id).fetch_optional(&st.db).await?;
    let Some((shift_id, opened_at, opening_cash)) = sh else {
        return Err(AppError::BadRequest("无开班记录".into()));
    };
    // 本班现金净额：现金支付 - 现金退款（按本收银员、开班时间起）
    let (cash_in,): (Decimal,) = sqlx::query_as(
        "SELECT COALESCE(SUM(CASE WHEN p.kind='payment' THEN p.amount_base ELSE -p.amount_base END),0) \
         FROM payments p JOIN orders o ON o.id = p.order_id \
         WHERE p.method='cash' AND o.cashier_id = $1 AND o.created_at >= $2",
    ).bind(cashier_id).bind(opened_at).fetch_one(&st.db).await?;
    let expected = opening_cash + cash_in;
    let variance = b.closing_cash - expected;
    let status = if variance == Decimal::ZERO { "closed" } else { "pending_review" };
    Ok(Json(sqlx::query_as::<_, Shift>(
        "UPDATE shifts SET closed_at = now(), closing_cash = $2, expected_cash = $3, variance = $4, \
           status = $5, variance_reason = $6 WHERE id = $1 \
         RETURNING id, store_id, cashier_id, opened_at, closed_at, opening_cash, closing_cash, expected_cash, variance, status",
    ).bind(shift_id).bind(b.closing_cash).bind(expected).bind(variance).bind(status).bind(&b.variance_reason)
    .fetch_one(&st.db).await?))
}

async fn approve(State(st): State<AppState>, a: AdminActor, Path(id): Path<i64>) -> AppResult<Json<serde_json::Value>> {
    require_permission(&st, &a.0, "shifts.approve").await?;
    let r = sqlx::query("UPDATE shifts SET status='approved', approved_by=$2 WHERE id=$1 AND status='pending_review'")
        .bind(id).bind(a.0.session.actor_id).execute(&st.db).await?;
    if r.rows_affected() != 1 {
        return Err(AppError::NotFound);
    }
    Ok(Json(json!({ "ok": true })))
}

#[derive(Deserialize)]
struct AuditQuery {
    action: Option<String>,
    limit: Option<i64>,
}

#[derive(Serialize, sqlx::FromRow)]
struct AuditRow {
    id: i64,
    actor_type: String,
    actor_id: Option<i64>,
    action: String,
    entity: Option<String>,
    entity_id: Option<i64>,
    created_at: DateTime<Utc>,
}

async fn audit_logs(State(st): State<AppState>, a: AdminActor, Query(q): Query<AuditQuery>) -> AppResult<Json<Vec<AuditRow>>> {
    require_permission(&st, &a.0, "audit.read").await?;
    let limit = q.limit.unwrap_or(100).clamp(1, 1000);
    Ok(Json(sqlx::query_as::<_, AuditRow>(
        "SELECT id, actor_type, actor_id, action, entity, entity_id, created_at FROM audit_logs \
         WHERE ($1::text IS NULL OR action = $1) ORDER BY id DESC LIMIT $2",
    ).bind(&q.action).bind(limit).fetch_all(&st.db).await?))
}
