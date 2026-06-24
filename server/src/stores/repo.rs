//! stores 域数据访问。SQL 用字面量（sqlx 0.9 禁止动态 SQL 字符串）。
use sqlx::PgPool;

use crate::{error::AppResult, stores::model::*};

pub async fn list(db: &PgPool) -> AppResult<Vec<Store>> {
    Ok(sqlx::query_as::<_, Store>(
        "SELECT id, name, code, address, phone, timezone, status, created_at \
         FROM stores ORDER BY id",
    )
    .fetch_all(db)
    .await?)
}

pub async fn get(db: &PgPool, id: i64) -> AppResult<Option<Store>> {
    Ok(sqlx::query_as::<_, Store>(
        "SELECT id, name, code, address, phone, timezone, status, created_at \
         FROM stores WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(db)
    .await?)
}

pub async fn create(db: &PgPool, s: &CreateStore) -> AppResult<Store> {
    Ok(sqlx::query_as::<_, Store>(
        "INSERT INTO stores (name, code, address, phone, timezone) \
         VALUES ($1,$2,$3,$4,COALESCE($5,'Asia/Kuala_Lumpur')) \
         RETURNING id, name, code, address, phone, timezone, status, created_at",
    )
    .bind(&s.name).bind(&s.code).bind(&s.address).bind(&s.phone).bind(&s.timezone)
    .fetch_one(db)
    .await?)
}

/// 部分更新（COALESCE：未传字段保持原值）。
pub async fn update(db: &PgPool, id: i64, u: &UpdateStore) -> AppResult<Option<Store>> {
    Ok(sqlx::query_as::<_, Store>(
        "UPDATE stores SET name = COALESCE($2,name), address = COALESCE($3,address), \
         phone = COALESCE($4,phone), status = COALESCE($5,status) WHERE id = $1 \
         RETURNING id, name, code, address, phone, timezone, status, created_at",
    )
    .bind(id).bind(&u.name).bind(&u.address).bind(&u.phone).bind(&u.status)
    .fetch_optional(db)
    .await?)
}
