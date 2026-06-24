//! reports 域：营收（周/月/年）· 看板 · 税额 · 毛利 · 储值负债 · 退款问责 · 号空洞 · CSV 导出。
//! 报表只对 payments/orders 的记账币聚合；都要 reports.read（导出另需 reports.export）。
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    error::{AppError, AppResult},
    shared::auth::{require_permission, AdminActor},
    state::AppState,
};

#[derive(Deserialize)]
struct RevenueQuery {
    period: Option<String>,
    from: Option<DateTime<Utc>>,
    to: Option<DateTime<Utc>>,
}

#[derive(Serialize, sqlx::FromRow)]
struct RevenueRow {
    bucket: DateTime<Utc>,
    sales: Decimal,
    refunds: Decimal,
    net: Decimal,
    orders: i64,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/reports/revenue", get(revenue))
        .route("/reports/revenue.csv", get(revenue_csv))
        .route("/reports/dashboard", get(dashboard))
        .route("/reports/tax-summary", get(tax_summary))
        .route("/reports/margin", get(margin))
        .route("/reports/member-liability", get(member_liability))
        .route("/reports/refunds-summary", get(refunds_summary))
        .route("/reports/sequence-gaps", get(sequence_gaps))
}

fn valid_period(p: &str) -> AppResult<&str> {
    match p {
        "day" | "week" | "month" | "year" => Ok(p),
        _ => Err(AppError::BadRequest("period 须为 day|week|month|year".into())),
    }
}

async fn revenue_rows(st: &AppState, q: &RevenueQuery) -> AppResult<Vec<RevenueRow>> {
    let period = valid_period(q.period.as_deref().unwrap_or("month"))?;
    Ok(sqlx::query_as::<_, RevenueRow>(
        "SELECT date_trunc($1, created_at) AS bucket, \
           COALESCE(SUM(total) FILTER (WHERE kind='sale'),0) AS sales, \
           COALESCE(-SUM(total) FILTER (WHERE kind='refund'),0) AS refunds, \
           COALESCE(SUM(total),0) AS net, \
           COUNT(*) FILTER (WHERE kind='sale') AS orders \
         FROM orders WHERE status <> 'void' \
           AND ($2::timestamptz IS NULL OR created_at >= $2) \
           AND ($3::timestamptz IS NULL OR created_at < $3) \
         GROUP BY bucket ORDER BY bucket",
    )
    .bind(period).bind(q.from).bind(q.to)
    .fetch_all(&st.db)
    .await?)
}

async fn revenue(State(st): State<AppState>, a: AdminActor, Query(q): Query<RevenueQuery>) -> AppResult<Json<Vec<RevenueRow>>> {
    require_permission(&st, &a.0, "reports.read").await?;
    Ok(Json(revenue_rows(&st, &q).await?))
}

async fn revenue_csv(State(st): State<AppState>, a: AdminActor, Query(q): Query<RevenueQuery>) -> AppResult<Response> {
    require_permission(&st, &a.0, "reports.export").await?;
    let rows = revenue_rows(&st, &q).await?;
    let mut csv = String::from("bucket,sales,refunds,net,orders\n");
    for r in &rows {
        csv.push_str(&format!("{},{},{},{},{}\n", r.bucket.to_rfc3339(), r.sales, r.refunds, r.net, r.orders));
    }
    Ok(([(axum::http::header::CONTENT_TYPE, "text/csv; charset=utf-8")], csv).into_response())
}

#[derive(Serialize, sqlx::FromRow)]
struct Dashboard {
    today_orders: i64,
    today_revenue: Decimal,
    avg_ticket: Decimal,
}

async fn dashboard(State(st): State<AppState>, a: AdminActor) -> AppResult<Json<serde_json::Value>> {
    require_permission(&st, &a.0, "reports.read").await?;
    let d: Dashboard = sqlx::query_as(
        "SELECT COUNT(*) FILTER (WHERE kind='sale') AS today_orders, \
           COALESCE(SUM(total) FILTER (WHERE kind='sale'),0) AS today_revenue, \
           COALESCE(AVG(total) FILTER (WHERE kind='sale'),0) AS avg_ticket \
         FROM orders WHERE status <> 'void' AND created_at >= date_trunc('day', now())",
    )
    .fetch_one(&st.db)
    .await?;
    let top: Vec<(String, Decimal)> = sqlx::query_as(
        "SELECT oi.name_snapshot, SUM(oi.quantity) AS qty FROM order_items oi \
         JOIN orders o ON o.id = oi.order_id WHERE o.kind='sale' AND o.status<>'void' \
           AND o.created_at >= date_trunc('day', now()) \
         GROUP BY oi.name_snapshot ORDER BY qty DESC LIMIT 5",
    )
    .fetch_all(&st.db)
    .await?;
    Ok(Json(serde_json::json!({
        "today_orders": d.today_orders, "today_revenue": d.today_revenue,
        "avg_ticket": d.avg_ticket,
        "top_products": top.into_iter().map(|(n,q)| serde_json::json!({"name":n,"qty":q})).collect::<Vec<_>>(),
    })))
}

