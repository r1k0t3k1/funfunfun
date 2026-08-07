use domain::model::role_model::Role;

#[derive(Debug, Clone, sqlx::Type)]
pub enum RoleEntity {
    Admin,
    Write,
    Read,
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
