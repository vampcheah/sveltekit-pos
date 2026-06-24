//! auth 域：登录（admin/cashier）、登出、me。会话存 Redis（见 shared::session）。
mod model;
mod repo;
mod service;
pub mod routes;
