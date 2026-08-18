use crate::{domain::model::{operator_model::{OperatorId, Operator}, role_model::Role}, port::inbound::error::OperatorUsecaseError};

#[async_trait::async_trait]
pub trait OperatorUsecase: Send + Sync {
    async fn list_operators(&self) -> Result<Vec<Operator>, OperatorUsecaseError>;
    async fn get_operator(&self, operator_id: OperatorId) -> Result<Option<Operator>, OperatorUsecaseError>;
    async fn register_operator(
        &self,
        operator_id: OperatorId,
        password: String,
        name: String,
        description: String,
        role: Role,
    ) -> Result<Operator, OperatorUsecaseError>;
    async fn disable_operator(&self, operator_id: OperatorId) -> Result<(), OperatorUsecaseError>;
    async fn enable_operator(&self, operator_id: OperatorId) -> Result<(), OperatorUsecaseError>;
    async fn change_password(
        &self, 
        operator_id: OperatorId,
        current_password: String,
        new_password: String,
    ) -> Result<(), OperatorUsecaseError>;
}
