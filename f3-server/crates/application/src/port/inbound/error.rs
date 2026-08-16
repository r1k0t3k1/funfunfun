use domain::error::DomainError;

#[derive(Debug, thiserror::Error)]
pub enum UsecaseError {
    #[error("Resource not found")]
    NotFound,

    #[error("Authentication failed")]
    Unauthorized,

    #[error("Session expired")]
    SessionExpired,

    #[error("validation error: {0}")]
    Validation(String),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),

    #[error(transparent)]
    Domain(#[from] DomainError),
}

//impl From<DomainError> for UsecaseError {
//    fn from(e: DomainError) -> Self { Self(e) }
//}
