use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use log::{error, warn};
use usecase::error::UsecaseError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Resource not found")]
    NotFound,

    #[error("Authentication failed")]
    Unauthorized,

    #[error("Permission required")]
    Forbidden,

    #[error("validation error: {0}")]
    Validation(String),

    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),

    #[error(transparent)]
    UsecaseError(#[from] UsecaseError),
}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Unauthorized | AppError::UsecaseError(UsecaseError::Unauthorized) => {
                StatusCode::UNAUTHORIZED
            }
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::Database(_) | AppError::Unexpected(_) | AppError::UsecaseError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let status_code = self.status_code();
        let error_message = match self.status_code() {
            StatusCode::INTERNAL_SERVER_ERROR => {
                error!("{self:?}");
                "Internal Server Error".to_string()
            }
            _ => {
                warn!("{status_code} - {self:?}");
                self.to_string()
            }
        };
        HttpResponse::build(status_code).json(serde_json::json!({"message": error_message}))
    }
}
