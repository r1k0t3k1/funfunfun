use crate::domain::error::DomainError;

#[derive(Debug, thiserror::Error)]
pub enum AuthUsecaseError {
    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("Session expired")]
    SessionExpired,

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),

    #[error(transparent)]
    Domain(#[from] DomainError)
}

#[derive(Debug, thiserror::Error)]
pub enum ListenerUsecaseError {
}

// TODO
// ユースケースごとのエラーを定義
