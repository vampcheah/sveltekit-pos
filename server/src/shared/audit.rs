//! 操作审计（best-effort，事务外）：敏感操作成功后记一条，失败只记日志不影响业务。
use serde_json::Value;

use crate::{shared::auth::CurrentActor, state::AppState};

pub async fn record(
    st: &AppState,
    actor: &CurrentActor,
    action: &str,
    entity: &str,
    entity_id: i64,
    after: Option<Value>,
) {
    let res = sqlx::query(
        "INSERT INTO audit_logs (actor_type, actor_id, action, entity, entity_id, after_jsonb) \
         VALUES ($1,$2,$3,$4,$5,$6)",
    )
    .bind(&actor.session.actor_type)
    .bind(actor.session.actor_id)
    .bind(action)
    .bind(entity)
    .bind(entity_id)
    .bind(after)
    .execute(&st.db)
    .await;
    if let Err(e) = res {
        tracing::error!(error = %e, action, "audit write failed"); // 不阻断业务
    }
}
