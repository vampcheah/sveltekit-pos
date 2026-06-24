//! stores 域 DTO。
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow)]
pub struct Store {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub timezone: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateStore {
    pub name: String,
    pub code: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateStore {
    pub name: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub status: Option<String>,
}
