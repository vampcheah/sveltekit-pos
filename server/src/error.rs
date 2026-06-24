//! 统一错误模型：AppError → HTTP 响应（§1.1 错误模型铁律）。
//! 所有 handler 返回 Result<_, AppError>，错误格式全局一致。
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{0}")]
    BadRequest(String),
    #[error("未认证")]
    Unauthorized,
    #[error("无权限")]
    Forbidden,
    #[error("未找到")]
    NotFound,
    #[error("数据库错误")]
    Db(#[from] sqlx::Error),
    #[error("缓存错误")]
    Redis(#[from] redis::RedisError),
    #[error("{0}")]
    Internal(String),
}

impl AppError {
    fn parts(&self) -> (StatusCode, &'static str) {
        match self {
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, "bad_request"),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "forbidden"),
            AppError::NotFound => (StatusCode::NOT_FOUND, "not_found"),
            AppError::Db(_) | AppError::Redis(_) | AppError::Internal(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal")
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = self.parts();
        // 5xx 记日志，细节不外泄；4xx 把消息回传给前端做 i18n 映射
        if status.is_server_error() {
            tracing::error!(error = %self, "server error");
        }
        let message = if status.is_server_error() { code.to_string() } else { self.to_string() };
        (status, Json(json!({ "error": code, "message": message }))).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
