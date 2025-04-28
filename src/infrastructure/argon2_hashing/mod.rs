use anyhow::{Result, anyhow};
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

pub fn hash_password(password: String) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let bytes_password = password.as_bytes();

    let argon2 = Argon2::default();
    let hashed = argon2
        .hash_password(bytes_password, &salt)
        .map_err(|e| anyhow!("Hashing failed: {}", e.to_string()))?;

    Ok(hashed.to_string())
}

pub fn verify_password(password: String, hashed_password: String) -> Result<bool> {
    let parsed_hash = PasswordHash::new(&hashed_password)
        .map_err(|e| anyhow!("Parsing failed: {}", e.to_string()))?;
    let bytes_password = password.as_bytes();

    Ok(Argon2::default()
        .verify_password(bytes_password, &parsed_hash)
        .is_ok())
}
