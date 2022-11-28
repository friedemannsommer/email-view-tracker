use argon2::{PasswordHasher, PasswordVerifier};

pub static SALT: once_cell::sync::OnceCell<argon2::password_hash::SaltString> =
    once_cell::sync::OnceCell::new();
static ARGON2: once_cell::sync::Lazy<argon2::Argon2<'_>> =
    once_cell::sync::Lazy::new(argon2::Argon2::default);

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    Ok(ARGON2
        .hash_password(password.as_bytes(), &SALT.get().unwrap())?
        .to_string())
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    ARGON2
        .verify_password(
            password.as_bytes(),
            &match argon2::PasswordHash::new(password_hash) {
                Ok(hash) => hash,
                Err(_) => return false,
            },
        )
        .is_ok()
}
