use std::sync::Arc;

use crate::{
    domain::model::{
        operator_model::{Operator, OperatorId},
        role_model::Role,
    },
    inbound::{error::OperatorUsecaseError, operator_usecase::OperatorUsecase},
    outbound::operator_repository::OperatorRepository,
};

#[derive(Clone)]
pub struct OperatorService {
    operator_repository: Arc<dyn OperatorRepository>,
}

impl OperatorService {
    pub fn new(
        operator_repository: Arc<dyn OperatorRepository>,
    ) -> Self {
        Self {
            operator_repository,
        }
    }
}

#[async_trait::async_trait]
impl OperatorUsecase for OperatorService {
    async fn list_operators(&self) -> Result<Vec<Operator>, OperatorUsecaseError> {
        self.operator_repository
            .list()
            .await
            .map_err(|e| OperatorUsecaseError::RepositoryError(e.into()))
    }

    async fn get_operator(
        &self,
        operator_id: OperatorId,
    ) -> Result<Option<Operator>, OperatorUsecaseError> {
        self.operator_repository
            .find_by_id(operator_id.to_string())
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
        self.operator_repository
            .insert(operator_id.to_string(), password, name, description, role, false)
            .await
            .map_err(|e| OperatorUsecaseError::RepositoryError(e.into()))
    }

    async fn toggle_status(&self, operator_id: OperatorId) -> Result<Operator, OperatorUsecaseError> {
        let mut operator = self.
            operator_repository
            .find_by_id(operator_id)
            .await
            .map_err(|e| OperatorUsecaseError::RepositoryError(e.into()))?
            .ok_or(OperatorUsecaseError::OperatorNotFound)?;

        operator.is_enabled = !operator.is_enabled;

        self.operator_repository.save(operator)
            .await
            .map_err(|e| OperatorUsecaseError::RepositoryError(e.into()))
    }

    async fn change_password(
        &self,
        operator_id: OperatorId,
        current_password: String,
        new_password: String,
    ) -> Result<(), OperatorUsecaseError> {
        let operator = self
            .operator_repository
            .find_by_id(operator_id)
            .await
            .map_err(|e| OperatorUsecaseError::RepositoryError(e.into()))?
            .ok_or_else(|| OperatorUsecaseError::OperatorNotFound)?;

        Ok(())
    }
}
