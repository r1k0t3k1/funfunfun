use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use actix_web::{error::ResponseError, mime};
use application::inbound::error::AuthUsecaseError;
use serde::Serialize;
use std::fmt::Display;

use crate::response::ResponseBody;

#[derive(Debug, Clone, Serialize, thiserror::Error)]
pub enum ApiError {
    BadRequest { detail: String },
    Unauthorized,
    Forbidden,
    NotFound,
    InternelServerError,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::BadRequest {detail:d} => write!(f, "BadRequest: {d}"),
            ApiError::Unauthorized => write!(f, "Unauthorized"),
            ApiError::Forbidden => write!(f, "Forbidden"),
            ApiError::NotFound  => write!(f, "NotFound"),
            ApiError::InternelServerError => write!(f, "InternelServerError"),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::BadRequest {detail:_}=> StatusCode::BAD_REQUEST,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden => StatusCode::FORBIDDEN,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::InternelServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let error = ResponseBody::error(self.status_code(), self.clone());
        HttpResponse::build(self.status_code())
            .content_type(mime::APPLICATION_JSON)
            .json(error)
    }
}

impl Into<ResponseBody<ApiError>> for ApiError {
    fn into(self) -> ResponseBody<ApiError> {
        ResponseBody::error(self.status_code(), self.clone())
    }
}

impl From<AuthUsecaseError> for ApiError {
    fn from(value: AuthUsecaseError) -> Self {
        match value {
            AuthUsecaseError::AuthenticationFailed | AuthUsecaseError::SessionExpired => {
                ApiError::Unauthorized
            }
            AuthUsecaseError::Unexpected(_) => ApiError::InternelServerError,
            AuthUsecaseError::Domain(e) => ApiError::BadRequest {detail: e.to_string()},
        }
    }
}
