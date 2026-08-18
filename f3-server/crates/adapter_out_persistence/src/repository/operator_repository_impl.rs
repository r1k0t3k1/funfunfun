use application::domain::model::role_model::Role;
use sqlx::PgPool;

use application::domain::model::operator_model::Operator;
use application::port::outbound::error::RepositoryError;
use application::port::outbound::operator_repository::OperatorRepository;

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
                  description,
                  created_at,
                  updated_at,
                  role AS "role: RoleEntity"
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
                  description,
                  created_at,
                  updated_at,
                  role AS "role: RoleEntity"
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
        password: String,
        name: String,
        description: String,
        role: Role,
    ) -> Result<Operator, RepositoryError> {
        sqlx::query_as!(
            OperatorEntity,
            r#"INSERT INTO operators (operator_id, password_hash, name, description, role)
               VALUES (
                    $1,
                    crypt($2, gen_salt('bf')),
                    $3,
                    $4,
                    $5
               ) RETURNING 
                operator_id,
                name,
                description, 
                role AS "role: RoleEntity",
                created_at,
                updated_at; 
            "#,
            id.into(),
            password,
            name,
            description,
            role.to_string(),
        ) 
        .fetch_one(&self.connection)
        .await
        .map(|oe| oe.into())
        .map_err(|e| RepositoryError::Infrastructure(e.into()))
    }

    async fn find_by_credential(
        &self,
        operator_id: String,
        password: String,
    ) -> Result<Option<Operator>, RepositoryError> {
        let mut tx = self
            .connection
            .begin()
            .await
            .map_err(|e| RepositoryError::Infrastructure(e.into()))?;

        let operator = sqlx::query_as!(
            OperatorEntity,
            r#"SELECT 
                  operator_id,
                  name,
                  description,
                  role AS "role: RoleEntity",
                  created_at,
                  updated_at
              FROM operators
              WHERE operator_id = $1
            "#,
            operator_id.into(),
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| RepositoryError::Infrastructure(e.into()))?
        .ok_or_else(|| RepositoryError::NotFound)?;

        let is_password_match: bool = sqlx::query_scalar!(
            r#"SELECT
                   (password_hash = crypt($1, password_hash)) AS password_match 
               FROM operators
               WHERE operator_id = $2;
            "#,
            password.into(),
            operator.operator_id,
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| RepositoryError::Infrastructure(e.into()))?
        .unwrap_or(false);

        tx.commit()
            .await
            .map_err(|e| RepositoryError::Infrastructure(e.into()))?;

        match is_password_match {
            true => Ok(Some(operator.into())),
            false => Err(RepositoryError::NotFound),
        }
    }
}