#[derive(Serialize, sqlx::FromRow)]
struct TaxRow {
    tax_rate: Decimal,
    taxable_base: Decimal,
    tax: Decimal,
}

async fn tax_summary(State(st): State<AppState>, a: AdminActor) -> AppResult<Json<Vec<TaxRow>>> {
    require_permission(&st, &a.0, "reports.read").await?;
    Ok(Json(sqlx::query_as::<_, TaxRow>(
        "SELECT oi.tax_rate, COALESCE(SUM(oi.line_total),0) AS taxable_base, COALESCE(SUM(oi.tax_amount),0) AS tax \
         FROM order_items oi JOIN orders o ON o.id = oi.order_id \
         WHERE o.kind='sale' AND o.status<>'void' GROUP BY oi.tax_rate ORDER BY oi.tax_rate",
    ).fetch_all(&st.db).await?))
}

async fn margin(State(st): State<AppState>, a: AdminActor) -> AppResult<Json<serde_json::Value>> {
    require_permission(&st, &a.0, "reports.read").await?;
    let (rev, cogs): (Decimal, Decimal) = sqlx::query_as(
        "SELECT COALESCE(SUM(oi.line_total),0), COALESCE(SUM(oi.unit_cost*oi.quantity),0) \
         FROM order_items oi JOIN orders o ON o.id = oi.order_id WHERE o.kind='sale' AND o.status<>'void'",
    )
    .fetch_one(&st.db)
    .await?;
    Ok(Json(serde_json::json!({ "revenue": rev, "cogs": cogs, "margin": rev - cogs })))
}

async fn member_liability(State(st): State<AppState>, a: AdminActor) -> AppResult<Json<serde_json::Value>> {
    require_permission(&st, &a.0, "reports.read").await?;
    let (total, n): (Decimal, i64) = sqlx::query_as(
        "SELECT COALESCE(SUM(balance),0), COUNT(*) FILTER (WHERE balance > 0) FROM members WHERE deleted_at IS NULL",
    )
    .fetch_one(&st.db)
    .await?;
    Ok(Json(serde_json::json!({ "total_stored_value": total, "members_with_balance": n })))
}

#[derive(Serialize, sqlx::FromRow)]
struct RefundSummary {
    cashier_id: Option<i64>,
    reason_code: Option<String>,
    count: i64,
    refunded: Decimal,
}

async fn refunds_summary(State(st): State<AppState>, a: AdminActor) -> AppResult<Json<Vec<RefundSummary>>> {
    require_permission(&st, &a.0, "reports.read").await?;
    Ok(Json(sqlx::query_as::<_, RefundSummary>(
        "SELECT cashier_id, reason_code, COUNT(*) AS count, COALESCE(SUM(-total),0) AS refunded \
         FROM orders WHERE kind='refund' GROUP BY cashier_id, reason_code ORDER BY refunded DESC",
    ).fetch_all(&st.db).await?))
}

#[derive(Serialize, sqlx::FromRow)]
struct Gap {
    store_id: i64,
    gap_start: i64,
    gap_end: i64,
}

async fn sequence_gaps(State(st): State<AppState>, a: AdminActor) -> AppResult<Json<Vec<Gap>>> {
    require_permission(&st, &a.0, "reports.read").await?;
    Ok(Json(sqlx::query_as::<_, Gap>(
        "WITH s AS (SELECT store_id, seq_no, LAG(seq_no) OVER (PARTITION BY store_id ORDER BY seq_no) AS prev FROM orders) \
         SELECT store_id, prev + 1 AS gap_start, seq_no - 1 AS gap_end FROM s WHERE seq_no - prev > 1 ORDER BY store_id",
    ).fetch_all(&st.db).await?))
}
