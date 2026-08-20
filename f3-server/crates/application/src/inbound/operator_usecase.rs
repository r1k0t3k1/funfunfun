use crate::{
    domain::model::{
        operator_model::{Operator, OperatorId},
        role_model::Role,
    },
    inbound::error::OperatorUsecaseError,
};

#[async_trait::async_trait]
pub trait OperatorUsecase: Send + Sync {
    async fn list_operators(&self) -> Result<Vec<Operator>, OperatorUsecaseError>;
    async fn get_operator(
        &self,
        operator_id: OperatorId,
    ) -> Result<Option<Operator>, OperatorUsecaseError>;
    async fn register_operator(
        &self,
        operator_id: OperatorId,
        password: String,
        name: String,
        description: String,
        role: Role,
    ) -> Result<Operator, OperatorUsecaseError>;
    async fn toggle_status(&self, operator_id: OperatorId) -> Result<Operator, OperatorUsecaseError>;
    async fn change_password(
        &self,
        operator_id: OperatorId,
        current_password: String,
        new_password: String,
    ) -> Result<(), OperatorUsecaseError>;
}
