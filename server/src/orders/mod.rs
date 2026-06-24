//! orders 域：结账（服务端重算·原子扣库存·幂等）+ 退款（反向单）。
mod model;
mod repo;
mod service;
pub mod routes;
