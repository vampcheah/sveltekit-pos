//! auth 域路由（薄 handler：解析 → service → 映射响应）。
use axum::{extract::State, routing::{get, patch, post}, Json, Router};
use axum_extra::extract::CookieJar;
use serde_json::{json, Value};

use crate::{
    auth::{model::*, service},
    error::AppResult,
    shared::{auth::CurrentActor, session},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/admin/login", post(login_admin))
        .route("/auth/cashier/login", post(login_cashier))
        .route("/auth/logout", post(logout))
        .route("/auth/me", get(me))
        .route("/auth/change-password", post(change_password))
        .route("/auth/profile", patch(update_profile))
}

async fn login_admin(
    State(st): State<AppState>,
    jar: CookieJar,
    Json(req): Json<LoginAdminReq>,
) -> AppResult<(CookieJar, Json<MeResp>)> {
    let (cookie, me) = service::login_admin(&st, &req).await?;
    Ok((jar.add(cookie), Json(me)))
}

async fn login_cashier(
    State(st): State<AppState>,
    jar: CookieJar,
    Json(req): Json<LoginCashierReq>,
) -> AppResult<(CookieJar, Json<MeResp>)> {
    let (cookie, me) = service::login_cashier(&st, &req).await?;
    Ok((jar.add(cookie), Json(me)))
}

async fn logout(
    State(st): State<AppState>,
    actor: CurrentActor,
    jar: CookieJar,
) -> AppResult<(CookieJar, Json<Value>)> {
    session::destroy(&st, &actor.sid).await?;
    Ok((jar.add(session::clear_cookie(&st)), Json(json!({ "ok": true }))))
}

async fn me(State(st): State<AppState>, actor: CurrentActor) -> AppResult<Json<MeResp>> {
    Ok(Json(service::me_from(&st, &actor.session).await?))
}

async fn change_password(
    State(st): State<AppState>,
    actor: CurrentActor,
    Json(req): Json<ChangePasswordReq>,
) -> AppResult<Json<Value>> {
    service::change_password(&st, &actor.session, &req).await?;
    Ok(Json(json!({ "ok": true })))
}

async fn update_profile(
    State(st): State<AppState>,
    actor: CurrentActor,
    Json(req): Json<UpdateProfileReq>,
) -> AppResult<Json<MeResp>> {
    Ok(Json(service::update_profile(&st, &actor.session, &req).await?))
}
