use std::fmt::Display;

#[derive(Clone, PartialEq)]
pub enum Role {
    Admin,
    Write,
    Read,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Role::Admin => "Admin",
            Role::Write => "Write",
            Role::Read => "Read",
        };
        write!(f, "{}", str)
    }
}
