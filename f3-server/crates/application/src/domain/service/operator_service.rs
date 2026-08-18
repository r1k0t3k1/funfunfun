use std::sync::Arc;

use crate::{
    domain::model::{operator_model::{OperatorId, Operator}, role_model::Role, session_model::Session},
    port::{
        inbound::{error::OperatorUsecaseError, operator_usecase::OperatorUsecase},
        outbound::{
            operator_repository::OperatorRepository,
            session_repository::SessionRepository,
        },
    },
};

#[derive(Clone)]
pub struct OperatorService {
    operator_repository: Arc<dyn OperatorRepository>,
    session_repository: Arc<dyn SessionRepository>,
}

impl OperatorService {
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
impl OperatorUsecase for OperatorService {

    async fn list_operators(&self) -> Result<Vec<Operator>, OperatorUsecaseError> {
        self.operator_repository.list()
            .await
            .map_err(|e| OperatorUsecaseError::RepositoryError(e.into()))
    }

    async fn get_operator(&self,operator_id: OperatorId) -> Result<Option<Operator>, OperatorUsecaseError> {
        self.operator_repository.find_by_id(operator_id.to_string())
            .await
            .map_err(|e| OperatorUsecaseError::RepositoryError(e.into()))
    }

    async fn register_operator(
        &self,
        operator_id: OperatorId,
        password: String,
        name: String,
        description: String,
        role: Role,
    ) -> Result<Operator, OperatorUsecaseError> {
        self.operator_repository.insert(
            operator_id.to_string(),
            password,
            name,
            description,
            role
        )
            .await
            .map_err(|e| OperatorUsecaseError::RepositoryError(e.into()))
    }

    async fn disable_operator(&self, operator_id: OperatorId) -> Result<(), OperatorUsecaseError> {
        todo!()
    }

    async fn enable_operator(&self, operator_id: OperatorId) -> Result<(),OperatorUsecaseError> {
        todo!()
    }

    async fn change_password(
        &self,
        operator_id: OperatorId,
        current_password: String,
        new_password: String
    ) -> Result<(), OperatorUsecaseError> {
        todo!()
    }
}

