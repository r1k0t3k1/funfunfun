use application::{domain::model::password_model::{HashedPassword, RawPassword}, port::outbound::{error::HashError, password_hasher::PasswordHasherTrait}};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{SaltString, rand_core::OsRng}};


pub struct Argon2PasswordHasher {
    argon2: Argon2<'static>,
}
impl Argon2PasswordHasher {
    pub fn new() -> Self {
        Self { argon2: Argon2::default() }
    }
}

impl PasswordHasherTrait for Argon2PasswordHasher {
    fn hash(&self, raw: &RawPassword) -> Result<HashedPassword, HashError> {
        let salt = SaltString::generate(&mut OsRng);

        let phc = self.argon2
            .hash_password(raw.expose().as_bytes(), &salt)
            .map_err(|_| HashError::HashingFailed)?
            .to_string();

        Ok(HashedPassword::from_phc_string(phc))
    }

    fn verify(&self, raw: &RawPassword, hashed: &HashedPassword) -> bool {
        let Ok(parsed) = PasswordHash::new(hashed.expose_for_persistence()) else {
            return false
        };

        self.argon2
            .verify_password(raw.expose().as_bytes(), &parsed)
            .is_ok()
    }
}
