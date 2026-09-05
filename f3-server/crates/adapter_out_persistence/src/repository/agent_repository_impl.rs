use application::domain::model::agent_model::AgentModel;
use application::domain::model::id::{AgentId, ListenerId};
use application::outbound::agent_repository::AgentRepository;
use sqlx::PgPool;

use application::outbound::error::RepositoryError;
use sqlx::types::Uuid;

use crate::entity::agent_entity::AgentEntity;

#[derive(Debug, Clone)]
pub struct AgentRepositoryImpl {
    connection: PgPool,
}

impl AgentRepositoryImpl {
    pub fn new(connection: PgPool) -> Self {
        Self { connection }
    }
}

#[async_trait::async_trait]
impl AgentRepository for AgentRepositoryImpl {
    async fn find_by_id(&self, agent_id: AgentId) -> Result<Option<AgentModel>, RepositoryError> {
        sqlx::query_as!(
            AgentEntity,
            r#"SELECT 
                 *
               FROM agents
               WHERE id = $1
            "#,
            Into::<Uuid>::into(agent_id),
        )
        .fetch_optional(&self.connection)
        .await
        .map(|oae| oae.map(|ae| ae.into()))
        .map_err(|e| RepositoryError::Infrastructure(e.into()))
    }

    async fn list(&self) -> Result<Vec<AgentModel>, RepositoryError> {
        Ok(
            sqlx::query_as!(
                AgentEntity,
                r#"SELECT * FROM agents;"#,
            )
            .fetch_all(&self.connection)
            .await
            .map_err(|e| RepositoryError::Infrastructure(e.into()))?
            .into_iter()
            .map(|a| a.into())
            .collect()
        )
    }

    async fn list_by_listener_id(&self, listener_id: ListenerId) -> Result<Vec<AgentModel>, RepositoryError> {
        Ok(
            sqlx::query_as!(
                AgentEntity,
                r#"SELECT * FROM agents where listener_id = $1;"#,
                Into::<Uuid>::into(listener_id),
            )
            .fetch_all(&self.connection)
            .await
            .map_err(|e| RepositoryError::Infrastructure(e.into()))?
            .into_iter()
            .map(|a| a.into())
            .collect()
        )
    }

    async fn insert(
        &self,
        listener_id: ListenerId,
        shared_secret: [u8; 32],
        process_id: u64,
        thread_id: u64,
        arch: String,
        is_admin: bool,
        process_name: String,
        os: String,
        domain_name: String,
        computer_name: String,
        user_name: String,
    ) -> Result<AgentModel, RepositoryError> {
        sqlx::query_as!(
            AgentEntity,
            r#"INSERT INTO agents (
                 listener_id,
                 shared_secret,
                 process_id, 
                 thread_id, 
                 arch, 
                 is_admin,
                 process_name, 
                 os,
                 domain_name,
                 computer_name,
                 user_name
               )
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
               RETURNING *;
            "#,
            Into::<Uuid>::into(listener_id), 
            &shared_secret,
            process_id as i64, 
            thread_id as i64, 
            arch, 
            is_admin,
            process_name, 
            os,
            domain_name,
            computer_name,
            user_name
        )
        .fetch_one(&self.connection)
        .await
        .map(|ae| ae.into())
        .map_err(|e| RepositoryError::Infrastructure(e.into()))
    }

    async fn save(&self, agent: AgentModel) -> Result<AgentModel, RepositoryError> {
        Ok(
            sqlx::query_as!(
                AgentEntity,
                r#"UPDATE agents
                   SET 
                     process_id    = $2, 
                     thread_id     = $3, 
                     arch          = $4, 
                     is_admin      = $5,
                     process_name  = $6, 
                     os            = $7,
                     domain_name   = $8,
                     computer_name = $9,
                     user_name     = $10
                   WHERE id = $1
                   RETURNING *
                "#,
                Into::<Uuid>::into(agent.id),
                agent.process_id as i64,
                agent.thread_id as i64,
                agent.arch,
                agent.is_admin,
                agent.process_name,
                agent.os,
                agent.domain_name,
                agent.computer_name,
                agent.user_name,
            )
            .fetch_one(&self.connection)
            .await
            .map(|a| a.into())
            .map_err(|e| RepositoryError::Infrastructure(e.into()))?
        )
    }

    async fn delete_by_id(&self, agent_id: AgentId) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"DELETE FROM agents WHERE id = $1;"#,
            Into::<Uuid>::into(agent_id),
        )
        .execute(&self.connection)
        .await
        .map(|_| ())
        .map_err(|e| RepositoryError::Infrastructure(e.into()))
    }
}
