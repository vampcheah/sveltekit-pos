//! pos-server 入口：配置 → 连接池 → 路由 → 优雅停机。
use std::time::Duration;

use pos_server::{config::Config, routes, state::AppState};
use sqlx::postgres::PgPoolOptions;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_env("RUST_LOG").unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = Config::from_env().map_err(|e| {
        tracing::error!("{e}");
        e
    })?;

    // 连接池：显式上限 + 超时（共享 postgres，max_connections=200 全机共用，§1.0）
    let db = PgPoolOptions::new()
        .max_connections(cfg.db_max_connections)
        .acquire_timeout(Duration::from_secs(cfg.db_acquire_timeout_secs))
        .connect(&cfg.database_url)
        .await?;
    tracing::info!("postgres connected (max={})", cfg.db_max_connections);

    // 内嵌迁移：启动即自动应用未执行的迁移（dev 自愈，无需外部 sqlx-cli）
    sqlx::migrate!("../database/migrations").run(&db).await?;
    tracing::info!("migrations up to date");

    let redis_client = redis::Client::open(cfg.redis_url.as_str())?;
    let redis = redis::aio::ConnectionManager::new(redis_client).await?;
    tracing::info!("redis connected");

    let state = AppState {
        db,
        redis,
        session_ttl_secs: cfg.session_ttl_secs,
        session_cookie_name: cfg.session_cookie_name.clone(),
    };
    let cors = build_cors(&cfg.cors_origins);
    let app = routes::router(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let addr = format!("{}:{}", cfg.host, cfg.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("listening on http://{addr}");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

/// SIGTERM/Ctrl-C 优雅停机：不截断进行中的请求/事务（§1.1 横切）。
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("install Ctrl-C handler");
    };
    #[cfg(unix)]
    let term = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("install SIGTERM handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let term = std::future::pending::<()>();

    tokio::select! { _ = ctrl_c => {}, _ = term => {} }
    tracing::info!("shutting down gracefully");
}

fn build_cors(origins: &[String]) -> CorsLayer {
    use axum::http::{HeaderValue, Method};
    let list: Vec<HeaderValue> = origins.iter().filter_map(|o| o.parse().ok()).collect();
    CorsLayer::new()
        .allow_origin(list) // 显式白名单，绝不反射 Origin（§3 铁律 6）
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
        .allow_headers([axum::http::header::CONTENT_TYPE, axum::http::header::AUTHORIZATION])
        .allow_credentials(true)
}
