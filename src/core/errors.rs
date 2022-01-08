use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};
use serde_json::json;

#[derive(Debug, Display, Error)]
pub enum Errors {
    #[display(fmt = "Validation Error: {}", _0)]
    Validation(validator::ValidationErrors),

    #[display(fmt = "SQL Error: {}", _0)]
    SQL(sqlx::Error),

    #[display(fmt = "Password Verification Error: {}", _0)]
    PasswordVerificationError(argon2::password_hash::Error),
}

impl From<validator::ValidationErrors> for Errors {
    fn from(error: validator::ValidationErrors) -> Self {
        return Errors::Validation(error);
    }
}

impl From<sqlx::Error> for Errors {
    fn from(error: sqlx::Error) -> Self {
        return Errors::SQL(error);
    }
}


impl From<argon2::password_hash::Error> for Errors {
    fn from(error: argon2::password_hash::Error) -> Self {
        return Errors::PasswordVerificationError(error);
    }
}

impl ResponseError for Errors {
    fn error_response(&self) -> HttpResponse {
        return HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json(match self {
            Self::Validation(e) => json!(
                {
                    "message": "Validation Error",
                    "fields": e.field_errors(),
                }
            ),
            Self::SQL(_) => json!(
                {
                    "message": "Database Error",
                }
            ),
            Self::PasswordVerificationError(_) => json!(
                {
                    "messsage": "Password verification error!",
                }
            ),
        });
    }
}
