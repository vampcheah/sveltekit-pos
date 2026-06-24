//! inventory 域 DTO。
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow)]
pub struct Warehouse {
    pub id: i64,
    pub store_id: i64,
    pub code: String,
    pub name: String,
    pub r#type: String,
}

#[derive(Deserialize)]
pub struct CreateWarehouse {
    pub store_id: i64,
    pub code: String,
    pub name: String,
    pub r#type: Option<String>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct StockRow {
    pub warehouse_id: i64,
    pub product_id: i64,
    pub sku: String,
    pub product_name: String,
    pub quantity: Decimal,
    pub reorder_point: Decimal,
}

#[derive(Deserialize)]
pub struct AdjustReq {
    pub warehouse_id: i64,
    pub product_id: i64,
    pub quantity_delta: Decimal, // 正=入，负=出
    pub note: Option<String>,
}

#[derive(Deserialize)]
pub struct TransferReq {
    pub from_warehouse_id: i64,
    pub to_warehouse_id: i64,
    pub product_id: i64,
    pub quantity: Decimal, // 正数
    pub note: Option<String>,
}
