#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Password must be at least {min} characters")]
    PasswordLengthTooShort {min: usize},

    #[error("Password must be at most {max} characters")]
    PasswordLengthTooLong {max: usize},

    // TODO 後ほどエラーを追加していく、ドメインロジック上のエラー
}
