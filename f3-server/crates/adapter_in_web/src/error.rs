use actix_web::HttpResponse;
use actix_web::{error::ResponseError, mime};
use application::port::inbound::error::AuthUsecaseError;
use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    InternelServerError,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::BadRequest => write!(f, "BadRequest"),
            ApiError::Unauthorized => write!(f, "Unauthorized"),
            ApiError::Forbidden => write!(f, "Forbidden"),
            ApiError::NotFound => write!(f, "NotFound"),
            ApiError::InternelServerError => write!(f, "InternelServerError"),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .content_type(mime::APPLICATION_JSON)
            .body(self.to_string())
    }
}

impl From<AuthUsecaseError> for ApiError {
    fn from(value: AuthUsecaseError) -> Self {
        ApiError::InternelServerError // TODO
    }
}
