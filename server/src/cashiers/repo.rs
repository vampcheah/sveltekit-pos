//! cashiers 域数据访问。建/改门店分配在事务内（一致性）。
use sqlx::{PgPool, Postgres, Transaction};

use crate::{cashiers::model::*, error::AppResult};

pub async fn list(db: &PgPool) -> AppResult<Vec<CashierView>> {
    Ok(sqlx::query_as::<_, CashierView>(
        "SELECT id, username, full_name, is_supervisor, status, last_login_at, created_at \
         FROM cashiers ORDER BY id",
    )
    .fetch_all(db)
    .await?)
}

pub async fn get(db: &PgPool, id: i64) -> AppResult<Option<CashierView>> {
    Ok(sqlx::query_as::<_, CashierView>(
        "SELECT id, username, full_name, is_supervisor, status, last_login_at, created_at \
         FROM cashiers WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(db)
    .await?)
}

pub async fn store_ids(db: &PgPool, id: i64) -> AppResult<Vec<i64>> {
    let rows: Vec<(i64,)> = sqlx::query_as(
        "SELECT store_id FROM cashier_stores WHERE cashier_id = $1 ORDER BY store_id",
    )
    .bind(id)
    .fetch_all(db)
    .await?;
    Ok(rows.into_iter().map(|r| r.0).collect())
}

async fn replace_stores(
    tx: &mut Transaction<'_, Postgres>,
    cashier_id: i64,
    store_ids: &[i64],
    home: Option<i64>,
) -> AppResult<()> {
    sqlx::query("DELETE FROM cashier_stores WHERE cashier_id = $1")
        .bind(cashier_id)
        .execute(&mut **tx)
        .await?;
    for &sid in store_ids {
        sqlx::query("INSERT INTO cashier_stores (cashier_id, store_id, is_home) VALUES ($1,$2,$3)")
            .bind(cashier_id)
            .bind(sid)
            .bind(Some(sid) == home)
            .execute(&mut **tx)
            .await?;
    }
    Ok(())
}

pub async fn create(db: &PgPool, c: &CreateCashier, pin_hash: &str) -> AppResult<CashierView> {
    let mut tx = db.begin().await?;
    let view = sqlx::query_as::<_, CashierView>(
        "INSERT INTO cashiers (username, pin_hash, full_name, is_supervisor, status) \
         VALUES ($1,$2,$3,COALESCE($4,false),'active') \
         RETURNING id, username, full_name, is_supervisor, status, last_login_at, created_at",
    )
    .bind(&c.username).bind(pin_hash).bind(&c.full_name).bind(c.is_supervisor)
    .fetch_one(&mut *tx)
    .await?;
    replace_stores(&mut tx, view.id, &c.store_ids, c.home_store_id).await?;
    tx.commit().await?;
    Ok(view)
}

pub async fn update(db: &PgPool, id: i64, u: &UpdateCashier) -> AppResult<Option<CashierView>> {
    let mut tx = db.begin().await?;
    let view = sqlx::query_as::<_, CashierView>(
        "UPDATE cashiers SET full_name = COALESCE($2,full_name), \
         is_supervisor = COALESCE($3,is_supervisor) WHERE id = $1 \
         RETURNING id, username, full_name, is_supervisor, status, last_login_at, created_at",
    )
    .bind(id).bind(&u.full_name).bind(u.is_supervisor)
    .fetch_optional(&mut *tx)
    .await?;
    let Some(view) = view else {
        return Ok(None);
    };
    if let Some(ref sids) = u.store_ids {
        replace_stores(&mut tx, id, sids, u.home_store_id).await?;
    }
    tx.commit().await?;
    Ok(Some(view))
}

pub async fn set_pin(db: &PgPool, id: i64, pin_hash: &str) -> AppResult<bool> {
    let r = sqlx::query("UPDATE cashiers SET pin_hash = $2, must_change_password = true WHERE id = $1")
        .bind(id).bind(pin_hash)
        .execute(db)
        .await?;
    Ok(r.rows_affected() == 1)
}

pub async fn set_status(db: &PgPool, id: i64, disable: bool) -> AppResult<bool> {
    let r = sqlx::query(
        "UPDATE cashiers SET status = CASE WHEN $2 THEN 'disabled' ELSE 'active' END, \
         disabled_at = CASE WHEN $2 THEN now() ELSE NULL END WHERE id = $1",
    )
    .bind(id).bind(disable)
    .execute(db)
    .await?;
    Ok(r.rows_affected() == 1)
}

pub async fn unlock(db: &PgPool, id: i64) -> AppResult<bool> {
    let r = sqlx::query("UPDATE cashiers SET locked_until = NULL, failed_login_count = 0 WHERE id = $1")
        .bind(id)
        .execute(db)
        .await?;
    Ok(r.rows_affected() == 1)
}
