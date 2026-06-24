//! 鉴权抽取器与 RBAC（§3 铁律 1/deny-by-default）。所有受保护 handler 复用。
//! - `CurrentActor`：从 cookie 解析会话，无则 401。
//! - `AdminActor` / `CashierActor`：强制 actor_type，跨类访问即 403。
//! - `require_permission`：admin 按 role 校验权限，deny-by-default。
use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};
use axum_extra::extract::CookieJar;
use redis::AsyncCommands;

use crate::{
    error::AppError,
    shared::session::{self, Session},
    state::AppState,
};

/// 已认证 actor（admin 或 cashier）。
pub struct CurrentActor {
    pub sid: String,
    pub session: Session,
}

impl FromRequestParts<AppState> for CurrentActor {
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, st: &AppState) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);
        let sid = jar
            .get(&st.session_cookie_name)
            .map(|c| c.value().to_string())
            .ok_or(AppError::Unauthorized)?;
        let session = session::get(st, &sid).await?.ok_or(AppError::Unauthorized)?;
        Ok(CurrentActor { sid, session })
    }
}

/// 仅 admin。
pub struct AdminActor(pub CurrentActor);
impl FromRequestParts<AppState> for AdminActor {
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, st: &AppState) -> Result<Self, Self::Rejection> {
        let actor = CurrentActor::from_request_parts(parts, st).await?;
        if actor.session.actor_type != "admin" {
            return Err(AppError::Forbidden);
        }
        Ok(AdminActor(actor))
    }
}

/// 仅 cashier（含主管）。
pub struct CashierActor(pub CurrentActor);
impl FromRequestParts<AppState> for CashierActor {
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, st: &AppState) -> Result<Self, Self::Rejection> {
        let actor = CurrentActor::from_request_parts(parts, st).await?;
        if actor.session.actor_type != "cashier" {
            return Err(AppError::Forbidden);
        }
        Ok(CashierActor(actor))
    }
}

/// 加载某角色的权限码集合（Redis 缓存，改角色权限时失效该 key → 撤权实时）。
pub async fn role_permissions(st: &AppState, role_id: i64) -> Result<Vec<String>, AppError> {
    let mut redis = st.redis.clone();
    let cache_key = format!("perms:role:{role_id}");
    let cached: Option<String> = redis.get(&cache_key).await?;
    if let Some(raw) = cached {
        if let Ok(v) = serde_json::from_str::<Vec<String>>(&raw) {
            return Ok(v);
        }
    }
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT p.code FROM role_permissions rp \
         JOIN permissions p ON p.id = rp.permission_id WHERE rp.role_id = $1",
    )
    .bind(role_id)
    .fetch_all(&st.db)
    .await?;
    let perms: Vec<String> = rows.into_iter().map(|r| r.0).collect();
    let _: () = redis
        .set_ex(&cache_key, serde_json::to_string(&perms).unwrap(), 300)
        .await?;
    Ok(perms)
}

pub async fn invalidate_role_cache(st: &AppState, role_id: i64) -> Result<(), AppError> {
    let mut redis = st.redis.clone();
    let _: () = redis.del(format!("perms:role:{role_id}")).await?;
    Ok(())
}

/// admin 权限校验：deny-by-default。无角色或无该 permission → 403。
pub async fn require_permission(
    st: &AppState,
    actor: &CurrentActor,
    permission: &str,
) -> Result<(), AppError> {
    let role_id = actor.session.role_id.ok_or(AppError::Forbidden)?;
    let perms = role_permissions(st, role_id).await?;
    if perms.iter().any(|p| p == permission) {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}
