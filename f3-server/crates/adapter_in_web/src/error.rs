use std::fmt::Display;
use actix_web::HttpResponse;
use actix_web::{error::ResponseError, mime};
use application::port::inbound::error::AuthUsecaseError;

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
        todo!()
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
        todo!()
    }
}
