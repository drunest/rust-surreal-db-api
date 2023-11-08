pub mod password {
    use crate::app_error::AppError;
    use argon2::{
        password_hash::{rand_core::OsRng, SaltString},
        Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    };

    pub fn hash_password(password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hash_password() {
        // Test a valid password hash
        let password = "my_password";
        let hashed_password = password::hash_password(password).unwrap();
        assert_ne!(hashed_password, password);
        let verify = password::verify(&password, &hashed_password).unwrap();
        assert!(verify);

        let error_password = "somesafepassword";
        let hashed = password::hash_password(error_password).unwrap();
        let v = password::verify(&error_password, &hashed).unwrap();
        assert!(v);
    }

    #[test]
    fn test_verify() {
        // Test a valid password verification
        let password = "RadhaKrsna";
        let hash = password::hash_password(&password).unwrap();

        let verified = password::verify(password, &hash).unwrap();

        let verify_2 = password::verify("Wrong", &hash).unwrap();

        assert!(verified, "Verified");
        assert!(!verify_2, "Wrong Password");
    }
}
