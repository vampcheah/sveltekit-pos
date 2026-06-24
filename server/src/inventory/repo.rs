//! inventory 域数据访问。库存变动=原子条件更新 + 同事务写流水（防超卖）。
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, Transaction};

use crate::{error::AppError, error::AppResult, inventory::model::*};

pub async fn list_warehouses(db: &PgPool, store_id: Option<i64>) -> AppResult<Vec<Warehouse>> {
    Ok(sqlx::query_as::<_, Warehouse>(
        "SELECT id, store_id, code, name, type FROM warehouses \
         WHERE ($1::bigint IS NULL OR store_id = $1) ORDER BY id",
    )
    .bind(store_id)
    .fetch_all(db)
    .await?)
}

pub async fn create_warehouse(db: &PgPool, w: &CreateWarehouse) -> AppResult<Warehouse> {
    Ok(sqlx::query_as::<_, Warehouse>(
        "INSERT INTO warehouses (store_id, code, name, type) VALUES ($1,$2,$3,COALESCE($4,'store')) \
         RETURNING id, store_id, code, name, type",
    )
    .bind(w.store_id).bind(&w.code).bind(&w.name).bind(&w.r#type)
    .fetch_one(db)
    .await?)
}

pub async fn list_stock(db: &PgPool, warehouse_id: Option<i64>) -> AppResult<Vec<StockRow>> {
    Ok(sqlx::query_as::<_, StockRow>(
        "SELECT sl.warehouse_id, sl.product_id, p.sku, p.name AS product_name, sl.quantity, sl.reorder_point \
         FROM stock_levels sl JOIN products p ON p.id = sl.product_id \
         WHERE ($1::bigint IS NULL OR sl.warehouse_id = $1) ORDER BY sl.warehouse_id, sl.product_id",
    )
    .bind(warehouse_id)
    .fetch_all(db)
    .await?)
}

pub async fn low_stock(db: &PgPool) -> AppResult<Vec<StockRow>> {
    Ok(sqlx::query_as::<_, StockRow>(
        "SELECT sl.warehouse_id, sl.product_id, p.sku, p.name AS product_name, sl.quantity, sl.reorder_point \
         FROM stock_levels sl JOIN products p ON p.id = sl.product_id \
         WHERE sl.quantity < sl.reorder_point ORDER BY sl.warehouse_id, sl.product_id",
    )
    .fetch_all(db)
    .await?)
}

/// 可复用的原子库存变动（P3 调整/调拨、P4 结账扣库存都调它）。
/// 先保证行存在，再条件更新（quantity+delta>=0），0 行=库存不足；同事务写流水。
pub async fn apply_delta(
    tx: &mut Transaction<'_, Postgres>,
    warehouse_id: i64,
    product_id: i64,
    delta: Decimal,
    mtype: &str,
    reference: Option<&str>,
    note: Option<&str>,
    created_by: Option<i64>,
) -> AppResult<()> {
    sqlx::query(
        "INSERT INTO stock_levels (warehouse_id, product_id, quantity) VALUES ($1,$2,0) \
         ON CONFLICT (warehouse_id, product_id) DO NOTHING",
    )
    .bind(warehouse_id).bind(product_id)
    .execute(&mut **tx)
    .await?;

    let r = sqlx::query(
        "UPDATE stock_levels SET quantity = quantity + $3 \
         WHERE warehouse_id = $1 AND product_id = $2 AND quantity + $3 >= 0",
    )
    .bind(warehouse_id).bind(product_id).bind(delta)
    .execute(&mut **tx)
    .await?;
    if r.rows_affected() != 1 {
        return Err(AppError::BadRequest("库存不足".into()));
    }

    sqlx::query(
        "INSERT INTO stock_movements (warehouse_id, product_id, type, quantity, ref, note, created_by) \
         VALUES ($1,$2,$3,$4,$5,$6,$7)",
    )
    .bind(warehouse_id).bind(product_id).bind(mtype).bind(delta).bind(reference).bind(note).bind(created_by)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn adjust(db: &PgPool, req: &AdjustReq, by: i64) -> AppResult<()> {
    let mut tx = db.begin().await?;
    apply_delta(&mut tx, req.warehouse_id, req.product_id, req.quantity_delta, "adjust", None, req.note.as_deref(), Some(by)).await?;
    tx.commit().await?;
    Ok(())
}

pub async fn transfer(db: &PgPool, req: &TransferReq, by: i64) -> AppResult<()> {
    if req.quantity <= Decimal::ZERO {
        return Err(AppError::BadRequest("调拨数量须为正".into()));
    }
    let mut tx = db.begin().await?;
    apply_delta(&mut tx, req.from_warehouse_id, req.product_id, -req.quantity, "transfer", None, req.note.as_deref(), Some(by)).await?;
    apply_delta(&mut tx, req.to_warehouse_id, req.product_id, req.quantity, "transfer", None, req.note.as_deref(), Some(by)).await?;
    tx.commit().await?;
    Ok(())
}
