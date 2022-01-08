use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[inline(always)]
pub fn hash_password(password: &String) -> Result<String, crate::core::Errors> {
    let password = password.as_bytes();

    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password, &salt)?.to_string();

    return Ok(password_hash);
}

#[inline(always)]
pub fn verify_password(password: &String, password_hash: &String) -> Result<bool, crate::core::Errors> {
    let parsed_hash = PasswordHash::new(&password_hash)?;
    return Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok());
}
