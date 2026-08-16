use crate::model::role_model::Role;

#[derive(Clone)]
pub struct Operator {
    pub operator_id: String,
    pub name: String,
    pub description: Option<String>,
    pub role: Role,
}
