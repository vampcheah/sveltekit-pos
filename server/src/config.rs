//! 从环境变量加载配置（dotenvy 在 main 里先 load）。
use std::env;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub db_max_connections: u32,
    pub db_acquire_timeout_secs: u64,
    pub redis_url: String,
    pub cors_origins: Vec<String>,
    pub session_ttl_secs: u64,
    pub session_cookie_name: String,
}

fn var(key: &str) -> Result<String, String> {
    env::var(key).map_err(|_| format!("缺少环境变量 {key}"))
}

fn var_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            host: var_or("APP_HOST", "127.0.0.1"),
            port: var_or("APP_PORT", "8080").parse().map_err(|_| "APP_PORT 非数字")?,
            database_url: var("DATABASE_URL")?,
            db_max_connections: var_or("DB_MAX_CONNECTIONS", "20").parse().unwrap_or(20),
            db_acquire_timeout_secs: var_or("DB_ACQUIRE_TIMEOUT_SECS", "5").parse().unwrap_or(5),
            redis_url: var("REDIS_URL")?,
            cors_origins: var_or("CORS_ORIGINS", "http://localhost:5173,http://localhost:5174")
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
            session_ttl_secs: var_or("SESSION_TTL_SECS", "43200").parse().unwrap_or(43200),
            session_cookie_name: var_or("SESSION_COOKIE_NAME", "pos_sid"),
        })
    }
}
