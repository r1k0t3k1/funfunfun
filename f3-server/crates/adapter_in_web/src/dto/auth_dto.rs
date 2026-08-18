use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema, PartialEq)]
pub struct AuthenticatedResponse {
    pub access_token: String,
}
