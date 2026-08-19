use crate::{domain::model::{operator_model::Operator, role_model::Role}, port::outbound::error::RepositoryError};

#[async_trait::async_trait]
pub trait OperatorRepository: Send + Sync {
    async fn find_by_id(&self, operator_id: String) -> Result<Option<Operator>, RepositoryError>;
    async fn list(&self) -> Result<Vec<Operator>, RepositoryError>;
    async fn insert(
        &self,
        id: String,
        password: String,
        name: String,
        description: String,
        role: Role,
    ) -> Result<Operator, RepositoryError>;

    async fn save(
        &self,
        operator: Operator,
    ) -> Result<Operator, RepositoryError>;
}
