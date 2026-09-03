use crate::{
    domain::model::{id::OperatorId, operator_model::OperatorModel, role_model::Role},
    outbound::error::RepositoryError,
};

#[async_trait::async_trait]
pub trait OperatorRepository: Send + Sync {
    async fn find_by_id(&self, operator_id: OperatorId) -> Result<Option<OperatorModel>, RepositoryError>;
    async fn list(&self) -> Result<Vec<OperatorModel>, RepositoryError>;
    async fn insert(
        &self,
        name: String,
        password: String,
        description: String,
        role: Role,
        is_enabled: bool,
    ) -> Result<OperatorModel, RepositoryError>;

    async fn save(&self, operator: OperatorModel) -> Result<OperatorModel, RepositoryError>;
}
