//! cashiers 域 DTO（响应不含 pin_hash）。
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow)]
pub struct CashierView {
    pub id: i64,
    pub username: String,
    pub full_name: Option<String>,
    pub is_supervisor: bool,
    pub status: String,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct CashierDetail {
    #[serde(flatten)]
    pub cashier: CashierView,
    pub store_ids: Vec<i64>,
}

#[derive(Deserialize)]
pub struct CreateCashier {
    pub username: String,
    pub pin: String,
    pub full_name: Option<String>,
    pub is_supervisor: Option<bool>,
    pub store_ids: Vec<i64>,
    pub home_store_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct UpdateCashier {
    pub full_name: Option<String>,
    pub is_supervisor: Option<bool>,
    pub store_ids: Option<Vec<i64>>,
    pub home_store_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct ResetPin {
    pub new_pin: String,
}
