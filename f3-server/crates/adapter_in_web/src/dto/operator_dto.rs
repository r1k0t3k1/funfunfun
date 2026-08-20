use std::pin::Pin;

use actix_web::{FromRequest, HttpMessage, HttpRequest, dev::Payload};
use application::domain::model::operator_model::Operator;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{dto::role_dto::Role, error::ApiError};

#[derive(Deserialize, ToSchema)]
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

#[derive(Deserialize, ToSchema)]
pub struct OperatorCredential {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthOperator {
    pub operator_id: String,
    pub name: String,
    pub description: Option<String>,
    pub role: Role,
}

impl AuthOperator {
    pub fn has_role(&self, role: &Role) -> bool {
        self.role == *role
    }
}

impl From<Operator> for AuthOperator {
    fn from(value: Operator) -> Self {
        Self {
            operator_id: value.operator_id,
            name: value.name,
            description: value.description,
            role: value.role.into(),
        }
    }
}

impl FromRequest for AuthOperator {
    type Error = ApiError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let request = req.clone();
        let operator = request.extensions().get::<AuthOperator>().cloned();
        Box::pin(async move { operator.ok_or_else(|| ApiError::Unauthorized) })
    }
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
