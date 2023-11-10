use std::sync::Arc;

use chrono::{Duration, Utc};
use common::database::{Protobuf, Ulid};
use pb::ext::UlidExt;
use pb::scuffle::platform::internal::two_fa::{
    two_fa_request_action::{ChangePassword, Login},
    TwoFaRequestAction,
};

use crate::global::GlobalState;

use super::{Session, User};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TwoFaRequest {
    pub id: Ulid,
    pub user_id: Ulid,
    pub action: Protobuf<TwoFaRequestAction>,
}

#[async_trait::async_trait]
pub trait TwoFaRequestActionTrait {
    type Result;

    async fn execute(self, global: &Arc<GlobalState>, user_id: Ulid) -> Self::Result;
}

#[async_trait::async_trait]
impl TwoFaRequestActionTrait for Login {
    type Result = sqlx::Result<Session>;

    async fn execute(self, global: &Arc<GlobalState>, user_id: Ulid) -> Self::Result {
        let expires_at = Utc::now() + Duration::seconds(self.login_duration as i64);

        // TODO: maybe look to batch this
        let mut tx = global.db.begin().await?;

        let session: Session = sqlx::query_as(
            "INSERT INTO user_sessions (id, user_id, expires_at) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(Ulid::from(ulid::Ulid::new()))
        .bind(user_id)
        .bind(expires_at)
        .fetch_one(tx.as_mut())
        .await?;

        sqlx::query("UPDATE users SET last_login_at = NOW() WHERE id = $1")
            .bind(user_id)
            .execute(tx.as_mut())
            .await?;

        tx.commit().await?;

        Ok(session)
    }
}

#[async_trait::async_trait]
impl TwoFaRequestActionTrait for ChangePassword {
    type Result = sqlx::Result<()>;

    async fn execute(self, global: &Arc<GlobalState>, user_id: Ulid) -> sqlx::Result<()> {
        let mut tx = global.db.begin().await?;

        let user: User =
            sqlx::query_as("UPDATE users SET password_hash = $1 WHERE id = $2 RETURNING *")
                .bind(self.new_password_hash)
                .bind(user_id)
                .fetch_one(tx.as_mut())
                .await?;

        // Delete all sessions except current
        sqlx::query("DELETE FROM user_sessions WHERE user_id = $1 AND id != $2")
            .bind(user.id)
            .bind(Ulid::from(self.current_session_id.to_ulid()))
            .execute(tx.as_mut())
            .await?;

        // TODO: Logout active connections (See #145)

        tx.commit().await?;

        Ok(())
    }
}
