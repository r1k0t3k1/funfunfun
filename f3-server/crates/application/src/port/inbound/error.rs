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
    Domain(#[from] DomainError),
}

#[derive(Debug, thiserror::Error)]
pub enum ListenerUsecaseError {
    #[error("Address already in use")]
    AddressAlreadyInUse,

    #[error("Invalid Address")]
    InvalidAddress,

    #[error("Failed to start the listener")]
    FailedToStart,

    #[error("Failed to stop the listener")]
    FailedToStop,

    #[error("Failed to remove the listener")]
    FailedToRemove,
}
// TODO
// ユースケースごとのエラーを定義
