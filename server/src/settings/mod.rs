//! settings 域：系统设置 KV（税率/货币/小票模板/营业参数）。税率/货币结账依赖。
use axum::{extract::State, routing::get, Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    error::AppResult,
    shared::auth::{require_permission, AdminActor},
    state::AppState,
};

#[derive(Serialize, sqlx::FromRow)]
struct Setting {
    key: String,
    value: Value,
    updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
struct PutSetting {
    key: String,
    value: Value,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/settings", get(list).put(put))
}

async fn list(State(st): State<AppState>, _a: AdminActor) -> AppResult<Json<Vec<Setting>>> {
    Ok(Json(
        sqlx::query_as::<_, Setting>("SELECT key, value, updated_at FROM settings ORDER BY key")
            .fetch_all(&st.db)
            .await?,
    ))
}

async fn put(State(st): State<AppState>, a: AdminActor, Json(body): Json<PutSetting>) -> AppResult<Json<Setting>> {
    require_permission(&st, &a.0, "settings.write").await?;
    Ok(Json(
        sqlx::query_as::<_, Setting>(
            "INSERT INTO settings (key, value, updated_by, updated_at) VALUES ($1,$2,$3,now()) \
             ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value, updated_by = EXCLUDED.updated_by, updated_at = now() \
             RETURNING key, value, updated_at",
        )
        .bind(&body.key)
        .bind(&body.value)
        .bind(a.0.session.actor_id)
        .fetch_one(&st.db)
        .await?,
    ))
}
