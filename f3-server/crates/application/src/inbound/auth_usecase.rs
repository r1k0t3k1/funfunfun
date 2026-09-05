use crate::domain::model::{id::SessionId, operator_model::OperatorModel, session_model::SessionModel};

use super::error::AuthUsecaseError;

#[async_trait::async_trait]
pub trait AuthUsecase: Send + Sync {
    async fn authenticate_operator(
        &self,
        operator_name: &String,
        password: &String,
    ) -> Result<SessionModel, AuthUsecaseError>;

    async fn is_valid_session(&self, session_id: SessionId) -> Result<bool, AuthUsecaseError>;

    async fn get_operator_from_session(
        &self,
        session_id: SessionId,
    ) -> Result<Option<OperatorModel>, AuthUsecaseError>;

    async fn logout(&self, session_id: SessionId) -> Result<(), AuthUsecaseError>;
}
