pub mod password {
    use crate::app_error::AppError;
    use argon2::{
        password_hash::{rand_core::OsRng, SaltString},
        Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    };

    pub fn hash_password(password: &str) -> Result<String, AppError> {
        let password_b = password.as_bytes();
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password_b, &salt)
            .map_err(|err| {
                AppError::InternalError(format!(
                    "Something went wrong hashing password Err: {}",
                    err
                ))
            })?
            .to_string();

        Ok(password_hash)
    }

    pub fn verify(password: &str, hashed: &str) -> Result<bool, AppError> {
        let parsed_hash = PasswordHash::new(hashed).map_err(|err| {
            AppError::InternalError(format!(
                "Something went wrong verifying password Err: {}",
                err
            ))
        })?;

        let argon2 = Argon2::default();

        let verified = argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();

        Ok(verified)
    }
}
