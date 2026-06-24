//! auth 域 DTO。
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginAdminReq {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginCashierReq {
    pub username: String,
    pub pin: String,
}

#[derive(Serialize)]
pub struct MeResp {
    pub actor_type: String,
    pub actor_id: i64,
    pub username: Option<String>,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub role_id: Option<i64>,
    pub store_id: Option<i64>,
    pub is_supervisor: bool,
    pub permissions: Vec<String>,
}

#[derive(Deserialize)]
pub struct ChangePasswordReq {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Deserialize)]
pub struct UpdateProfileReq {
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
}
