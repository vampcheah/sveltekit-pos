//! 种子数据（幂等）：权限 / 角色 / 超管 / 样本门店 + 收银员。
//! 默认登录：admin/admin123（管理台）、cashier1/123456（pos-console）。
use pos_server::shared::password;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

// (code, name, group)
const PERMISSIONS: &[(&str, &str, &str)] = &[
    ("admins.write", "管理员管理", "admin"),
    ("roles.write", "角色权限", "admin"),
    ("cashiers.write", "收银员管理", "admin"),
    ("cashiers.reset_pin", "重置PIN", "admin"),
    ("stores.write", "门店管理", "admin"),
    ("settings.write", "系统设置", "admin"),
    ("members.write", "会员管理", "member"),
    ("members.points.write", "会员积分", "member"),
    ("members.balance.write", "会员储值", "member"),
    ("products.write", "商品管理", "catalog"),
    ("products.price.write", "商品改价", "catalog"),
    ("stock.adjust", "库存调整", "inventory"),
    ("stock.transfer", "库存调拨", "inventory"),
    ("orders.create", "下单结账", "orders"),
    ("orders.refund", "退款", "orders"),
    ("promotions.write", "促销管理", "promo"),
    ("reports.read", "查看报表", "reports"),
    ("reports.export", "导出报表", "reports"),
    ("audit.read", "审计日志", "audit"),
    ("shifts.manage", "班次管理", "shift"),
    ("shifts.approve", "班次审批", "shift"),
];

const ROLES: &[(&str, &str)] = &[
    ("super_admin", "超级管理员"),
    ("store_manager", "店长"),
    ("finance", "财务"),
    ("viewer", "只读"),
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let db = PgPoolOptions::new()
        .max_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;

    // 权限
    for (code, name, group) in PERMISSIONS {
        sqlx::query(
            "INSERT INTO permissions (code, name, group_) VALUES ($1,$2,$3) \
             ON CONFLICT (code) DO UPDATE SET name = EXCLUDED.name, group_ = EXCLUDED.group_",
        )
        .bind(code).bind(name).bind(group)
        .execute(&db).await?;
    }
    // 角色
    for (code, name) in ROLES {
        sqlx::query("INSERT INTO roles (code, name) VALUES ($1,$2) ON CONFLICT (code) DO NOTHING")
            .bind(code).bind(name).execute(&db).await?;
    }
    let super_id: i64 =
        sqlx::query_scalar("SELECT id FROM roles WHERE code = 'super_admin'").fetch_one(&db).await?;
    // super_admin 拥有全部权限
    sqlx::query(
        "INSERT INTO role_permissions (role_id, permission_id) \
         SELECT $1, id FROM permissions ON CONFLICT DO NOTHING",
    )
    .bind(super_id).execute(&db).await?;

    // 超管账号 admin/admin123
    let pw = password::hash("admin123").map_err(|e| e.to_string())?;
    sqlx::query(
        "INSERT INTO admins (username, email, password_hash, full_name, role_id, status) \
         VALUES ('admin','admin@pos.local',$1,'Super Admin',$2,'active') \
         ON CONFLICT (username) DO NOTHING",
    )
    .bind(&pw).bind(super_id).execute(&db).await?;

    // 样本门店
    sqlx::query(
        "INSERT INTO stores (name, code, timezone) VALUES ('总店','S001','Asia/Kuala_Lumpur') \
         ON CONFLICT (code) DO NOTHING",
    )
    .execute(&db).await?;
    let store_id: i64 =
        sqlx::query_scalar("SELECT id FROM stores WHERE code = 'S001'").fetch_one(&db).await?;

    // 样本收银员 cashier1/123456（主管），绑主店
    let pin = password::hash("123456").map_err(|e| e.to_string())?;
    sqlx::query(
        "INSERT INTO cashiers (username, pin_hash, full_name, is_supervisor, status) \
         VALUES ('cashier1',$1,'收银员一',true,'active') ON CONFLICT (username) DO NOTHING",
    )
    .bind(&pin).execute(&db).await?;
    let cashier_id: i64 =
        sqlx::query_scalar("SELECT id FROM cashiers WHERE username = 'cashier1'").fetch_one(&db).await?;
    sqlx::query(
        "INSERT INTO cashier_stores (cashier_id, store_id, is_home) VALUES ($1,$2,true) \
         ON CONFLICT DO NOTHING",
    )
    .bind(cashier_id).bind(store_id).execute(&db).await?;

    println!("seed done: admin/admin123, cashier1/123456 @ store S001");
    Ok(())
}
