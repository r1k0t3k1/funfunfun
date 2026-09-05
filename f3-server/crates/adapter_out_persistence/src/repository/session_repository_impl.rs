use application::domain::model::id::{OperatorId, SessionId};
use sqlx::PgPool;
use sqlx::postgres::types::PgHstore;

use application::domain::model::session_model::SessionModel;
use application::outbound::error::RepositoryError;
use application::outbound::session_repository::SessionRepository;
use sqlx::types::Uuid;

use crate::entity::session_entity::SessionEntity;

#[derive(Debug, Clone)]
pub struct SessionRepositoryImpl {
    connection: PgPool,
}

impl SessionRepositoryImpl {
    pub fn new(connection: PgPool) -> Self {
        Self { connection }
    }
}

#[async_trait::async_trait]
impl SessionRepository for SessionRepositoryImpl {
    async fn find_by_id(&self, session_id: SessionId) -> Result<Option<SessionModel>, RepositoryError> {
        sqlx::query_as!(
            SessionEntity,
            r#"SELECT 
                  id,
                  operator_id,
                  expire_at,
                  attribute AS "attribute: PgHstore"
              FROM sessions
              WHERE id = $1;
            "#,
            Into::<Uuid>::into(session_id),
        )
        .fetch_optional(&self.connection)
        .await
        .map(|ose| ose.map(|se| se.into()))
        .map_err(|e| RepositoryError::Infrastructure(e.into()))
    }

    async fn insert(&self, operator_id: OperatorId) -> Result<SessionModel, RepositoryError> {
        sqlx::query_as!(
            SessionEntity,
            r#"INSERT INTO sessions (operator_id) 
               VALUES ($1) RETURNING *; 
            "#,
            Into::<Uuid>::into(operator_id),
        )
        .fetch_one(&self.connection)
        .await
        .map(|se| se.into())
        .map_err(|e| RepositoryError::Infrastructure(e.into()))
    }

    async fn delete_by_id(&self, session_id: SessionId) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"DELETE FROM sessions WHERE id = $1;"#,
            Into::<Uuid>::into(session_id),
        )
        .execute(&self.connection)
        .await
        .map(|_| ())
        .map_err(|e| RepositoryError::Infrastructure(e.into()))
    }
}
