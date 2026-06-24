//! orders 域 DTO。结账只收 product_id+quantity（金额服务端重算，§3 铁律 2）。
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CheckoutItem {
    pub product_id: i64,
    pub quantity: Decimal,
}

#[derive(Deserialize)]
pub struct PaymentIn {
    pub method: String,
    pub amount: Decimal,            // 原始支付币种金额
    pub currency: Option<String>,
    pub rate: Option<Decimal>,      // 1 currency = rate 记账币
    pub tendered: Option<Decimal>,  // 现金实收
    pub r#ref: Option<String>,
}

#[derive(Deserialize)]
pub struct CheckoutReq {
    pub store_id: Option<i64>,      // admin 必填；cashier 取自会话
    pub warehouse_id: i64,
    pub member_id: Option<i64>,
    pub idempotency_key: Option<Uuid>,
    pub coupon_code: Option<String>,
    pub items: Vec<CheckoutItem>,
    #[serde(default)]
    pub payments: Vec<PaymentIn>,   // 空=默认整单现金
}

#[derive(Deserialize)]
pub struct RefundReq {
    pub reason_code: Option<String>,
    pub note: Option<String>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct OrderRow {
    pub id: i64,
    pub order_no: String,
    pub kind: String,
    pub parent_order_id: Option<i64>,
    pub store_id: i64,
    pub cashier_id: Option<i64>,
    pub member_id: Option<i64>,
    pub subtotal: Decimal,
    pub discount: Decimal,
    pub tax: Decimal,
    pub total: Decimal,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct OrderItemRow {
    pub id: i64,
    pub product_id: Option<i64>,
    pub sku_snapshot: String,
    pub name_snapshot: String,
    pub unit_price: Decimal,
    pub quantity: Decimal,
    pub tax_amount: Decimal,
    pub line_total: Decimal,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct PaymentRow {
    pub method: String,
    pub kind: String,
    pub amount_base: Decimal,
    pub tendered: Option<Decimal>,
}

#[derive(Serialize)]
pub struct OrderView {
    #[serde(flatten)]
    pub order: OrderRow,
    pub items: Vec<OrderItemRow>,
    pub payments: Vec<PaymentRow>,
}
