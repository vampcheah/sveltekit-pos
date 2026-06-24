-- P1 核心鉴权域：门店 / RBAC / 管理员 / 收银员。
-- 约定：金额留待后续域；时间 timestamptz；status/type 用 TEXT+CHECK；标识符 UNIQUE。

CREATE TABLE stores (
    id         BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name       TEXT NOT NULL,
    code       TEXT NOT NULL UNIQUE,
    address    TEXT,
    phone      TEXT,
    timezone   TEXT NOT NULL DEFAULT 'Asia/Kuala_Lumpur',
    status     TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'inactive')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE roles (
    id          BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    code        TEXT NOT NULL UNIQUE,
    name        TEXT NOT NULL,
    description TEXT
);

CREATE TABLE permissions (
    id      BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    code    TEXT NOT NULL UNIQUE,          -- 如 products.price.write / orders.refund
    name    TEXT NOT NULL,
    group_  TEXT NOT NULL DEFAULT 'general'
);

CREATE TABLE role_permissions (
    role_id       BIGINT NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_id BIGINT NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);

-- 管理员（platform-admin）。含账号生命周期字段（角色评审）。
CREATE TABLE admins (
    id                   BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    username             TEXT NOT NULL UNIQUE,
    email                TEXT,
    password_hash        TEXT NOT NULL,
    full_name            TEXT,
    role_id              BIGINT REFERENCES roles(id),       -- 空=无权限（deny-by-default）
    status               TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'disabled')),
    must_change_password BOOLEAN NOT NULL DEFAULT false,
    failed_login_count   INTEGER NOT NULL DEFAULT 0,
    locked_until         TIMESTAMPTZ,
    disabled_at          TIMESTAMPTZ,
    last_login_at        TIMESTAMPTZ,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 收银员（登陆 pos-console）。⚠ 无明文 PIN：pin_hash 走 Argon2id。is_supervisor=主管层。
CREATE TABLE cashiers (
    id                   BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    username             TEXT NOT NULL UNIQUE,
    pin_hash             TEXT NOT NULL,
    password_hash        TEXT,
    full_name            TEXT,
    is_supervisor        BOOLEAN NOT NULL DEFAULT false,
    status               TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'disabled')),
    must_change_password BOOLEAN NOT NULL DEFAULT false,
    failed_login_count   INTEGER NOT NULL DEFAULT 0,
    locked_until         TIMESTAMPTZ,
    disabled_at          TIMESTAMPTZ,
    last_login_at        TIMESTAMPTZ,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 收银员↔门店（含主店）。开单须命中此表（防越权开单）。
CREATE TABLE cashier_stores (
    cashier_id BIGINT NOT NULL REFERENCES cashiers(id) ON DELETE CASCADE,
    store_id   BIGINT NOT NULL REFERENCES stores(id) ON DELETE CASCADE,
    is_home    BOOLEAN NOT NULL DEFAULT false,
    PRIMARY KEY (cashier_id, store_id)
);
