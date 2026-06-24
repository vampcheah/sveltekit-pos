//! promotions 域：促销/优惠券管理 + 结账前校验。
use axum::{extract::State, routing::post, Json, Router};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    error::AppResult,
    shared::auth::{require_permission, AdminActor, CurrentActor},
    state::AppState,
};

#[derive(Deserialize)]
struct CreatePromotion {
    name: String,
    r#type: String, // percent|amount
    value: Decimal,
    min_amount: Option<Decimal>,
}

#[derive(Deserialize)]
struct CreateCoupon {
    code: String,
    promotion_id: i64,
    max_uses: Option<i32>,
    per_member_limit: Option<i32>,
}

#[derive(Deserialize)]
struct ValidateReq {
    code: String,
    subtotal: Decimal,
}

#[derive(Serialize)]
struct ValidateResp {
    valid: bool,
    discount: Decimal,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/promotions", post(create_promotion))
        .route("/coupons", post(create_coupon))
        .route("/coupons/validate", post(validate))
}

async fn create_promotion(State(st): State<AppState>, a: AdminActor, Json(b): Json<CreatePromotion>) -> AppResult<Json<serde_json::Value>> {
    require_permission(&st, &a.0, "promotions.write").await?;
    let id: i64 = sqlx::query_scalar(
        "INSERT INTO promotions (name, type, value, min_amount) VALUES ($1,$2,$3,COALESCE($4,0)) RETURNING id",
    )
    .bind(&b.name).bind(&b.r#type).bind(b.value).bind(b.min_amount)
    .fetch_one(&st.db).await?;
    Ok(Json(json!({ "id": id })))
}

async fn create_coupon(State(st): State<AppState>, a: AdminActor, Json(b): Json<CreateCoupon>) -> AppResult<Json<serde_json::Value>> {
    require_permission(&st, &a.0, "promotions.write").await?;
    let id: i64 = sqlx::query_scalar(
        "INSERT INTO coupons (code, promotion_id, max_uses, per_member_limit) VALUES ($1,$2,$3,$4) RETURNING id",
    )
    .bind(&b.code).bind(b.promotion_id).bind(b.max_uses).bind(b.per_member_limit)
    .fetch_one(&st.db).await?;
    Ok(Json(json!({ "id": id })))
}

/// 结账前预览券折扣（不核销）。
async fn validate(State(st): State<AppState>, _a: CurrentActor, Json(b): Json<ValidateReq>) -> AppResult<Json<ValidateResp>> {
    let row: Option<(String, Decimal, Decimal)> = sqlx::query_as(
        "SELECT p.type, p.value, p.min_amount FROM coupons c JOIN promotions p ON p.id = c.promotion_id \
         WHERE c.code = $1 AND c.status='active' AND p.status='active' \
           AND (c.expires_at IS NULL OR c.expires_at > now()) \
           AND (c.max_uses IS NULL OR c.used_count < c.max_uses)",
    )
    .bind(&b.code)
    .fetch_optional(&st.db)
    .await?;
    let Some((ptype, value, min_amount)) = row else {
        return Ok(Json(ValidateResp { valid: false, discount: Decimal::ZERO }));
    };
    if b.subtotal < min_amount {
        return Ok(Json(ValidateResp { valid: false, discount: Decimal::ZERO }));
    }
    let mut discount = match ptype.as_str() {
        "percent" => (b.subtotal * value / Decimal::from(100)).round_dp(2),
        _ => value,
    };
    if discount > b.subtotal {
        discount = b.subtotal;
    }
    Ok(Json(ValidateResp { valid: true, discount }))
}
