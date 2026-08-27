use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, ToSchema, PartialEq)]
pub struct AuthenticateRequest {
    pub operator_id: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, ToSchema, PartialEq)]
pub struct AuthenticatedResponse {
    pub access_token: String,
}
