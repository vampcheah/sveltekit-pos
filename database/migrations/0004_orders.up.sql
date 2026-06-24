-- P4：订单/明细/支付 + 促销/优惠券 + 单据连号计数器。
-- 铁律：金额服务端重算；paid 后金额逻辑不可变；退款走反向单；幂等键防重复扣款。

CREATE TABLE order_counters (
    store_id BIGINT PRIMARY KEY REFERENCES stores(id),
    next_seq BIGINT NOT NULL DEFAULT 1
);

CREATE TABLE orders (
    id              BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    order_no        TEXT NOT NULL,
    seq_no          BIGINT NOT NULL,                 -- 按店严格连续（作废占号）
    idempotency_key UUID,                            -- 客户端下单时生成，补传防重
    kind            TEXT NOT NULL DEFAULT 'sale' CHECK (kind IN ('sale','refund')),
    parent_order_id BIGINT REFERENCES orders(id),
    store_id        BIGINT NOT NULL REFERENCES stores(id),
    warehouse_id    BIGINT REFERENCES warehouses(id),
    cashier_id      BIGINT REFERENCES cashiers(id),
    member_id       BIGINT REFERENCES members(id),
    subtotal        NUMERIC(12,2) NOT NULL,
    discount        NUMERIC(12,2) NOT NULL DEFAULT 0,
    tax             NUMERIC(12,2) NOT NULL DEFAULT 0,
    total           NUMERIC(12,2) NOT NULL,
    status          TEXT NOT NULL DEFAULT 'paid'
                    CHECK (status IN ('paid','partially_refunded','refunded','void')),
    reason_code     TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE UNIQUE INDEX uq_orders_idem ON orders(idempotency_key) WHERE idempotency_key IS NOT NULL;
CREATE UNIQUE INDEX uq_orders_no   ON orders(store_id, order_no);
CREATE INDEX ix_orders_store_time  ON orders(store_id, created_at);

CREATE TABLE order_items (
    id            BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    order_id      BIGINT NOT NULL REFERENCES orders(id),
    product_id    BIGINT REFERENCES products(id),    -- 弱引用，软删后仍保留历史
    sku_snapshot  TEXT NOT NULL,
    name_snapshot TEXT NOT NULL,
    unit_price    NUMERIC(12,2) NOT NULL,            -- 下单时售价快照
    unit_cost     NUMERIC(12,2) NOT NULL DEFAULT 0,  -- 锁住毛利
    quantity      NUMERIC(12,3) NOT NULL,            -- 称重支持小数
    tax_rate      NUMERIC(6,4) NOT NULL DEFAULT 0,
    tax_amount    NUMERIC(12,2) NOT NULL DEFAULT 0,
    line_discount NUMERIC(12,2) NOT NULL DEFAULT 0,
    line_total    NUMERIC(12,2) NOT NULL             -- 税前行额（=unit_price*qty）
);
CREATE INDEX ix_order_items_order   ON order_items(order_id);
CREATE INDEX ix_order_items_product ON order_items(product_id);

CREATE TABLE payments (
    id          BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    order_id    BIGINT NOT NULL REFERENCES orders(id),
    kind        TEXT NOT NULL DEFAULT 'payment' CHECK (kind IN ('payment','refund')),
    method      TEXT NOT NULL,                        -- cash|card|bsc_usdt|balance
    amount      NUMERIC(20,8) NOT NULL,               -- 原始支付币种金额
    currency    TEXT NOT NULL DEFAULT 'MYR',
    rate        NUMERIC(20,10) NOT NULL DEFAULT 1,    -- 1 currency = rate 记账币
    amount_base NUMERIC(12,2) NOT NULL,               -- 折算记账币；报表只 SUM 这列
    tendered    NUMERIC(12,2),                        -- 实收（现金算找零）
    ref         TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX ix_payments_order ON payments(order_id);

CREATE TABLE promotions (
    id         BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name       TEXT NOT NULL,
    type       TEXT NOT NULL CHECK (type IN ('percent','amount')),  -- bxgy 后续扩展
    value      NUMERIC(12,4) NOT NULL,
    min_amount NUMERIC(12,2) NOT NULL DEFAULT 0,
    starts_at  TIMESTAMPTZ,
    ends_at    TIMESTAMPTZ,
    status     TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active','inactive'))
);

CREATE TABLE coupons (
    id               BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    code             TEXT NOT NULL UNIQUE,
    promotion_id     BIGINT NOT NULL REFERENCES promotions(id),
    max_uses         INT,
    used_count       INT NOT NULL DEFAULT 0,
    per_member_limit INT,
    expires_at       TIMESTAMPTZ,
    status           TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active','inactive'))
);

CREATE TABLE coupon_redemptions (
    id          BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    coupon_id   BIGINT NOT NULL REFERENCES coupons(id),
    order_id    BIGINT NOT NULL REFERENCES orders(id),
    member_id   BIGINT,
    redeemed_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE UNIQUE INDEX uq_redemption_once ON coupon_redemptions(coupon_id, order_id);
