use application::domain::model::id::OperatorId;
use application::domain::model::role_model::Role;
use sqlx::PgPool;

use application::domain::model::operator_model::OperatorModel;
use application::outbound::error::RepositoryError;
use application::outbound::operator_repository::OperatorRepository;
use sqlx::types::Uuid;

use crate::entity::operator_entity::OperatorEntity;
use crate::entity::role_entity::RoleEntity;

#[derive(Debug, Clone)]
pub struct OperatorRepositoryImpl {
    connection: PgPool,
}

impl OperatorRepositoryImpl {
    pub fn new(connection: PgPool) -> Self {
        Self { connection }
    }
}

#[async_trait::async_trait]
impl OperatorRepository for OperatorRepositoryImpl {
    async fn find_by_id(&self, operator_id: OperatorId) -> Result<Option<OperatorModel>, RepositoryError> {
        sqlx::query_as!(
            OperatorEntity,
            r#"SELECT 
                  id,
                  name,
                  password_hash,
                  description,
                  created_at,
                  updated_at,
                  role AS "role: RoleEntity",
                  is_enabled,
                  version
              FROM operators
              WHERE id = $1
            "#,
            Into::<Uuid>::into(operator_id),
        )
        .fetch_optional(&self.connection)
        .await
        .map(|ooe| ooe.map(|oe| oe.into()))
        .map_err(|e| RepositoryError::Infrastructure(e.into()))
    }

    async fn list(&self) -> Result<Vec<OperatorModel>, RepositoryError> {
        sqlx::query_as!(
            OperatorEntity,
            r#"SELECT 
                  id,
                  name,
                  password_hash,
                  description,
                  role AS "role: RoleEntity",
                  is_enabled,
                  version,
                  created_at,
                  updated_at
              FROM operators
            "#
        )
        .fetch_all(&self.connection)
        .await
        .map(|oes| oes.iter().map(|oe| oe.clone().into()).collect())
        .map_err(|e| RepositoryError::Infrastructure(e.into()))
    }

    async fn insert(
        &self,
        name: String,
        password_hash: String,
        description: String,
        role: Role,
        is_enabled: bool,
    ) -> Result<OperatorModel, RepositoryError> {
        sqlx::query_as!(
            OperatorEntity,
            r#"INSERT INTO operators (name, password_hash, description, role, is_enabled)
               VALUES (
                    $1,
                    $2,
                    $3,
                    $4,
                    $5
               ) RETURNING 
                id,
                name,
                password_hash,
                description, 
                role AS "role: RoleEntity",
                is_enabled,
                version,
                created_at,
                updated_at; 
            "#,
            name,
            password_hash,
            description,
            role.to_string(),
            is_enabled 
        )
        .fetch_one(&self.connection)
        .await
        .map(|oe| oe.into())
        .map_err(|e| RepositoryError::Infrastructure(e.into()))
    }

    async fn save(&self, operator: OperatorModel) -> Result<OperatorModel, RepositoryError> {
        sqlx::query_as!(
            OperatorEntity,
            r#"UPDATE operators
               SET name = $2,
                   password_hash = $3,
                   description = $4,
                   role = $5,
                   is_enabled = $6
               WHERE id = $1
               RETURNING 
                id,
                name,
                password_hash,
                description, 
                role AS "role: RoleEntity",
                is_enabled,
                version,
                created_at,
                updated_at; 
            "#,
            Into::<Uuid>::into(operator.id),
            operator.name,
            operator.password_hash,
            operator.description,
            operator.role.to_string(),
            operator.is_enabled,
        )
        .fetch_one(&self.connection)
        .await
        .map(|oe| oe.into())
        .map_err(|e| RepositoryError::Infrastructure(e.into()))
    }
}
