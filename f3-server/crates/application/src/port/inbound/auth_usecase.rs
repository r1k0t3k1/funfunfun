use crate::domain::model::{operator_model::Operator, session_model::Session};

use super::error::AuthUsecaseError;

#[async_trait::async_trait]
pub trait AuthUsecase: Send + Sync {
    async fn authenticate_operator(
        &self,
        operator_id: String,
        password: String,
    ) -> Result<Session, AuthUsecaseError>;

    async fn is_valid_session(
        &self,
        session_id: String,
    ) -> Result<bool, AuthUsecaseError>;

    async fn get_operator_from_session(
        &self,
        session_id: String,
    ) -> Result<Option<Operator>, AuthUsecaseError>;

    async fn logout(&self, session_id: String) -> Result<(), AuthUsecaseError>;
}
