use application::domain::model::role_model::Role;

#[derive(Debug, Clone, sqlx::Type)]
pub enum RoleEntity {
    Admin,
    Write,
    Read,
}

impl From<String> for RoleEntity {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Admin" => RoleEntity::Admin,
            "Write" => RoleEntity::Write,
            "Read" => RoleEntity::Read,
            _ => RoleEntity::Read, //TODO
        }
    }
}

impl Into<Role> for RoleEntity {
    fn into(self) -> Role {
        match self {
            RoleEntity::Admin => Role::Admin,
            RoleEntity::Write => Role::Write,
            RoleEntity::Read => Role::Read,
        }
    }
}
