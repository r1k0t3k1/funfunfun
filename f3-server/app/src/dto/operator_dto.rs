use std::pin::Pin;

use actix_web::{FromRequest, HttpMessage, HttpRequest, dev::Payload};
use domain::model::operator_model::Operator;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{dto::role_dto::Role, error::AppError};

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
    type Error = AppError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let request = req.clone();
        let operator = request.extensions().get::<AuthOperator>().cloned();
        Box::pin(async move { operator.ok_or_else(|| AppError::Unauthorized) })
    }
}
