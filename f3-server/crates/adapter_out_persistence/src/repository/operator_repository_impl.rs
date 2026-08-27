use application::domain::model::role_model::Role;
use sqlx::PgPool;

use application::domain::model::operator_model::Operator;
use application::outbound::error::RepositoryError;
use application::outbound::operator_repository::OperatorRepository;

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
    async fn find_by_id(&self, operator_id: String) -> Result<Option<Operator>, RepositoryError> {
        sqlx::query_as!(
            OperatorEntity,
            r#"SELECT 
                  operator_id,
                  name,
                  password_hash,
                  description,
                  created_at,
                  updated_at,
                  role AS "role: RoleEntity",
                  is_enabled,
                  version
              FROM operators
              WHERE operator_id = $1
            "#,
            operator_id.into()
        )
        .fetch_optional(&self.connection)
        .await
        .map(|ooe| ooe.map(|oe| oe.into()))
        .map_err(|e| RepositoryError::Infrastructure(e.into()))
    }

    async fn list(&self) -> Result<Vec<Operator>, RepositoryError> {
        sqlx::query_as!(
            OperatorEntity,
            r#"SELECT 
                  operator_id,
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
        id: String,
        password_hash: String,
        name: String,
        description: String,
        role: Role,
        is_enabled: bool,
    ) -> Result<Operator, RepositoryError> {
        sqlx::query_as!(
            OperatorEntity,
            r#"INSERT INTO operators (operator_id, password_hash, name, description, role, is_enabled)
               VALUES (
                    $1,
                    $2,
                    $3,
                    $4,
                    $5,
                    $6
               ) RETURNING 
                operator_id,
                name,
                password_hash,
                description, 
                role AS "role: RoleEntity",
                is_enabled,
                version,
                created_at,
                updated_at; 
            "#,
            id.into(),
            password_hash,
            name,
            description,
            role.to_string(),
            is_enabled 
        )
        .fetch_one(&self.connection)
        .await
        .map(|oe| oe.into())
        .map_err(|e| RepositoryError::Infrastructure(e.into()))
    }

    async fn save(&self, operator: Operator) -> Result<Operator, RepositoryError> {
        sqlx::query_as!(
            OperatorEntity,
            r#"UPDATE operators
               SET password_hash = $2,
                   name = $3,
                   description = $4,
                   role = $5,
                   is_enabled = $6
               WHERE operator_id = $1
               RETURNING 
                operator_id,
                name,
                password_hash,
                description, 
                role AS "role: RoleEntity",
                is_enabled,
                version,
                created_at,
                updated_at; 
            "#,
            operator.operator_id.into(),
            operator.password_hash,
            operator.name,
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
