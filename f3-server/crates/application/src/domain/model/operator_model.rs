use crate::domain::{
    error::DomainError,
    model::{password_model::HashedPassword, role_model::Role},
};

pub type OperatorId = String;

#[derive(Clone)]
pub struct Operator {
    pub operator_id: OperatorId,
    pub password_hash: String,
    pub name: String,
    pub description: Option<String>,
    pub role: Role,
    pub is_enabled: bool,
    pub version: i64,
}

impl Operator {
    pub fn verify_password(&self, password_hash: HashedPassword) -> bool {
        self.password_hash == password_hash.expose_for_persistence()
    }
    pub fn update_password(
        &self,
        current_password: String,
        new_password: String,
    ) -> Result<(), DomainError> {
        todo!()
    }
}
