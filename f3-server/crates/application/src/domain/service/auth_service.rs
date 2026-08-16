use std::sync::Arc;

use crate::{domain::model::{operator_model::Operator, session_model::Session}, port::{inbound::{auth_usecase::AuthUsecase, error::AuthUsecaseError}, outbound::{operator_repository::OperatorRepository, session_repository::SessionRepository}}};

#[derive(Clone)]
pub struct AuthService {
    operator_repository: Arc<dyn OperatorRepository>,
    session_repository: Arc<dyn SessionRepository>,
}

impl AuthService {
    pub fn new(
        operator_repository: Arc<dyn OperatorRepository>,
        session_repository: Arc<dyn SessionRepository>,
    ) -> Self {
        Self {
            operator_repository,
            session_repository,
        }
    }
}

#[async_trait::async_trait]
impl AuthUsecase for AuthService {
    async fn authenticate_operator(
        &self,
        operator_id: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<Session, AuthUsecaseError> {
        let operator = self
            .operator_repository
            .find_by_credential(operator_id.into(), password.into())
            .await
            .map_err(|e| {
                log::error!("{e}");
                match e {
                    DomainError::InvalidCredentials => AuthUsecaseError::Unauthorized,
                    _ => AuthUsecaseError::Unexpected(e.into()),
                }
            })?
            .ok_or(AuthUsecaseError::Unauthorized)?;

        self.session_repository
            .insert(operator.operator_id)
            .await
            .map_err(|e| AuthUsecaseError::Unexpected(e.into()))
    }

    async fn is_valid_session(
        &self,
        session_id: impl Into<String>,
    ) -> Result<bool, AuthUsecaseError> {
        let session = self
            .session_repository
            .find_by_id(session_id.into())
            .await
            .map_err(|e| AuthUsecaseError::Unexpected(e.into()))?
            .ok_or_else(|| AuthUsecaseError::Unauthorized)?;

        if session.is_expired() {
            return Ok(false);
        }

        Ok(true)
    }

    async fn get_operator_from_session(
        &self,
        session_id: impl Into<String>,
    ) -> Result<Option<Operator>, AuthUsecaseError> {
        let session = self
            .session_repository
            .find_by_id(session_id.into())
            .await
            .map_err(|e| AuthUsecaseError::Unexpected(e.into()))?
            .ok_or_else(|| AuthUsecaseError::Unauthorized)?;

        if session.is_expired() {
            return Err(AuthUsecaseError::SessionExpired);
        }

        let operator = self
            .operator_repository
            .find_by_id(session.operator_id)
            .await
            .map_err(|e| AuthUsecaseError::Domain(e.into()))?;

        Ok(operator)
    }

    async fn logout(&self, session_id: String) -> Result<(), AuthUsecaseError> {
        self.session_repository
            .delete_by_id(session_id)
            .await
            .map_err(|e| AuthUsecaseError::Unexpected(e.into()))
    }
}

