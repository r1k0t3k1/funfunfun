use std::sync::Arc;

use crate::{
    domain::model::{
        operator_model::Operator, password_model::{HashedPassword, RawPassword}, session_model::Session,
    },
    inbound::{auth_usecase::AuthUsecase, error::AuthUsecaseError},
    outbound::{
        error::RepositoryError,
        operator_repository::OperatorRepository,
        password_hasher::{self, PasswordHasherTrait},
        session_repository::SessionRepository,
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
            })?;

        // レスポンスタイムによるユーザ列挙を防ぐ
        let Some(operator) = operator else {
            let _ = self.password_hasher.hash(&RawPassword::new("dummydummy".to_string()).unwrap());
            return Err(AuthUsecaseError::AuthenticationFailed)
        };

        if operator.is_enabled == false {
            let _ = self.password_hasher.hash(&RawPassword::new("dummydummy".to_string()).unwrap());
            return Err(AuthUsecaseError::AuthenticationFailed);
        }
        
        let raw_password = RawPassword::new(password)
            .map_err(|_| AuthUsecaseError::AuthenticationFailed)?;

        let password_match = self.password_hasher.verify(&raw_password, &HashedPassword::from_phc_string(operator.password_hash));

        if password_match == false {
            return Err(AuthUsecaseError::AuthenticationFailed);
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
