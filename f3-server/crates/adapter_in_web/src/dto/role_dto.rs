use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use application::domain::model::role_model::Role as DomainRole;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub enum Role {
    Admin,
    Write,
    Read,
}

impl From<DomainRole> for Role {
    fn from(value: DomainRole) -> Self {
        match value {
            DomainRole::Admin => Self::Admin,
            DomainRole::Write => Self::Write,
            DomainRole::Read => Self::Read,
        }
    }
}
