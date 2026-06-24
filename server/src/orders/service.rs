//! orders 域业务：结账（单事务·服务端重算·原子扣库存·幂等）、退款（反向单）。
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, Transaction};

use crate::{
    error::{AppError, AppResult},
    inventory,
    orders::{model::*, repo},
};

fn r2(d: Decimal) -> Decimal {
    d.round_dp(2)
}

/// 结账。cashier_id/store_id 由 route 按会话解析后传入。
pub async fn checkout(
    db: &PgPool,
    cashier_id: Option<i64>,
    store_id: i64,
    req: &CheckoutReq,
) -> AppResult<OrderView> {
    // 幂等：命中已存在订单即原样返回（补传安全）
    if let Some(key) = req.idempotency_key {
        if let Some(id) = repo::find_by_idem(db, &key).await? {
            return repo::get_view(db, id).await?.ok_or(AppError::NotFound);
        }
    }
    if req.items.is_empty() {
        return Err(AppError::BadRequest("订单为空".into()));
    }

    let mut tx = db.begin().await?;

    // 1) 服务端按商品主数据重算每行（绝不信前端金额）
    struct Line {
        product_id: i64,
        sku: String,
        name: String,
        unit_price: Decimal,
        unit_cost: Decimal,
        qty: Decimal,
        tax_rate: Decimal,
        tax_amount: Decimal,
        line_total: Decimal,
    }
    let mut lines: Vec<Line> = Vec::new();
    let mut subtotal = Decimal::ZERO;
    let mut tax = Decimal::ZERO;
    for it in &req.items {
        if it.quantity <= Decimal::ZERO {
            return Err(AppError::BadRequest("数量须为正".into()));
        }
        let row: Option<(String, String, Decimal, Decimal, Decimal)> = sqlx::query_as(
            "SELECT sku, name, price, cost, tax_rate FROM products WHERE id = $1 AND deleted_at IS NULL",
        )
        .bind(it.product_id)
        .fetch_optional(&mut *tx)
        .await?;
        let Some((sku, name, price, cost, tax_rate)) = row else {
            return Err(AppError::BadRequest(format!("商品 {} 不存在", it.product_id)));
        };
        let line_total = r2(price * it.quantity);
        let tax_amount = r2(price * it.quantity * tax_rate);
        subtotal += line_total;
        tax += tax_amount;
        lines.push(Line {
            product_id: it.product_id, sku, name, unit_price: price, unit_cost: cost,
            qty: it.quantity, tax_rate, tax_amount, line_total,
        });
    }

    // 2) 优惠券（原子核销）
    let mut discount = Decimal::ZERO;
    let mut coupon_id: Option<i64> = None;
    if let Some(code) = req.coupon_code.as_ref().filter(|c| !c.is_empty()) {
        let row: Option<(i64, String, Decimal, Decimal)> = sqlx::query_as(
            "SELECT c.id, p.type, p.value, p.min_amount FROM coupons c \
             JOIN promotions p ON p.id = c.promotion_id \
             WHERE c.code = $1 AND c.status = 'active' AND p.status = 'active' \
               AND (c.expires_at IS NULL OR c.expires_at > now())",
        )
        .bind(code)
        .fetch_optional(&mut *tx)
        .await?;
        let Some((cid, ptype, pvalue, min_amount)) = row else {
            return Err(AppError::BadRequest("优惠券无效或已过期".into()));
        };
        if subtotal < min_amount {
            return Err(AppError::BadRequest("未达优惠券门槛".into()));
        }
        // 原子自增用量，0 行=已用尽
        let used = sqlx::query(
            "UPDATE coupons SET used_count = used_count + 1 \
             WHERE id = $1 AND (max_uses IS NULL OR used_count < max_uses)",
        )
        .bind(cid)
        .execute(&mut *tx)
        .await?;
        if used.rows_affected() != 1 {
            return Err(AppError::BadRequest("优惠券已用尽".into()));
        }
        discount = match ptype.as_str() {
            "percent" => r2(subtotal * pvalue / Decimal::from(100)),
            _ => pvalue, // amount
        };
        if discount > subtotal {
            discount = subtotal;
        }
        coupon_id = Some(cid);
    }

    let mut total = subtotal + tax - discount;
    if total < Decimal::ZERO {
        total = Decimal::ZERO;
    }

    // 3) 单据连号 + 扣库存（原子，防超卖）
    let seq = repo::next_seq(&mut tx, store_id).await?;
    let order_no = format!("S{store_id}-{seq:06}");
    for l in &lines {
        inventory::repo::apply_delta(
            &mut tx, req.warehouse_id, l.product_id, -l.qty, "sale",
            Some(&order_no), None, cashier_id,
        )
        .await?;
    }

    // 4) 落订单头
    let order_id: i64 = sqlx::query_scalar(
        "INSERT INTO orders (order_no, seq_no, idempotency_key, kind, store_id, warehouse_id, \
           cashier_id, member_id, subtotal, discount, tax, total, status) \
         VALUES ($1,$2,$3,'sale',$4,$5,$6,$7,$8,$9,$10,$11,'paid') RETURNING id",
    )
    .bind(&order_no).bind(seq).bind(req.idempotency_key).bind(store_id).bind(req.warehouse_id)
    .bind(cashier_id).bind(req.member_id).bind(subtotal).bind(discount).bind(tax).bind(total)
    .fetch_one(&mut *tx)
    .await?;

    // 5) 明细快照
    for l in &lines {
        sqlx::query(
            "INSERT INTO order_items (order_id, product_id, sku_snapshot, name_snapshot, unit_price, \
               unit_cost, quantity, tax_rate, tax_amount, line_total) \
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)",
        )
        .bind(order_id).bind(l.product_id).bind(&l.sku).bind(&l.name).bind(l.unit_price)
        .bind(l.unit_cost).bind(l.qty).bind(l.tax_rate).bind(l.tax_amount).bind(l.line_total)
        .execute(&mut *tx)
        .await?;
    }

    // 6) 支付（默认整单现金；校验付足）
    let mut paid_base = Decimal::ZERO;
    if req.payments.is_empty() {
        sqlx::query(
            "INSERT INTO payments (order_id, kind, method, amount, currency, rate, amount_base, tendered) \
             VALUES ($1,'payment','cash',$2,'MYR',1,$2,$2)",
        )
        .bind(order_id).bind(total)
        .execute(&mut *tx)
        .await?;
        paid_base = total;
    } else {
        for p in &req.payments {
            let rate = p.rate.unwrap_or(Decimal::ONE);
            let amount_base = r2(p.amount * rate);
            paid_base += amount_base;
            sqlx::query(
                "INSERT INTO payments (order_id, kind, method, amount, currency, rate, amount_base, tendered, ref) \
                 VALUES ($1,'payment',$2,$3,$4,$5,$6,$7,$8)",
            )
            .bind(order_id).bind(&p.method).bind(p.amount).bind(p.currency.as_deref().unwrap_or("MYR"))
            .bind(rate).bind(amount_base).bind(p.tendered).bind(&p.r#ref)
            .execute(&mut *tx)
            .await?;
        }
    }
    if paid_base < total {
        return Err(AppError::BadRequest("支付金额不足".into()));
    }

    // 7) 券核销记录
    if let Some(cid) = coupon_id {
        sqlx::query(
            "INSERT INTO coupon_redemptions (coupon_id, order_id, member_id) VALUES ($1,$2,$3)",
        )
        .bind(cid).bind(order_id).bind(req.member_id)
        .execute(&mut *tx)
        .await?;
    }

    // 8) 会员积分（1 分/记账币元）
    if let Some(mid) = req.member_id {
        let earned: i64 = total.trunc().try_into().unwrap_or(0);
        if earned > 0 {
            earn_points(&mut tx, mid, earned, store_id, order_id).await?;
        }
    }

    tx.commit().await?;
    repo::get_view(db, order_id).await?.ok_or(AppError::NotFound)
}

async fn earn_points(
    tx: &mut Transaction<'_, Postgres>,
    member_id: i64,
    points: i64,
    store_id: i64,
    order_id: i64,
) -> AppResult<()> {
    let cur: Option<(i64, Decimal)> =
        sqlx::query_as("SELECT points, balance FROM members WHERE id = $1 FOR UPDATE")
            .bind(member_id)
            .fetch_optional(&mut **tx)
            .await?;
    let Some((p, b)) = cur else { return Ok(()) };
    let np = p + points;
    sqlx::query("UPDATE members SET points = $2 WHERE id = $1").bind(member_id).bind(np).execute(&mut **tx).await?;
    sqlx::query(
        "INSERT INTO member_ledger (member_id, store_id, kind, points_delta, balance_delta, points_after, balance_after, order_id) \
         VALUES ($1,$2,'earn',$3,0,$4,$5,$6)",
    )
    .bind(member_id).bind(store_id).bind(points).bind(np).bind(b).bind(order_id)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

/// 退款=反向单（整单）。store_scope=Some 时强制本店（防 IDOR）。
pub async fn refund(
    db: &PgPool,
    store_scope: Option<i64>,
    order_id: i64,
    req: &RefundReq,
) -> AppResult<OrderView> {
    let order = repo::get_row(db, order_id).await?.ok_or(AppError::NotFound)?;
    if let Some(s) = store_scope {
        if order.store_id != s {
            return Err(AppError::NotFound); // 不泄露存在性
        }
    }
    if order.kind != "sale" || matches!(order.status.as_str(), "refunded" | "void") {
        return Err(AppError::BadRequest("订单不可退款".into()));
    }
    let items = repo::order_items_for_refund(db, order_id).await?;
    let warehouse = repo::order_warehouse(db, order_id).await?;

    let mut tx = db.begin().await?;
    let seq = repo::next_seq(&mut tx, order.store_id).await?;
    let refund_no = format!("S{}-R{seq:06}", order.store_id);

    // 反向单头（负额）
    let refund_id: i64 = sqlx::query_scalar(
        "INSERT INTO orders (order_no, seq_no, kind, parent_order_id, store_id, warehouse_id, \
           cashier_id, member_id, subtotal, discount, tax, total, status, reason_code) \
         VALUES ($1,$2,'refund',$3,$4,$5,$6,$7,$8,$9,$10,$11,'paid',$12) RETURNING id",
    )
    .bind(&refund_no).bind(seq).bind(order_id).bind(order.store_id).bind(warehouse)
    .bind(order.cashier_id).bind(order.member_id)
    .bind(-order.subtotal).bind(-order.discount).bind(-order.tax).bind(-order.total)
    .bind(&req.reason_code)
    .fetch_one(&mut *tx)
    .await?;

    for (product_id, sku, name, unit_price, unit_cost, qty, tax_amount, line_total) in &items {
        // 回库
        if let (Some(wh), Some(pid)) = (warehouse, product_id) {
            inventory::repo::apply_delta(&mut tx, wh, *pid, *qty, "refund", Some(&refund_no), req.note.as_deref(), order.cashier_id).await?;
        }
        // 负向明细
        sqlx::query(
            "INSERT INTO order_items (order_id, product_id, sku_snapshot, name_snapshot, unit_price, \
               unit_cost, quantity, tax_amount, line_total) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)",
        )
        .bind(refund_id).bind(product_id).bind(sku).bind(name).bind(unit_price)
        .bind(unit_cost).bind(-*qty).bind(-*tax_amount).bind(-*line_total)
        .execute(&mut *tx)
        .await?;
    }

    // 退款支付（退回原首个支付方式）
    let method: String = sqlx::query_scalar(
        "SELECT method FROM payments WHERE order_id = $1 AND kind = 'payment' ORDER BY id LIMIT 1",
    )
    .bind(order_id)
    .fetch_optional(&mut *tx)
    .await?
    .unwrap_or_else(|| "cash".to_string());
    sqlx::query(
        "INSERT INTO payments (order_id, kind, method, amount, currency, rate, amount_base) \
         VALUES ($1,'refund',$2,$3,'MYR',1,$3)",
    )
    .bind(refund_id).bind(&method).bind(-order.total)
    .execute(&mut *tx)
    .await?;

    // 原单标记已退
    sqlx::query("UPDATE orders SET status = 'refunded' WHERE id = $1").bind(order_id).execute(&mut *tx).await?;

    tx.commit().await?;
    repo::get_view(db, refund_id).await?.ok_or(AppError::NotFound)
}
