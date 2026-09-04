use crate::{domain::error::DomainError, outbound::error::{C2Error, RepositoryError}};

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

    #[error("Listener not found")]
    NotFound,
    
    #[error(transparent)]
    Repository(#[from] RepositoryError),

    #[error(transparent)]
    Unexpected(#[from] C2Error),
}

#[derive(Debug, thiserror::Error)]
pub enum OperatorUsecaseError {
    #[error("Invalid current password")]
    InvalidCurrentPassword,

    #[error("Operator not found")]
    OperatorNotFound,

    #[error("Failed to register Operator")]
    FailedToRegisterOperator,

    #[error(transparent)]
    RepositoryError(#[from] RepositoryError),
}

#[derive(Debug, thiserror::Error)]
pub enum AgentUsecaseError {
    #[error("Agent not found")]
    AgentNotFound,

    #[error(transparent)]
    Unexpected(#[from] C2Error),
}
// TODO
// ユースケースごとのエラーを定義
