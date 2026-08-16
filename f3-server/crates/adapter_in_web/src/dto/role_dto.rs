use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub enum Role {
    Admin,
    Write,
    Read,
}

impl From<domain::model::role_model::Role> for Role {
    fn from(value: domain::model::role_model::Role) -> Self {
        match value {
            domain::model::role_model::Role::Admin => Self::Admin,
            domain::model::role_model::Role::Write => Self::Write,
            domain::model::role_model::Role::Read => Self::Read,
        }
    }
}
