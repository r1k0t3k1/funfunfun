use crate::{error::DomainError, model::operator_model::Operator};

#[async_trait::async_trait]
pub trait OperatorRepository: Send + Sync {
    async fn find_by_id(&self, operator_id: String) -> Result<Option<Operator>, DomainError>;
    async fn find_by_credential(
        &self,
        operator_id: String,
        password: String,
    ) -> Result<Option<Operator>, DomainError>;
}
