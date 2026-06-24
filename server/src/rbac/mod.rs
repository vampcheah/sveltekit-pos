//! rbac 域：角色列表、权限列表、给角色配权限（改后失效 perms 缓存 → 撤权实时）。
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    error::AppResult,
    shared::auth::{invalidate_role_cache, require_permission, AdminActor},
    state::AppState,
};

#[derive(Serialize, sqlx::FromRow)]
struct Role {
    id: i64,
    code: String,
    name: String,
    description: Option<String>,
}

#[derive(Serialize, sqlx::FromRow)]
struct Permission {
    id: i64,
    code: String,
    name: String,
    group_: String,
}

#[derive(Deserialize)]
struct SetPermissions {
    permissions: Vec<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/roles", get(list_roles))
        .route("/permissions", get(list_permissions))
        .route("/roles/{id}/permissions", get(role_permissions).put(set_permissions))
}

async fn list_roles(State(st): State<AppState>, a: AdminActor) -> AppResult<Json<Vec<Role>>> {
    require_permission(&st, &a.0, "roles.write").await?;
    Ok(Json(
        sqlx::query_as::<_, Role>("SELECT id, code, name, description FROM roles ORDER BY id")
            .fetch_all(&st.db)
            .await?,
    ))
}

async fn list_permissions(State(st): State<AppState>, a: AdminActor) -> AppResult<Json<Vec<Permission>>> {
    require_permission(&st, &a.0, "roles.write").await?;
    Ok(Json(
        sqlx::query_as::<_, Permission>("SELECT id, code, name, group_ FROM permissions ORDER BY group_, code")
            .fetch_all(&st.db)
            .await?,
    ))
}

async fn role_permissions(State(st): State<AppState>, a: AdminActor, Path(id): Path<i64>) -> AppResult<Json<Vec<String>>> {
    require_permission(&st, &a.0, "roles.write").await?;
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT p.code FROM role_permissions rp JOIN permissions p ON p.id = rp.permission_id \
         WHERE rp.role_id = $1 ORDER BY p.code",
    )
    .bind(id)
    .fetch_all(&st.db)
    .await?;
    Ok(Json(rows.into_iter().map(|r| r.0).collect()))
}

async fn set_permissions(State(st): State<AppState>, a: AdminActor, Path(id): Path<i64>, Json(body): Json<SetPermissions>) -> AppResult<Json<Vec<String>>> {
    require_permission(&st, &a.0, "roles.write").await?;
    let mut tx = st.db.begin().await?;
    sqlx::query("DELETE FROM role_permissions WHERE role_id = $1").bind(id).execute(&mut *tx).await?;
    sqlx::query(
        "INSERT INTO role_permissions (role_id, permission_id) \
         SELECT $1, id FROM permissions WHERE code = ANY($2)",
    )
    .bind(id)
    .bind(&body.permissions)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    invalidate_role_cache(&st, id).await?; // 撤权实时
    Ok(Json(body.permissions))
}
