// あくまで永続化のエラー、ソフトウェア固有のエラーは外側に露出させない
#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("entity not found")]
    NotFound,
    #[error("conflict (e.g. optimistic lock)")]
    Conflict,
    #[error("infrastructure failure")]
    Infrastructure(#[source] Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Debug, thiserror::Error)]
pub enum HashError {
    #[error("password hashing failed")]
    HashingFailed,
}

#[derive(Debug, thiserror::Error)]
pub enum C2Error {
    #[error("Listener {id} not found")]
    NotFound{id : String},

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}
