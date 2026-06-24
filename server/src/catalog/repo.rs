//! catalog 域数据访问。
use sqlx::PgPool;

use crate::{catalog::model::*, error::AppResult};

// --- 分类 ---
pub async fn list_categories(db: &PgPool) -> AppResult<Vec<Category>> {
    Ok(sqlx::query_as::<_, Category>("SELECT id, code, name, sort, parent_id FROM categories ORDER BY sort, id")
        .fetch_all(db).await?)
}

pub async fn create_category(db: &PgPool, c: &CreateCategory) -> AppResult<Category> {
    Ok(sqlx::query_as::<_, Category>(
        "INSERT INTO categories (code, name, sort, parent_id) VALUES ($1,$2,COALESCE($3,0),$4) \
         RETURNING id, code, name, sort, parent_id",
    )
    .bind(&c.code).bind(&c.name).bind(c.sort).bind(c.parent_id)
    .fetch_one(db).await?)
}

// --- 商品 ---
pub async fn list_products(db: &PgPool, q: Option<&str>, category_id: Option<i64>) -> AppResult<Vec<Product>> {
    // 单查询：q 为空时匹配全部；category 同理。$2/$3 用 NULL 短路。
    Ok(sqlx::query_as::<_, Product>(
        "SELECT id, sku, barcode, name, category_id, price, cost, unit, is_weighted, tax_rate, tax_category, image_url, status, created_at \
         FROM products WHERE deleted_at IS NULL \
           AND ($1::text IS NULL OR name ILIKE '%'||$1||'%' OR sku ILIKE '%'||$1||'%' OR barcode = $1) \
           AND ($2::bigint IS NULL OR category_id = $2) \
         ORDER BY id",
    )
    .bind(q).bind(category_id)
    .fetch_all(db).await?)
}

pub async fn get_product(db: &PgPool, id: i64) -> AppResult<Option<Product>> {
    Ok(sqlx::query_as::<_, Product>(
        "SELECT id, sku, barcode, name, category_id, price, cost, unit, is_weighted, tax_rate, tax_category, image_url, status, created_at \
         FROM products WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id).fetch_optional(db).await?)
}

pub async fn create_product(db: &PgPool, p: &CreateProduct) -> AppResult<Product> {
    Ok(sqlx::query_as::<_, Product>(
        "INSERT INTO products (sku, name, price, barcode, category_id, cost, unit, is_weighted, tax_rate, tax_category, image_url) \
         VALUES ($1,$2,$3,$4,$5,COALESCE($6,0),$7,COALESCE($8,false),COALESCE($9,0),$10,$11) \
         RETURNING id, sku, barcode, name, category_id, price, cost, unit, is_weighted, tax_rate, tax_category, image_url, status, created_at",
    )
    .bind(&p.sku).bind(&p.name).bind(p.price).bind(&p.barcode).bind(p.category_id)
    .bind(p.cost).bind(&p.unit).bind(p.is_weighted).bind(p.tax_rate).bind(&p.tax_category).bind(&p.image_url)
    .fetch_one(db).await?)
}

pub async fn update_product(db: &PgPool, id: i64, u: &UpdateProduct) -> AppResult<Option<Product>> {
    Ok(sqlx::query_as::<_, Product>(
        "UPDATE products SET name = COALESCE($2,name), price = COALESCE($3,price), cost = COALESCE($4,cost), \
         barcode = COALESCE($5,barcode), category_id = COALESCE($6,category_id), unit = COALESCE($7,unit), \
         tax_rate = COALESCE($8,tax_rate), status = COALESCE($9,status), image_url = COALESCE($10,image_url), updated_at = now() \
         WHERE id = $1 AND deleted_at IS NULL \
         RETURNING id, sku, barcode, name, category_id, price, cost, unit, is_weighted, tax_rate, tax_category, image_url, status, created_at",
    )
    .bind(id).bind(&u.name).bind(u.price).bind(u.cost).bind(&u.barcode).bind(u.category_id)
    .bind(&u.unit).bind(u.tax_rate).bind(&u.status).bind(&u.image_url)
    .fetch_optional(db).await?)
}

pub async fn soft_delete_product(db: &PgPool, id: i64) -> AppResult<bool> {
    let r = sqlx::query("UPDATE products SET deleted_at = now() WHERE id = $1 AND deleted_at IS NULL")
        .bind(id).execute(db).await?;
    Ok(r.rows_affected() == 1)
}
