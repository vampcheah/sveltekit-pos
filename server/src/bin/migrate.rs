//! 独立迁移入口（`make migrate`）：连库 → 应用迁移。
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL")?;
    let db = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&url)
        .await?;
    sqlx::migrate!("../database/migrations").run(&db).await?;
    println!("migrations up to date");
    Ok(())
}
