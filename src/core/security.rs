use crate::types::*;

use actix_web::{
    http::StatusCode,
    web::{Data, Form},
    HttpResponse,
};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use data_encoding::BASE64URL;
use rand_core::{OsRng, RngCore};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use validator::Validate;

pub fn generate_session_token() -> Result<String, crate::core::Errors> {
    let mut session_token = [0u8; 64];

    OsRng.try_fill_bytes(&mut session_token)?;

    Ok(BASE64URL.encode(&session_token))
}

pub fn generate_email_token() -> Result<String, crate::core::Errors> {
    let mut session_token = [0u8; 64];

    OsRng.try_fill_bytes(&mut session_token)?;

    Ok(BASE64URL.encode(&session_token))
}

pub fn generate_reset_token() -> Result<String, crate::core::Errors> {
    let mut session_token = [0u8; 32];

    OsRng.try_fill_bytes(&mut session_token)?;

    Ok(BASE64URL.encode(&session_token))
}

pub fn generate_2fa_secret_token() -> Result<String, crate::core::Errors> {
    let mut session_token = [0u8; 64];

    OsRng.try_fill_bytes(&mut session_token)?;

    Ok(BASE64URL.encode(&session_token))
}

#[inline(always)]
pub fn hash_password(password: &str) -> Result<String, crate::core::Errors> {
    let password = password.as_bytes();

    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password, &salt)?.to_string();

    Ok(password_hash)
}

#[inline(always)]
pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, crate::core::Errors> {
    let parsed_hash = PasswordHash::new(password_hash)?;
    return Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok());
}

// MAYBE: Replace with procedural macro
#[inline(always)]
pub async fn get_user(
    req: &actix_web::HttpRequest,
    redis_pool: &RedisPool,
) -> Result<uuid::Uuid, Errors> {
    match req.cookie("session") {
        None => Err(Errors::standard(
            "Please login before accessing this endpoint!",
            StatusCode::UNAUTHORIZED,
        )),

        Some(cookie) => {
            let mut redis_connection = redis_pool.get().await?;

            let user_id: Option<String> = redis_connection.get(cookie.value()).await?;

            if user_id.is_none() {
                return Err(Errors::InvalidSessionError);
            }

            Ok(uuid::Uuid::parse_str(user_id.unwrap().as_str()).unwrap())
        }
    }
}
