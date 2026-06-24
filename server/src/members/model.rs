//! members 域 DTO。金额用 Decimal（精确，绝不 float）。
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow)]
pub struct MemberView {
    pub id: i64,
    pub code: Option<String>,
    pub phone: Option<String>,
    pub name: String,
    pub email: Option<String>,
    pub tier: String,
    pub points: i64,
    pub balance: Decimal,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateMember {
    pub name: String,
    pub phone: Option<String>,
    pub code: Option<String>,
    pub email: Option<String>,
    pub tier: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateMember {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub tier: Option<String>,
    pub status: Option<String>,
}

/// 积分调整：正负皆可。
#[derive(Deserialize)]
pub struct PointsOp {
    pub points_delta: i64,
    pub note: Option<String>,
}

/// 储值充值：amount 用字符串避免 float 精度损失。
#[derive(Deserialize)]
pub struct TopupOp {
    pub amount: Decimal,
    pub note: Option<String>,
}
