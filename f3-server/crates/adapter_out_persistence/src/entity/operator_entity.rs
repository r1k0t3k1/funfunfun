use sqlx::types::chrono;

use domain::model::operator_model::Operator;

use crate::entity::role_entity::RoleEntity;

#[derive(Clone)]
pub struct OperatorEntity {
    pub operator_id: String,
    pub name: String,
    pub description: Option<String>,
    pub role: RoleEntity,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Into<Operator> for OperatorEntity {
    fn into(self) -> Operator {
        Operator {
            operator_id: self.operator_id,
            name: self.name,
            description: self.description,
            role: self.role.into(),
        }
    }
}
