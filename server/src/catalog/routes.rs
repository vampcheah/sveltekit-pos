//! catalog 域路由。读对任意登录者开放（pos 也要读）；写要 products.write，改价另需 products.price.write。
use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use serde::Deserialize;

use crate::{
    catalog::{model::*, repo},
    error::{AppError, AppResult},
    shared::auth::{require_permission, AdminActor, CurrentActor},
    state::AppState,
};

#[derive(Deserialize)]
struct ProductQuery {
    q: Option<String>,
    category_id: Option<i64>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/categories", get(list_categories).post(create_category))
        .route("/products", get(list_products).post(create_product))
        .route("/products/{id}", get(get_product).patch(update_product).delete(delete_product))
}

async fn list_categories(State(st): State<AppState>, _a: CurrentActor) -> AppResult<Json<Vec<Category>>> {
    Ok(Json(repo::list_categories(&st.db).await?))
}

async fn create_category(State(st): State<AppState>, a: AdminActor, Json(body): Json<CreateCategory>) -> AppResult<Json<Category>> {
    require_permission(&st, &a.0, "products.write").await?;
    Ok(Json(repo::create_category(&st.db, &body).await?))
}

async fn list_products(State(st): State<AppState>, _a: CurrentActor, Query(q): Query<ProductQuery>) -> AppResult<Json<Vec<Product>>> {
    Ok(Json(repo::list_products(&st.db, q.q.as_deref(), q.category_id).await?))
}

async fn get_product(State(st): State<AppState>, _a: CurrentActor, Path(id): Path<i64>) -> AppResult<Json<Product>> {
    repo::get_product(&st.db, id).await?.map(Json).ok_or(AppError::NotFound)
}

async fn create_product(State(st): State<AppState>, a: AdminActor, Json(body): Json<CreateProduct>) -> AppResult<Json<Product>> {
    require_permission(&st, &a.0, "products.write").await?;
    Ok(Json(repo::create_product(&st.db, &body).await?))
}

async fn update_product(State(st): State<AppState>, a: AdminActor, Path(id): Path<i64>, Json(body): Json<UpdateProduct>) -> AppResult<Json<Product>> {
    require_permission(&st, &a.0, "products.write").await?;
    if body.price.is_some() {
        require_permission(&st, &a.0, "products.price.write").await?; // 改价分权
    }
    repo::update_product(&st.db, id, &body).await?.map(Json).ok_or(AppError::NotFound)
}

async fn delete_product(State(st): State<AppState>, a: AdminActor, Path(id): Path<i64>) -> AppResult<Json<serde_json::Value>> {
    require_permission(&st, &a.0, "products.write").await?;
    if !repo::soft_delete_product(&st.db, id).await? {
        return Err(AppError::NotFound);
    }
    Ok(Json(serde_json::json!({ "ok": true })))
}
