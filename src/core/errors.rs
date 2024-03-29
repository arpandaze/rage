use crate::core::config::CONFIG;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, Error)]
pub struct StandardError {
    pub detail: String,
    pub status_code: StatusCode,
}

impl std::fmt::Display for StandardError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.detail)
    }
}

#[derive(Debug, Display, Error)]
pub enum Errors {
    #[display(fmt = "Validation Error: {_0}")]
    Validation(validator::ValidationError),

    #[display(fmt = "Validation Errors: {_0}")]
    Validations(validator::ValidationErrors),

    #[display(fmt = "SQL Error: {_0}")]
    SQL(sqlx::Error),

    #[display(fmt = "Password Verification Error: {_0}")]
    PasswordVerificationError(argon2::password_hash::Error),

    #[display(fmt = "SMTP Error: {_0}")]
    SMTPError(lettre::transport::smtp::Error),

    #[display(fmt = "RNG Error: {_0}")]
    RNGError(rand_core::Error),

    #[display(fmt = "Redis Pool Error: {_0}")]
    RedisPoolError(deadpool_redis::PoolError),

    #[display(fmt = "Redis Error: {_0}")]
    RedisError(redis::RedisError),

    #[display(fmt = "UUID Error")]
    UUIDError(uuid::Error),

    #[display(fmt = "Standard Error: {_0}")]
    StandardError(StandardError),

    #[display(fmt = "Invalid Session Error")]
    InvalidSessionError,

    #[display(fmt = "TOTP URL Error")]
    TOTPError(totp_rs::TotpUrlError),
}

impl From<validator::ValidationError> for Errors {
    fn from(error: validator::ValidationError) -> Self {
        Errors::Validation(error)
    }
}

impl From<validator::ValidationErrors> for Errors {
    fn from(error: validator::ValidationErrors) -> Self {
        Errors::Validations(error)
    }
}

impl From<sqlx::Error> for Errors {
    fn from(error: sqlx::Error) -> Self {
        Errors::SQL(error)
    }
}

impl From<argon2::password_hash::Error> for Errors {
    fn from(error: argon2::password_hash::Error) -> Self {
        Errors::PasswordVerificationError(error)
    }
}

impl From<lettre::transport::smtp::Error> for Errors {
    fn from(error: lettre::transport::smtp::Error) -> Self {
        Errors::SMTPError(error)
    }
}

impl From<rand_core::Error> for Errors {
    fn from(error: rand_core::Error) -> Self {
        Errors::RNGError(error)
    }
}

impl From<deadpool_redis::PoolError> for Errors {
    fn from(error: deadpool_redis::PoolError) -> Self {
        Errors::RedisPoolError(error)
    }
}

impl From<redis::RedisError> for Errors {
    fn from(error: redis::RedisError) -> Self {
        Errors::RedisError(error)
    }
}

impl From<uuid::Error> for Errors {
    fn from(error: uuid::Error) -> Self {
        Errors::UUIDError(error)
    }
}

impl From<StandardError> for Errors {
    fn from(error: StandardError) -> Self {
        Errors::StandardError(error)
    }
}

impl From<totp_rs::TotpUrlError> for Errors {
    fn from(error: totp_rs::TotpUrlError) -> Self {
        Errors::TOTPError(error)
    }
}

impl ResponseError for Errors {
    #[tracing::instrument(name = "Error Handler")]
    fn error_response(&self) -> HttpResponse {
        let (status_code, body) = match self {
            Self::Validations(e) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                json!(
                    {
                        "message": "Validation Error",
                        "fields": e.field_errors(),
                    }
                ),
            ),

            Self::Validation(e) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                json!(
                    {
                        "message": e.message,
                    }
                ),
            ),

            Self::SQL(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!(
                    {
                        "message": "Database error",
                        "detail": e.to_string(),
                    }
                ),
            ),

            Self::PasswordVerificationError(_) => (
                StatusCode::UNAUTHORIZED,
                json!(
                    {
                        "messsage": "Password verification error",
                    }
                ),
            ),

            Self::SMTPError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!(
                    {
                        "messsage": "Email error",
                    }
                ),
            ),

            Self::RNGError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!(
                    {
                        "messsage": "Unexpected RNG error",
                    }
                ),
            ),

            Self::RedisPoolError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!(
                    {
                        "messsage": "Unexpected pool error",
                    }
                ),
            ),

            Self::RedisError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!(
                    {
                        "messsage": "Unexpected cache error",
                    }
                ),
            ),

            Self::UUIDError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!(
                    {
                        "messsage": "Unexpected UUID error",
                    }
                ),
            ),

            Self::StandardError(e) => (
                e.status_code,
                json!(
                    {
                        "message": e.detail,
                    }
                ),
            ),

            Self::InvalidSessionError => {
                let json_body = json!(
                    {
                        "message": "Invalid or expired session",
                    }
                );

                let mut response = HttpResponse::build(StatusCode::UNAUTHORIZED).json(json_body);

                let cookie_to_remove = actix_web::cookie::Cookie::build("session", "")
                    .domain(CONFIG.application.domain.to_owned())
                    .path("/")
                    .finish();

                response.add_removal_cookie(&cookie_to_remove).unwrap();

                return response;
            }

            Self::TOTPError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!(
                    {
                        "message": "An unknown error occured!",
                    }
                ),
            ),
        };

        HttpResponse::build(status_code).json(body)
    }
}

impl Errors {
    pub fn standard(message: &str, status_code: StatusCode) -> Errors {
        Errors::StandardError(StandardError {
            detail: message.to_string(),
            status_code,
        })
    }
}
