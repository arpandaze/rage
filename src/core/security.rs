use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use base64::encode_config;
use rand_core::{OsRng, RngCore};

pub fn generate_session_token() -> Result<String, crate::core::Errors> {
    let mut session_token = [0u8; 32];

    OsRng.try_fill_bytes(&mut session_token)?;

    return Ok(encode_config(session_token, base64::URL_SAFE_NO_PAD));
}

pub fn generate_email_token() -> Result<String, crate::core::Errors> {
    let mut session_token = [0u8; 64];

    OsRng.try_fill_bytes(&mut session_token)?;

    return Ok(encode_config(session_token, base64::URL_SAFE_NO_PAD));
}

#[inline(always)]
pub fn hash_password(password: &String) -> Result<String, crate::core::Errors> {
    let password = password.as_bytes();

    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password, &salt)?.to_string();

    return Ok(password_hash);
}

#[inline(always)]
pub fn verify_password(
    password: &String,
    password_hash: &String,
) -> Result<bool, crate::core::Errors> {
    let parsed_hash = PasswordHash::new(&password_hash)?;
    return Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok());
}
