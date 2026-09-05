use application::{domain::model::{id::ListenerId, listener_model::{ListenerConfig, ListenerModel}}, outbound::{error::RepositoryError, listener_repository::ListenerRepository}};
use sqlx::{PgPool, types::Uuid};
use crate::entity::listener_entity::{ListenerConfigEntity, ListenerEntity};

#[derive(Debug, Clone)]
pub struct ListenerRepositoryImpl {
    connection: PgPool,
}

impl ListenerRepositoryImpl {
    pub fn new(connection: PgPool) -> Self {
        Self { connection }
    }
}

#[async_trait::async_trait]
impl ListenerRepository for ListenerRepositoryImpl {
    async fn find_by_id(&self, listener_id: ListenerId) -> Result<Option<ListenerModel>, RepositoryError> {
        let row = sqlx::query_as!(
            ListenerEntity, 
            "SELECT * FROM listeners WHERE id = $1",
            Into::<Uuid>::into(listener_id),
        )
        .fetch_optional(&self.connection)
        .await
        .map_err(|e| RepositoryError::Infrastructure(e.into()))?
        .ok_or(RepositoryError::NotFound)?;

        Ok(Some(TryInto::<ListenerModel>::try_into(row)?))
    }

    async fn list(&self) -> Result<Vec<ListenerModel>, RepositoryError> {
        let entities = sqlx::query_as!(
            ListenerEntity, 
            "SELECT * FROM listeners",
        )
        .fetch_all(&self.connection)
        .await
        .map_err(|e| RepositoryError::Infrastructure(e.into()))?;
        
        let mut results: Vec<ListenerModel> = vec![];

        for r in entities.into_iter() {
            results.push(TryInto::<ListenerModel>::try_into(r)?);
        }

        Ok(results)
    }

    async fn insert(
        &self,
        name: String,
        lhost: String,
        lport: u16,
        config: ListenerConfig,
    ) -> Result<ListenerModel, RepositoryError> {
        let row = sqlx::query_as!(
            ListenerEntity,
            r#"INSERT INTO listeners (name, lhost, lport, config)
               VALUES (
                    $1,
                    $2,
                    $3,
                    $4
               ) RETURNING 
                id,
                name,
                lhost,
                lport, 
                is_running,
                checkin_key,
                config,
                created_at
            "#,
            name,
            lhost,
            lport as i32,
            serde_json::to_string(&Into::<ListenerConfigEntity>::into(config)).unwrap(),
        )
        .fetch_one(&self.connection)
        .await
        .map_err(|e| RepositoryError::Infrastructure(e.into()))?;

        Ok(row.try_into()?)
    }
//  id | name | lhost | lport | is_running | checkin_key | config
    async fn save(&self, listener: ListenerModel) -> Result<ListenerModel, RepositoryError> {
        let row = sqlx::query_as!(
            ListenerEntity,
            r#"UPDATE listeners
               SET name = $2,
                   lhost = $3,
                   lport = $4,
                   is_running = $5,
                   config = $6
               WHERE id = $1
               RETURNING 
                id,
                name,
                lhost,
                lport, 
                is_running,
                checkin_key,
                config,
                created_at;
            "#,
            Into::<Uuid>::into(listener.id),
            listener.name,
            listener.lhost,
            listener.lport as i32,
            listener.is_running,
            serde_json::to_string(&Into::<ListenerConfigEntity>::into(listener.config))
                .map_err(|e| RepositoryError::FailedToDesirialize { detail: e.to_string()})?,
        )
        .fetch_one(&self.connection)
        .await
        .map_err(|e| RepositoryError::Infrastructure(e.into()))?;

        Ok(row.try_into()?)
    }

    async fn delete_by_id(&self, listener_id: ListenerId) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"DELETE FROM listeners WHERE id = $1;"#,
            Into::<Uuid>::into(listener_id),
        )
        .execute(&self.connection)
        .await
        .map(|_| ())
        .map_err(|e| RepositoryError::Infrastructure(e.into()))
    }
}
