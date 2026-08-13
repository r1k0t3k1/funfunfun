use crate::{error::DomainError, model::session_model::Session};

#[async_trait::async_trait]
pub trait SessionRepository: Send + Sync {
    async fn find_by_id(&self, session_id: String) -> Result<Option<Session>, DomainError>;
    async fn insert(&self, operator_id: String) -> Result<Session, DomainError>;
    async fn delete_by_id(&self, session_id: String) -> Result<(), DomainError>;
}
