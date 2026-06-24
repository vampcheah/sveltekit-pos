-- P7：交接班/对账 + 操作审计。

CREATE TABLE shifts (
    id            BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    store_id      BIGINT NOT NULL REFERENCES stores(id),
    cashier_id    BIGINT NOT NULL REFERENCES cashiers(id),
    opened_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at     TIMESTAMPTZ,
    opening_cash  NUMERIC(12,2) NOT NULL DEFAULT 0,
    closing_cash  NUMERIC(12,2),
    expected_cash NUMERIC(12,2),    -- 开班备用金 + 现金销售 - 现金退款
    variance      NUMERIC(12,2),    -- 实点 - 理论
    status        TEXT NOT NULL DEFAULT 'open'
                  CHECK (status IN ('open','closed','pending_review','approved')),
    variance_reason TEXT,
    approved_by   BIGINT,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX ix_shifts_store ON shifts(store_id, opened_at);

-- 操作审计（仅追加；记发起人 + 现场授权人）。
CREATE TABLE audit_logs (
    id              BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    actor_type      TEXT NOT NULL,
    actor_id        BIGINT,
    authorizer_id   BIGINT,         -- 现场授权主管（override）
    action          TEXT NOT NULL,  -- 如 orders.refund / members.topup / stock.adjust
    entity          TEXT,
    entity_id       BIGINT,
    before_jsonb    JSONB,
    after_jsonb     JSONB,
    ip              TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX ix_audit_actor ON audit_logs(actor_type, actor_id, created_at);
CREATE INDEX ix_audit_action ON audit_logs(action, created_at);
