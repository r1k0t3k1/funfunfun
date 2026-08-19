use std::sync::Arc;

use crate::{
    domain::model::{operator_model::Operator, password_model::RawPassword, session_model::Session},
    port::{
        inbound::{auth_usecase::AuthUsecase, error::AuthUsecaseError},
        outbound::{
            error::RepositoryError, operator_repository::OperatorRepository, password_hasher::{self, PasswordHasherTrait}, session_repository::SessionRepository
        },
    },
};

#[derive(Clone)]
pub struct AuthService {
    operator_repository: Arc<dyn OperatorRepository>,
    session_repository: Arc<dyn SessionRepository>,
    password_hasher: Arc<dyn PasswordHasherTrait>,
}

impl AuthService {
    pub fn new(
        operator_repository: Arc<dyn OperatorRepository>,
        session_repository: Arc<dyn SessionRepository>,
        password_hasher: Arc<dyn PasswordHasherTrait>,
    ) -> Self {
        Self {
            operator_repository,
            session_repository,
            password_hasher,
        }
    }
}

#[async_trait::async_trait]
impl AuthUsecase for AuthService {
    async fn authenticate_operator(
        &self,
        operator_id: String,
        password: String,
    ) -> Result<Session, AuthUsecaseError> {
        // ユーザ列挙防止のため最初にハッシュ化
        let password_hash = self.password_hasher
            .clone()
            .hash(&RawPassword::new(password).map_err(|e| AuthUsecaseError::AuthenticationFailed)?)
            .map_err(|e| AuthUsecaseError::AuthenticationFailed)?;

        let operator = self
            .operator_repository
            .find_by_id(operator_id)
            .await
            .map_err(|e| {
                log::error!("{e}");
                match e {
                    RepositoryError::NotFound => AuthUsecaseError::AuthenticationFailed,
                    _ => AuthUsecaseError::Unexpected(e.into()),
                }
            })?
            .ok_or(AuthUsecaseError::AuthenticationFailed)?;
        
        if !operator.verify_password(password_hash) {
           return Err(AuthUsecaseError::AuthenticationFailed) 
        }

        self.session_repository
            .insert(operator.operator_id)
            .await
            .map_err(|e| AuthUsecaseError::Unexpected(e.into()))
    }

    async fn is_valid_session(&self, session_id: String) -> Result<bool, AuthUsecaseError> {
        let session = self
            .session_repository
            .find_by_id(session_id.into())
            .await
            .map_err(|e| AuthUsecaseError::Unexpected(e.into()))?
            .ok_or_else(|| AuthUsecaseError::AuthenticationFailed)?;

        if session.is_expired() {
            return Ok(false);
        }

        Ok(true)
    }

    async fn get_operator_from_session(
        &self,
        session_id: String,
    ) -> Result<Option<Operator>, AuthUsecaseError> {
        let session = self
            .session_repository
            .find_by_id(session_id.into())
            .await
            .map_err(|e| AuthUsecaseError::Unexpected(e.into()))?
            .ok_or_else(|| AuthUsecaseError::AuthenticationFailed)?;

        if session.is_expired() {
            return Err(AuthUsecaseError::SessionExpired);
        }

        let operator = self
            .operator_repository
            .find_by_id(session.operator_id)
            .await
            .map_err(|e| AuthUsecaseError::Unexpected(e.into()))?;

        Ok(operator)
    }

    async fn logout(&self, session_id: String) -> Result<(), AuthUsecaseError> {
        self.session_repository
            .delete_by_id(session_id)
            .await
            .map_err(|e| AuthUsecaseError::Unexpected(e.into()))
    }
}
