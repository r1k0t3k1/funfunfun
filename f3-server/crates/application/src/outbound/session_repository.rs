use crate::{domain::model::{id::{OperatorId, SessionId}, session_model::SessionModel}, outbound::error::RepositoryError};

#[async_trait::async_trait]
pub trait SessionRepository: Send + Sync {
    async fn find_by_id(&self, session_id: SessionId) -> Result<Option<SessionModel>, RepositoryError>;
    async fn insert(&self, operator_id: OperatorId) -> Result<SessionModel, RepositoryError>;
    async fn delete_by_id(&self, session_id: SessionId) -> Result<(), RepositoryError>;
}
