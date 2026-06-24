//! 服务端会话：sid → Redis(JSON)。即时登出=删 key，撤权实时生效，XSS 偷不到。
//! 同时维护 sessions:by_actor:<type>:<id> 集合，支持"踢线"（revoke-all）。
use axum_extra::extract::cookie::{Cookie, SameSite};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::AppResult, state::AppState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub actor_type: String, // "admin" | "cashier"
    pub actor_id: i64,
    pub role_id: Option<i64>,    // admin 的角色
    pub store_id: Option<i64>,   // cashier 的主店
    pub is_supervisor: bool,     // cashier 主管层
}

fn skey(sid: &str) -> String {
    format!("session:{sid}")
}
fn actor_key(actor_type: &str, actor_id: i64) -> String {
    format!("sessions:by_actor:{actor_type}:{actor_id}")
}

/// 创建会话，返回 (sid, cookie)。
pub async fn create(st: &AppState, s: &Session) -> AppResult<(String, Cookie<'static>)> {
    let sid = Uuid::new_v4().to_string();
    let mut redis = st.redis.clone();
    let payload = serde_json::to_string(s).expect("session serialize");
    let ttl = st.session_ttl_secs;
    let _: () = redis.set_ex(skey(&sid), payload, ttl).await?;
    let _: () = redis.sadd(actor_key(&s.actor_type, s.actor_id), &sid).await?;
    Ok((sid.clone(), build_cookie(st, sid)))
}

pub async fn get(st: &AppState, sid: &str) -> AppResult<Option<Session>> {
    let mut redis = st.redis.clone();
    let raw: Option<String> = redis.get(skey(sid)).await?;
    Ok(raw.and_then(|r| serde_json::from_str(&r).ok()))
}

pub async fn destroy(st: &AppState, sid: &str) -> AppResult<()> {
    let mut redis = st.redis.clone();
    if let Some(s) = get(st, sid).await? {
        let _: () = redis.srem(actor_key(&s.actor_type, s.actor_id), sid).await?;
    }
    let _: () = redis.del(skey(sid)).await?;
    Ok(())
}

/// 踢线：删除某 actor 的全部会话（禁用/改密/改角色时调用）。
pub async fn revoke_all(st: &AppState, actor_type: &str, actor_id: i64) -> AppResult<()> {
    let mut redis = st.redis.clone();
    let sids: Vec<String> = redis.smembers(actor_key(actor_type, actor_id)).await?;
    for sid in &sids {
        let _: () = redis.del(skey(sid)).await?;
    }
    let _: () = redis.del(actor_key(actor_type, actor_id)).await?;
    Ok(())
}

fn build_cookie(st: &AppState, sid: String) -> Cookie<'static> {
    Cookie::build((st.session_cookie_name.clone(), sid))
        .http_only(true)       // JS 读不到
        .secure(false)         // dev=http；生产置 true
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(time::Duration::seconds(st.session_ttl_secs as i64))
        .build()
}

pub fn clear_cookie(st: &AppState) -> Cookie<'static> {
    Cookie::build((st.session_cookie_name.clone(), ""))
        .path("/")
        .max_age(time::Duration::seconds(0))
        .build()
}
