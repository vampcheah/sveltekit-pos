-- P3：商品/分类 + 仓库/库存/库存流水。

CREATE TABLE categories (
    id        BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    code      TEXT NOT NULL UNIQUE,
    name      TEXT NOT NULL,
    sort      INT NOT NULL DEFAULT 0,
    parent_id BIGINT REFERENCES categories(id)
);

CREATE TABLE products (
    id           BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    sku          TEXT NOT NULL,
    barcode      TEXT,
    name         TEXT NOT NULL,
    category_id  BIGINT REFERENCES categories(id),
    price        NUMERIC(12,2) NOT NULL CHECK (price >= 0),
    cost         NUMERIC(12,2) NOT NULL DEFAULT 0 CHECK (cost >= 0),
    unit         TEXT,
    is_weighted  BOOLEAN NOT NULL DEFAULT false,
    tax_rate     NUMERIC(6,4) NOT NULL DEFAULT 0,   -- 0.06 = 6%
    tax_category TEXT,
    image_url    TEXT,
    status       TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active','inactive')),
    deleted_at   TIMESTAMPTZ,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE UNIQUE INDEX uq_products_sku     ON products(sku)     WHERE deleted_at IS NULL;
CREATE UNIQUE INDEX uq_products_barcode ON products(barcode) WHERE barcode IS NOT NULL AND deleted_at IS NULL;
CREATE INDEX ix_products_category ON products(category_id);

CREATE TABLE warehouses (
    id       BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    store_id BIGINT NOT NULL REFERENCES stores(id),
    code     TEXT NOT NULL,
    name     TEXT NOT NULL,
    type     TEXT NOT NULL DEFAULT 'store' CHECK (type IN ('store','central'))
);

CREATE TABLE stock_levels (
    id            BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    warehouse_id  BIGINT NOT NULL REFERENCES warehouses(id),
    product_id    BIGINT NOT NULL REFERENCES products(id),
    quantity      NUMERIC(12,3) NOT NULL DEFAULT 0 CHECK (quantity >= 0),  -- 防超卖
    reorder_point NUMERIC(12,3) NOT NULL DEFAULT 0,
    UNIQUE (warehouse_id, product_id)
);
CREATE INDEX ix_low_stock ON stock_levels(warehouse_id) WHERE quantity < reorder_point;

CREATE TABLE stock_movements (
    id           BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    warehouse_id BIGINT NOT NULL REFERENCES warehouses(id),
    product_id   BIGINT NOT NULL REFERENCES products(id),
    type         TEXT NOT NULL CHECK (type IN ('in','out','adjust','transfer','sale','refund','count')),
    quantity     NUMERIC(12,3) NOT NULL,    -- 带符号：出库为负
    ref          TEXT,
    note         TEXT,
    created_by   BIGINT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX ix_movements ON stock_movements(warehouse_id, product_id, created_at);
