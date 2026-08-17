use crate::{domain::model::operator_model::Operator, port::outbound::error::RepositoryError};

#[async_trait::async_trait]
pub trait OperatorRepository: Send + Sync {
    async fn find_by_id(&self, operator_id: String) -> Result<Option<Operator>, RepositoryError>;

    async fn find_by_credential(
        &self,
        operator_id: String,
        password: String,
    ) -> Result<Option<Operator>, RepositoryError>;
}
