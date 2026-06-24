//! auth 域业务逻辑：登录校验（锁定/限流）、建会话、组装 me。
use axum_extra::extract::cookie::Cookie;
use chrono::Utc;

use crate::{
    auth::{model::*, repo},
    error::{AppError, AppResult},
    shared::{auth::role_permissions, password, session},
    state::AppState,
};

const LOCK_THRESHOLD: i32 = 5;
const LOCK_SECS: i64 = 900; // 15 分钟

/// admin 登录：成功返回 (会话 cookie, me)。失败一律 Unauthorized，不泄露原因。
pub async fn login_admin(st: &AppState, req: &LoginAdminReq) -> AppResult<(Cookie<'static>, MeResp)> {
    let a = repo::find_admin(&st.db, &req.username).await?;
    let Some(a) = a else {
        return Err(AppError::Unauthorized);
    };
    if a.status != "active" || is_locked(a.locked_until) {
        return Err(AppError::Unauthorized);
    }
    if !password::verify(&req.password, &a.password_hash) {
        repo::record_failure(&st.db, repo::Actor::Admin, a.id, LOCK_THRESHOLD, LOCK_SECS).await?;
        return Err(AppError::Unauthorized);
    }
    repo::record_success(&st.db, repo::Actor::Admin, a.id).await?;

    let sess = session::Session {
        actor_type: "admin".into(),
        actor_id: a.id,
        role_id: a.role_id,
        store_id: None,
        is_supervisor: false,
    };
    let (_sid, cookie) = session::create(st, &sess).await?;
    Ok((cookie, me_from(st, &sess).await?))
}

/// cashier 登录（username + PIN）。
pub async fn login_cashier(st: &AppState, req: &LoginCashierReq) -> AppResult<(Cookie<'static>, MeResp)> {
    let c = repo::find_cashier(&st.db, &req.username).await?;
    let Some(c) = c else {
        return Err(AppError::Unauthorized);
    };
    if c.status != "active" || is_locked(c.locked_until) {
        return Err(AppError::Unauthorized);
    }
    if !password::verify(&req.pin, &c.pin_hash) {
        repo::record_failure(&st.db, repo::Actor::Cashier, c.id, LOCK_THRESHOLD, LOCK_SECS).await?;
        return Err(AppError::Unauthorized);
    }
    repo::record_success(&st.db, repo::Actor::Cashier, c.id).await?;

    let store_id = repo::cashier_home_store(&st.db, c.id).await?;
    let sess = session::Session {
        actor_type: "cashier".into(),
        actor_id: c.id,
        role_id: None,
        store_id,
        is_supervisor: c.is_supervisor,
    };
    let (_sid, cookie) = session::create(st, &sess).await?;
    Ok((cookie, me_from(st, &sess).await?))
}

fn actor_kind(sess: &session::Session) -> repo::Actor {
    if sess.actor_type == "admin" { repo::Actor::Admin } else { repo::Actor::Cashier }
}

pub async fn me_from(st: &AppState, sess: &session::Session) -> AppResult<MeResp> {
    let permissions = match (sess.actor_type.as_str(), sess.role_id) {
        ("admin", Some(role_id)) => role_permissions(st, role_id).await?,
        _ => Vec::new(), // cashier 权限在 P4 结账域定义
    };
    let (username, full_name, avatar_url) =
        repo::display_fields(&st.db, actor_kind(sess), sess.actor_id).await?;
    Ok(MeResp {
        actor_type: sess.actor_type.clone(),
        actor_id: sess.actor_id,
        username,
        full_name,
        avatar_url,
        role_id: sess.role_id,
        store_id: sess.store_id,
        is_supervisor: sess.is_supervisor,
        permissions,
    })
}

/// 自助改密：校验当前凭据 → 写新哈希。
pub async fn change_password(st: &AppState, sess: &session::Session, req: &ChangePasswordReq) -> AppResult<()> {
    if req.new_password.len() < 6 {
        return Err(AppError::BadRequest("新密码过短".into()));
    }
    let actor = actor_kind(sess);
    let hash = repo::credential_hash(&st.db, actor, sess.actor_id).await?.ok_or(AppError::NotFound)?;
    if !password::verify(&req.current_password, &hash) {
        return Err(AppError::BadRequest("当前密码不正确".into()));
    }
    let new_hash = password::hash(&req.new_password)?;
    repo::set_own_credential(&st.db, actor, sess.actor_id, &new_hash).await?;
    Ok(())
}

/// 自助改资料（姓名/头像，仅 admin）。
pub async fn update_profile(st: &AppState, sess: &session::Session, req: &UpdateProfileReq) -> AppResult<MeResp> {
    if sess.actor_type == "admin" {
        repo::update_profile(&st.db, sess.actor_id, req.full_name.as_deref(), req.avatar_url.as_deref()).await?;
    }
    me_from(st, sess).await
}

fn is_locked(locked_until: Option<chrono::DateTime<Utc>>) -> bool {
    matches!(locked_until, Some(t) if t > Utc::now())
}
