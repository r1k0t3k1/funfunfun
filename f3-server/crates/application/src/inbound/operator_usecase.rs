use crate::{
    domain::model::{
        id::OperatorId, operator_model::OperatorModel, role_model::Role
    },
    inbound::error::OperatorUsecaseError,
};

#[async_trait::async_trait]
pub trait OperatorUsecase: Send + Sync {
    async fn list_operators(&self) -> Result<Vec<OperatorModel>, OperatorUsecaseError>;
    async fn get_operator(
        &self,
        operator_id: OperatorId,
    ) -> Result<Option<OperatorModel>, OperatorUsecaseError>;
    async fn register_operator(
        &self,
        password: String,
        name: String,
        description: String,
        role: Role,
    ) -> Result<OperatorModel, OperatorUsecaseError>;
    async fn toggle_status(&self, operator_id: OperatorId) -> Result<OperatorModel, OperatorUsecaseError>;
    async fn change_password(
        &self,
        operator_id: OperatorId,
        current_password: String,
        new_password: String,
    ) -> Result<(), OperatorUsecaseError>;
}
