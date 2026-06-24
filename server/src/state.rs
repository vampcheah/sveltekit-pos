//! 共享应用状态：连接池句柄 + 会话配置。各域 service 通过它访问 db/redis。
use redis::aio::ConnectionManager;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: ConnectionManager,
    pub session_ttl_secs: u64,
    pub session_cookie_name: String,
}
