use application::domain::model::operator_model::OperatorModel;
use sqlx::types::{Uuid, chrono};

use crate::entity::role_entity::RoleEntity;

#[derive(Clone)]
pub struct OperatorEntity {
    pub id: Uuid,
    pub name: String,
    pub password_hash: String,
    pub description: Option<String>,
    pub role: RoleEntity,
    pub is_enabled: bool,
    pub version: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Into<OperatorModel> for OperatorEntity {
    fn into(self) -> OperatorModel {
        OperatorModel {
            id: self.id.into(),
            name: self.name,
            password_hash: self.password_hash,
            description: self.description,
            role: self.role.into(),
            is_enabled: self.is_enabled,
            version: self.version,
        }
    }
}
