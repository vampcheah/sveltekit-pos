//! admins 域数据访问。
use sqlx::PgPool;

use crate::{admins::model::*, error::AppResult};

pub async fn list(db: &PgPool) -> AppResult<Vec<AdminView>> {
    Ok(sqlx::query_as::<_, AdminView>(
        "SELECT id, username, email, full_name, role_id, status, must_change_password, last_login_at, created_at \
         FROM admins ORDER BY id",
    )
    .fetch_all(db)
    .await?)
}

pub async fn get(db: &PgPool, id: i64) -> AppResult<Option<AdminView>> {
    Ok(sqlx::query_as::<_, AdminView>(
        "SELECT id, username, email, full_name, role_id, status, must_change_password, last_login_at, created_at \
         FROM admins WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(db)
    .await?)
}

/// 新建：密码已哈希；强制首登改密。
pub async fn create(db: &PgPool, a: &CreateAdmin, password_hash: &str) -> AppResult<AdminView> {
    Ok(sqlx::query_as::<_, AdminView>(
        "INSERT INTO admins (username, password_hash, full_name, email, role_id, must_change_password, status) \
         VALUES ($1,$2,$3,$4,$5,true,'active') \
         RETURNING id, username, email, full_name, role_id, status, must_change_password, last_login_at, created_at",
    )
    .bind(&a.username).bind(password_hash).bind(&a.full_name).bind(&a.email).bind(&a.role_id)
    .fetch_one(db)
    .await?)
}

pub async fn update(db: &PgPool, id: i64, u: &UpdateAdmin) -> AppResult<Option<AdminView>> {
    Ok(sqlx::query_as::<_, AdminView>(
        "UPDATE admins SET full_name = COALESCE($2,full_name), email = COALESCE($3,email), \
         role_id = COALESCE($4,role_id) WHERE id = $1 \
         RETURNING id, username, email, full_name, role_id, status, must_change_password, last_login_at, created_at",
    )
    .bind(id).bind(&u.full_name).bind(&u.email).bind(&u.role_id)
    .fetch_optional(db)
    .await?)
}

/// 重置密码：强制下次登录改密。返回是否命中。
pub async fn set_password(db: &PgPool, id: i64, password_hash: &str) -> AppResult<bool> {
    let r = sqlx::query(
        "UPDATE admins SET password_hash = $2, must_change_password = true WHERE id = $1",
    )
    .bind(id).bind(password_hash)
    .execute(db)
    .await?;
    Ok(r.rows_affected() == 1)
}

pub async fn set_status(db: &PgPool, id: i64, disable: bool) -> AppResult<bool> {
    let r = sqlx::query(
        "UPDATE admins SET status = CASE WHEN $2 THEN 'disabled' ELSE 'active' END, \
         disabled_at = CASE WHEN $2 THEN now() ELSE NULL END WHERE id = $1",
    )
    .bind(id).bind(disable)
    .execute(db)
    .await?;
    Ok(r.rows_affected() == 1)
}

pub async fn unlock(db: &PgPool, id: i64) -> AppResult<bool> {
    let r = sqlx::query("UPDATE admins SET locked_until = NULL, failed_login_count = 0 WHERE id = $1")
        .bind(id)
        .execute(db)
        .await?;
    Ok(r.rows_affected() == 1)
}
