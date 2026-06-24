//! Argon2id 密码/PIN 哈希。明文永不落库。
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;

use crate::error::{AppError, AppResult};

pub fn hash(plain: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(plain.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AppError::Internal(format!("hash: {e}")))
}

/// 校验失败返回 false（包括哈希解析失败），不区分原因以免信息泄露。
pub fn verify(plain: &str, hashed: &str) -> bool {
    match PasswordHash::new(hashed) {
        Ok(parsed) => Argon2::default()
            .verify_password(plain.as_bytes(), &parsed)
            .is_ok(),
        Err(_) => false,
    }
}
