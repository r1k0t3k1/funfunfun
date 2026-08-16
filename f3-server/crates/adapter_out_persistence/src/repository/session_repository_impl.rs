use sqlx::PgPool;
use sqlx::postgres::types::PgHstore;

use application::domain::model::session_model::Session;
use application::port::outbound::session_repository::SessionRepository;

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
    async fn find_by_id(&self, session_id: String) -> Result<Option<Session>, RepositoryError> {
        sqlx::query_as!(
            SessionEntity,
            r#"SELECT 
                  session_id,
                  operator_id,
                  expire_at,
                  attribute AS "attribute: PgHstore"
              FROM sessions
              WHERE session_id = $1;
            "#,
            session_id.into()
        )
        .fetch_optional(&self.connection)
        .await
        .map(|ose| ose.map(|se| se.into()))
        .map_err(|e| RepositoryError::Infrastructure(e.into()))
    }

    async fn insert(&self, operator_id: String) -> Result<Session, RepositoryError> {
        sqlx::query_as!(
            SessionEntity,
            r#"INSERT INTO sessions (operator_id) 
               VALUES ($1) RETURNING *; 
            "#,
            operator_id.into()
        )
        .fetch_one(&self.connection)
        .await
        .map(|se| se.into())
        .map_err(|e| RepositoryError::Infrastructure(e.into()))
    }

    async fn delete_by_id(&self, session_id: String) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"DELETE FROM sessions WHERE session_id = $1;"#,
            session_id.into()
        )
        .execute(&self.connection)
        .await
        .map(|_| ())
        .map_err(|e| RepositoryError::Infrastructure(e.into()))
    }
}
