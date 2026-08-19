use std::fmt::Debug;
use zeroize::Zeroizing;

use crate::domain::error::DomainError;

pub struct RawPassword(Zeroizing<String>);

impl RawPassword {
    pub fn new(value: String) -> Result<Self, DomainError> {
        RawPassword::validate(&value)?;
        Ok(Self(Zeroizing::new(value)))
    }

    fn validate(value: &String) -> Result<(), DomainError> {
        if value.len() < 8 {
            return Err(DomainError::PasswordLengthTooShort { min: 8 })
        }

        if value.len() > 256 {
            return Err(DomainError::PasswordLengthTooLong { max: 256 })
        }
        Ok(())
    }

    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl Debug for RawPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RawPassword: ********")
    }
}

#[derive(Clone)]
pub struct HashedPassword(String);

impl HashedPassword {
    pub fn from_phc_string(phc: String) -> Self {
        Self(phc)
    }
    pub fn from_stored(value: String) -> Self {
        Self(value)
    }
    pub fn expose_for_persistence(&self) -> &str {
        &self.0
    }
}

impl Debug for HashedPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HashedPassword: ********")
    }
}
