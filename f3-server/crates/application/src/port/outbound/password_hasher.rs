use crate::{domain::model::password_model::{HashedPassword, RawPassword}, port::outbound::error::HashError};

pub trait PasswordHasherTrait: Send + Sync {
    fn hash(&self, raw: &RawPassword) -> Result<HashedPassword, HashError>;
    fn verify(&self, raw: &RawPassword, hashed: &HashedPassword) -> bool;
}


