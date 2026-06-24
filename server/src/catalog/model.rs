//! catalog 域 DTO。
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow)]
pub struct Category {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub sort: i32,
    pub parent_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct CreateCategory {
    pub code: String,
    pub name: String,
    pub sort: Option<i32>,
    pub parent_id: Option<i64>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Product {
    pub id: i64,
    pub sku: String,
    pub barcode: Option<String>,
    pub name: String,
    pub category_id: Option<i64>,
    pub price: Decimal,
    pub cost: Decimal,
    pub unit: Option<String>,
    pub is_weighted: bool,
    pub tax_rate: Decimal,
    pub tax_category: Option<String>,
    pub image_url: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateProduct {
    pub sku: String,
    pub name: String,
    pub price: Decimal,
    pub barcode: Option<String>,
    pub category_id: Option<i64>,
    pub cost: Option<Decimal>,
    pub unit: Option<String>,
    pub is_weighted: Option<bool>,
    pub tax_rate: Option<Decimal>,
    pub tax_category: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub price: Option<Decimal>,
    pub cost: Option<Decimal>,
    pub barcode: Option<String>,
    pub category_id: Option<i64>,
    pub unit: Option<String>,
    pub tax_rate: Option<Decimal>,
    pub status: Option<String>,
    pub image_url: Option<String>,
}
