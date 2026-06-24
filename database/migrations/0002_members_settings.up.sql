-- P2：会员 + 会员台账 + 系统设置。

CREATE TABLE members (
    id         BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    code       TEXT,
    phone      TEXT,
    name       TEXT NOT NULL,
    email      TEXT,
    tier       TEXT NOT NULL DEFAULT 'regular',
    points     BIGINT NOT NULL DEFAULT 0 CHECK (points >= 0),
    balance    NUMERIC(12,2) NOT NULL DEFAULT 0 CHECK (balance >= 0),  -- 储值是钱
    status     TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'disabled')),
    deleted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
-- 软删/可空下的唯一性
CREATE UNIQUE INDEX uq_members_phone ON members(phone) WHERE phone IS NOT NULL AND deleted_at IS NULL;
CREATE UNIQUE INDEX uq_members_code  ON members(code)  WHERE code  IS NOT NULL AND deleted_at IS NULL;

-- 会员台账：积分/储值的真值流水（members.points/balance 为派生快照）。
CREATE TABLE member_ledger (
    id            BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    member_id     BIGINT NOT NULL REFERENCES members(id),
    store_id      BIGINT REFERENCES stores(id),
    kind          TEXT NOT NULL CHECK (kind IN ('earn','redeem','topup','refund','adjust','expire')),
    points_delta  BIGINT NOT NULL DEFAULT 0,
    balance_delta NUMERIC(12,2) NOT NULL DEFAULT 0,
    points_after  BIGINT NOT NULL,
    balance_after NUMERIC(12,2) NOT NULL,
    order_id      BIGINT,        -- 关联订单（orders 域在 P4 建表，此处先留弱引用）
    note          TEXT,
    created_by    BIGINT,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX ix_member_ledger_member ON member_ledger(member_id, created_at);

-- 系统设置（税率/货币/小票模板/营业参数集中落点）。一期全局 KV 足够，店级 scope 按需再加。
CREATE TABLE settings (
    key        TEXT PRIMARY KEY,
    value      JSONB NOT NULL,
    updated_by BIGINT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
