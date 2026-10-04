use argon2::{
    Argon2,
    password_hash::phc::PasswordHash,
    password_hash::{PasswordHasher, PasswordVerifier},
};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let hash = Argon2::default()
        .hash_password(password.as_bytes())?
        .to_string();

    Ok(hash)
}

pub fn verify_password(
    password: &str,
    password_hash: &str,
) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(password_hash)?;

    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
}

#[cfg(test)]
mod tests {
    use super::{hash_password, verify_password};

    #[test]
    fn accepts_the_correct_password() {
        let hash = hash_password("a-long-test-password").unwrap();

        assert!(verify_password("a-long-test-password", &hash).is_ok());
    }

    #[test]
    fn rejects_the_wrong_password() {
        let hash = hash_password("a-long-test-password").unwrap();

        assert!(verify_password("wrong-password", &hash).is_err());
    }
}
