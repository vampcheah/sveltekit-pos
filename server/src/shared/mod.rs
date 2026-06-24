//! 横切复用层：鉴权 / 会话 / 密码哈希 / 审计。各域 service 与 handler 共享。
pub mod audit;
pub mod auth;
pub mod password;
pub mod session;
