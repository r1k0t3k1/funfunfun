use application::domain::model::operator_model::Operator;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams,ToSchema};

#[derive(Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetOperatorRequest {
    pub operator_id: String,
}

#[derive(Deserialize, ToSchema)]
pub struct ToggleOperatorStatusRequest {
    pub operator_id: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdatePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Serialize, ToSchema)]
pub struct OperatorResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub role: String,
    pub is_enabled: bool,
}

impl From<Operator> for OperatorResponse {
    fn from(value: Operator) -> Self {
        Self {
            id: value.operator_id,
            name: value.name,
            description: value.description.unwrap_or("".to_string()),
            role: value.role.to_string(),
            is_enabled: value.is_enabled,
        }
    }
}
