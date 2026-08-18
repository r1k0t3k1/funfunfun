use crate::domain::model::role_model::Role;

pub type OperatorId = String;

#[derive(Clone)]
pub struct Operator {
    pub operator_id: OperatorId,
    pub name: String,
    pub description: Option<String>,
    pub role: Role,
}
