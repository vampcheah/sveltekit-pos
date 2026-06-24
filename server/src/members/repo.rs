//! members 域数据访问。积分/储值变动走台账，单事务 + 行锁防并发丢失更新。
use rust_decimal::Decimal;
use sqlx::PgPool;

use crate::{error::AppError, error::AppResult, members::model::*};

pub async fn list(db: &PgPool) -> AppResult<Vec<MemberView>> {
    Ok(sqlx::query_as::<_, MemberView>(
        "SELECT id, code, phone, name, email, tier, points, balance, status, created_at \
         FROM members WHERE deleted_at IS NULL ORDER BY id",
    )
    .fetch_all(db)
    .await?)
}

pub async fn get(db: &PgPool, id: i64) -> AppResult<Option<MemberView>> {
    Ok(sqlx::query_as::<_, MemberView>(
        "SELECT id, code, phone, name, email, tier, points, balance, status, created_at \
         FROM members WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(db)
    .await?)
}

pub async fn create(db: &PgPool, m: &CreateMember) -> AppResult<MemberView> {
    Ok(sqlx::query_as::<_, MemberView>(
        "INSERT INTO members (name, phone, code, email, tier) \
         VALUES ($1,$2,$3,$4,COALESCE($5,'regular')) \
         RETURNING id, code, phone, name, email, tier, points, balance, status, created_at",
    )
    .bind(&m.name).bind(&m.phone).bind(&m.code).bind(&m.email).bind(&m.tier)
    .fetch_one(db)
    .await?)
}

pub async fn update(db: &PgPool, id: i64, u: &UpdateMember) -> AppResult<Option<MemberView>> {
    Ok(sqlx::query_as::<_, MemberView>(
        "UPDATE members SET name = COALESCE($2,name), phone = COALESCE($3,phone), \
         email = COALESCE($4,email), tier = COALESCE($5,tier), status = COALESCE($6,status) \
         WHERE id = $1 AND deleted_at IS NULL \
         RETURNING id, code, phone, name, email, tier, points, balance, status, created_at",
    )
    .bind(id).bind(&u.name).bind(&u.phone).bind(&u.email).bind(&u.tier).bind(&u.status)
    .fetch_optional(db)
    .await?)
}

/// 积分/储值原子变动：行锁取当前值 → 校验非负 → 更新快照 + 写台账。
pub async fn apply_ledger(
    db: &PgPool,
    member_id: i64,
    kind: &str,
    points_delta: i64,
    balance_delta: Decimal,
    note: Option<&str>,
    created_by: i64,
) -> AppResult<MemberView> {
    let mut tx = db.begin().await?;
    let cur: Option<(i64, Decimal)> = sqlx::query_as(
        "SELECT points, balance FROM members WHERE id = $1 AND deleted_at IS NULL FOR UPDATE",
    )
    .bind(member_id)
    .fetch_optional(&mut *tx)
    .await?;
    let Some((points, balance)) = cur else {
        return Err(AppError::NotFound);
    };
    let new_points = points + points_delta;
    let new_balance = balance + balance_delta;
    if new_points < 0 {
        return Err(AppError::BadRequest("积分不足".into()));
    }
    if new_balance < Decimal::ZERO {
        return Err(AppError::BadRequest("储值余额不足".into()));
    }

    let view = sqlx::query_as::<_, MemberView>(
        "UPDATE members SET points = $2, balance = $3 WHERE id = $1 \
         RETURNING id, code, phone, name, email, tier, points, balance, status, created_at",
    )
    .bind(member_id).bind(new_points).bind(new_balance)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        "INSERT INTO member_ledger \
         (member_id, kind, points_delta, balance_delta, points_after, balance_after, note, created_by) \
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8)",
    )
    .bind(member_id).bind(kind).bind(points_delta).bind(balance_delta)
    .bind(new_points).bind(new_balance).bind(note).bind(created_by)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(view)
}
