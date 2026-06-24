//! auth 域数据访问：admins / cashiers 登录所需读写。
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::error::AppResult;

#[derive(sqlx::FromRow)]
pub struct AdminAuth {
    pub id: i64,
    pub password_hash: String,
    pub role_id: Option<i64>,
    pub status: String,
    pub failed_login_count: i32,
    pub locked_until: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow)]
pub struct CashierAuth {
    pub id: i64,
    pub pin_hash: String,
    pub status: String,
    pub failed_login_count: i32,
    pub locked_until: Option<DateTime<Utc>>,
    pub is_supervisor: bool,
}

pub async fn find_admin(db: &PgPool, username: &str) -> AppResult<Option<AdminAuth>> {
    Ok(sqlx::query_as::<_, AdminAuth>(
        "SELECT id, password_hash, role_id, status, failed_login_count, locked_until \
         FROM admins WHERE username = $1",
    )
    .bind(username)
    .fetch_optional(db)
    .await?)
}

pub async fn find_cashier(db: &PgPool, username: &str) -> AppResult<Option<CashierAuth>> {
    Ok(sqlx::query_as::<_, CashierAuth>(
        "SELECT id, pin_hash, status, failed_login_count, locked_until, is_supervisor \
         FROM cashiers WHERE username = $1",
    )
    .bind(username)
    .fetch_optional(db)
    .await?)
}

pub async fn cashier_home_store(db: &PgPool, cashier_id: i64) -> AppResult<Option<i64>> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT store_id FROM cashier_stores WHERE cashier_id = $1 \
         ORDER BY is_home DESC LIMIT 1",
    )
    .bind(cashier_id)
    .fetch_optional(db)
    .await?;
    Ok(row.map(|r| r.0))
}

/// 账号类型：决定操作哪张表（字面量 SQL，非拼接，无注入面）。
#[derive(Clone, Copy)]
pub enum Actor {
    Admin,
    Cashier,
}

/// 登录失败：自增计数，达阈值则锁定。
pub async fn record_failure(
    db: &PgPool,
    actor: Actor,
    id: i64,
    threshold: i32,
    lock_secs: i64,
) -> AppResult<()> {
    let sql = match actor {
        Actor::Admin => "UPDATE admins SET failed_login_count = failed_login_count + 1, \
            locked_until = CASE WHEN failed_login_count + 1 >= $2 \
               THEN now() + ($3 || ' seconds')::interval ELSE locked_until END WHERE id = $1",
        Actor::Cashier => "UPDATE cashiers SET failed_login_count = failed_login_count + 1, \
            locked_until = CASE WHEN failed_login_count + 1 >= $2 \
               THEN now() + ($3 || ' seconds')::interval ELSE locked_until END WHERE id = $1",
    };
    sqlx::query(sql)
        .bind(id)
        .bind(threshold)
        .bind(lock_secs.to_string())
        .execute(db)
        .await?;
    Ok(())
}

/// 个人资料显示字段（用户名/姓名/头像）。
pub async fn display_fields(
    db: &PgPool,
    actor: Actor,
    id: i64,
) -> AppResult<(Option<String>, Option<String>, Option<String>)> {
    // (username, full_name, avatar_url)
    let row: Option<(String, Option<String>, Option<String>)> = match actor {
        Actor::Admin => {
            sqlx::query_as("SELECT username, full_name, avatar_url FROM admins WHERE id = $1")
                .bind(id).fetch_optional(db).await?
        }
        Actor::Cashier => {
            sqlx::query_as("SELECT username, full_name, NULL::text FROM cashiers WHERE id = $1")
                .bind(id).fetch_optional(db).await?
        }
    };
    Ok(row.map(|(u, f, a)| (Some(u), f, a)).unwrap_or((None, None, None)))
}

/// 取当前凭据哈希（自助改密校验用）。admin=password_hash，cashier=pin_hash。
pub async fn credential_hash(db: &PgPool, actor: Actor, id: i64) -> AppResult<Option<String>> {
    let col_sql = match actor {
        Actor::Admin => "SELECT password_hash FROM admins WHERE id = $1",
        Actor::Cashier => "SELECT pin_hash FROM cashiers WHERE id = $1",
    };
    let row: Option<(String,)> = sqlx::query_as(col_sql).bind(id).fetch_optional(db).await?;
    Ok(row.map(|r| r.0))
}

/// 自助改密：写新哈希（admin→password_hash，cashier→pin_hash），清 must_change。
pub async fn set_own_credential(db: &PgPool, actor: Actor, id: i64, hash: &str) -> AppResult<()> {
    let sql = match actor {
        Actor::Admin => "UPDATE admins SET password_hash = $2, must_change_password = false WHERE id = $1",
        Actor::Cashier => "UPDATE cashiers SET pin_hash = $2, must_change_password = false WHERE id = $1",
    };
    sqlx::query(sql).bind(id).bind(hash).execute(db).await?;
    Ok(())
}

/// 自助改资料（仅 admin 有头像）。
pub async fn update_profile(
    db: &PgPool,
    id: i64,
    full_name: Option<&str>,
    avatar_url: Option<&str>,
) -> AppResult<()> {
    sqlx::query(
        "UPDATE admins SET full_name = COALESCE($2, full_name), avatar_url = COALESCE($3, avatar_url) WHERE id = $1",
    )
    .bind(id).bind(full_name).bind(avatar_url)
    .execute(db).await?;
    Ok(())
}

/// 登录成功：清零计数、解锁、更新 last_login。
pub async fn record_success(db: &PgPool, actor: Actor, id: i64) -> AppResult<()> {
    let sql = match actor {
        Actor::Admin => "UPDATE admins SET failed_login_count = 0, locked_until = NULL, \
            last_login_at = now() WHERE id = $1",
        Actor::Cashier => "UPDATE cashiers SET failed_login_count = 0, locked_until = NULL, \
            last_login_at = now() WHERE id = $1",
    };
    sqlx::query(sql).bind(id).execute(db).await?;
    Ok(())
}
