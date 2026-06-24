//! admins 域 DTO（响应永不含 password_hash）。
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow)]
pub struct AdminView {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub role_id: Option<i64>,
    pub status: String,
    pub must_change_password: bool,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateAdmin {
    pub username: String,
    pub password: String,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub role_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct UpdateAdmin {
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub role_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct ResetPassword {
    pub new_password: String,
}
