#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    // TODO 後ほどエラーを追加していく、現状はインフラエラーのラッパーで十分
    #[error(transparent)]
    Infrastructure(#[from] anyhow::Error),

    #[error("Invalid credentials")]
    InvalidCredentials,
}
