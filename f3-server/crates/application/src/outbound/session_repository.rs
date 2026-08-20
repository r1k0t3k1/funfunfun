use crate::{domain::model::session_model::Session, outbound::error::RepositoryError};

#[async_trait::async_trait]
pub trait SessionRepository: Send + Sync {
    async fn find_by_id(&self, session_id: String) -> Result<Option<Session>, RepositoryError>;
    async fn insert(&self, operator_id: String) -> Result<Session, RepositoryError>;
    async fn delete_by_id(&self, session_id: String) -> Result<(), RepositoryError>;
}
