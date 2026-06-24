//! orders 域数据访问。
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, Transaction};

use crate::{error::AppResult, orders::model::*};

/// 单据连号：按店原子自增（行锁，作废占号不回收）。
pub async fn next_seq(tx: &mut Transaction<'_, Postgres>, store_id: i64) -> AppResult<i64> {
    sqlx::query("INSERT INTO order_counters (store_id) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(store_id)
        .execute(&mut **tx)
        .await?;
    let row: (i64,) = sqlx::query_as(
        "UPDATE order_counters SET next_seq = next_seq + 1 WHERE store_id = $1 RETURNING next_seq - 1",
    )
    .bind(store_id)
    .fetch_one(&mut **tx)
    .await?;
    Ok(row.0)
}

pub async fn find_by_idem(db: &PgPool, key: &uuid::Uuid) -> AppResult<Option<i64>> {
    let row: Option<(i64,)> = sqlx::query_as("SELECT id FROM orders WHERE idempotency_key = $1")
        .bind(key)
        .fetch_optional(db)
        .await?;
    Ok(row.map(|r| r.0))
}

pub async fn get_view(db: &PgPool, id: i64) -> AppResult<Option<OrderView>> {
    let order = sqlx::query_as::<_, OrderRow>(
        "SELECT id, order_no, kind, parent_order_id, store_id, cashier_id, member_id, \
         subtotal, discount, tax, total, status, created_at FROM orders WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(db)
    .await?;
    let Some(order) = order else { return Ok(None) };
    let items = sqlx::query_as::<_, OrderItemRow>(
        "SELECT id, product_id, sku_snapshot, name_snapshot, unit_price, quantity, tax_amount, line_total \
         FROM order_items WHERE order_id = $1 ORDER BY id",
    )
    .bind(id).fetch_all(db).await?;
    let payments = sqlx::query_as::<_, PaymentRow>(
        "SELECT method, kind, amount_base, tendered FROM payments WHERE order_id = $1 ORDER BY id",
    )
    .bind(id).fetch_all(db).await?;
    Ok(Some(OrderView { order, items, payments }))
}

pub async fn list(db: &PgPool, store_id: Option<i64>) -> AppResult<Vec<OrderRow>> {
    Ok(sqlx::query_as::<_, OrderRow>(
        "SELECT id, order_no, kind, parent_order_id, store_id, cashier_id, member_id, \
         subtotal, discount, tax, total, status, created_at FROM orders \
         WHERE ($1::bigint IS NULL OR store_id = $1) ORDER BY id DESC LIMIT 200",
    )
    .bind(store_id)
    .fetch_all(db)
    .await?)
}

/// 取订单头（退款用，store 作用域校验在 service）。
pub async fn get_row(db: &PgPool, id: i64) -> AppResult<Option<OrderRow>> {
    Ok(sqlx::query_as::<_, OrderRow>(
        "SELECT id, order_no, kind, parent_order_id, store_id, cashier_id, member_id, \
         subtotal, discount, tax, total, status, created_at FROM orders WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(db)
    .await?)
}

/// 退款用：原单的明细（含仓库定位）。
pub async fn order_items_for_refund(db: &PgPool, order_id: i64) -> AppResult<Vec<(Option<i64>, String, String, Decimal, Decimal, Decimal, Decimal, Decimal)>> {
    // (product_id, sku, name, unit_price, unit_cost, quantity, tax_amount, line_total)
    Ok(sqlx::query_as(
        "SELECT product_id, sku_snapshot, name_snapshot, unit_price, unit_cost, quantity, tax_amount, line_total \
         FROM order_items WHERE order_id = $1",
    )
    .bind(order_id)
    .fetch_all(db)
    .await?)
}

pub async fn order_warehouse(db: &PgPool, order_id: i64) -> AppResult<Option<i64>> {
    let row: Option<(Option<i64>,)> = sqlx::query_as("SELECT warehouse_id FROM orders WHERE id = $1")
        .bind(order_id).fetch_optional(db).await?;
    Ok(row.and_then(|r| r.0))
}
