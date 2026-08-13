use std::sync::Arc;

use domain::{
    error::DomainError,
    model::{operator_model::Operator, session_model::Session},
    repository::operator_repository::OperatorRepository,
    repository::session_repository::SessionRepository,
};

use crate::error::UsecaseError;

#[derive(Clone)]
pub struct AuthUsecase {
    operator_repository: Arc<dyn OperatorRepository>,
    session_repository: Arc<dyn SessionRepository>,
}

impl AuthUsecase {
    pub fn new(
        operator_repository: Arc<dyn OperatorRepository>,
        session_repository: Arc<dyn SessionRepository>,
    ) -> Self {
        Self {
            operator_repository,
            session_repository,
        }
    }

    pub async fn authenticate_operator(
        &self,
        operator_id: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<Session, UsecaseError> {
        let operator = self
            .operator_repository
            .find_by_credential(operator_id.into(), password.into())
            .await
            .map_err(|e| {
                log::error!("{e}");
                match e {
                    DomainError::InvalidCredentials => UsecaseError::Unauthorized,
                    _ => UsecaseError::Unexpected(e.into()),
                }
            })?
            .ok_or(UsecaseError::Unauthorized)?;

        self.session_repository
            .insert(operator.operator_id)
            .await
            .map_err(|e| UsecaseError::Unexpected(e.into()))
    }

    pub async fn is_valid_session(
        &self,
        session_id: impl Into<String>,
    ) -> Result<bool, UsecaseError> {
        let session = self
            .session_repository
            .find_by_id(session_id.into())
            .await
            .map_err(|e| UsecaseError::Unexpected(e.into()))?
            .ok_or_else(|| UsecaseError::Unauthorized)?;

        if session.is_expired() {
            return Ok(false);
        }

        Ok(true)
    }

    pub async fn get_operator_from_session(
        &self,
        session_id: impl Into<String>,
    ) -> Result<Option<Operator>, UsecaseError> {
        let session = self
            .session_repository
            .find_by_id(session_id.into())
            .await
            .map_err(|e| UsecaseError::Unexpected(e.into()))?
            .ok_or_else(|| UsecaseError::Unauthorized)?;

        if session.is_expired() {
            return Err(UsecaseError::SessionExpired);
        }

        let operator = self
            .operator_repository
            .find_by_id(session.operator_id)
            .await
            .map_err(|e| UsecaseError::Domain(e.into()))?;

        Ok(operator)
    }

    pub async fn logout(&self, session_id: String) -> Result<(), UsecaseError> {
        self.session_repository
            .delete_by_id(session_id)
            .await
            .map_err(|e| UsecaseError::Unexpected(e.into()))
    }
}
