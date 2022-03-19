use crate::core::constants;
use crate::core::security::{generate_session_token, verify_password};
use crate::types::*;

use actix_web::{
    cookie::Cookie,
    http::StatusCode,
    web::{Data, Form},
    HttpResponse,
};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct LoginData {
    #[validate(email)]
    email: String,

    #[validate(length(min = 8, max = 32))]
    password: String,

    remember_me: Option<bool>,
}

pub async fn web_login_endpoint(
    form_data: Form<LoginData>,
    db_pool: Data<PgPool>,
    _mail_client: Data<Mailer>,
    redis_pool: Data<RedisPool>,
    configs: Data<Settings>,
) -> Response {
    form_data.validate()?;

    let optional_user = sqlx::query!(
        "\
            SELECT id, two_fa_secret, hashed_password, is_verified, is_active \
            FROM users \
            WHERE email=$1;\
        ",
        &form_data.email
    )
    .fetch_optional(db_pool.as_ref())
    .await?;

    if optional_user.is_none() {
        return Err(Errors::standard(
            "Invalid login credentials!",
            StatusCode::UNAUTHORIZED,
        ));
    }

    let user = optional_user.unwrap();

    if verify_password(&form_data.password, &user.hashed_password.unwrap())? == false {
        return Err(Errors::standard(
            "Invalid username or password!",
            StatusCode::UNAUTHORIZED,
        ));
    }

    match (user.is_active.unwrap(), user.is_verified.unwrap()) {
        (true, false) => {
            return Err(Errors::standard(
                "Account verification required before logging in!",
                StatusCode::FORBIDDEN,
            ));
        }

        (false, _) => {
            return Err(Errors::standard(
                "Account disabled. Please contact admin!",
                StatusCode::FORBIDDEN,
            ));
        }

        _ => (),
    }

    match user.two_fa_secret {
        Some(two_fa_secret) => {
            let token = generate_session_token()?;

            let mut redis_client = redis_pool.get().await?;

            let ttl = if form_data.remember_me.unwrap_or(false) {
                configs.ttl.session_token_long
            } else {
                configs.ttl.session_token_short
            };

            let key = format!("{}_{}", constants::TWO_FA_LOGIN_PREFIX, token);
            let value = json!(
                {
                    "user": user.id.to_string(),
                    "ttl": ttl,
                }
            );

            redis_client
                .set_ex(&key, value.to_string(), configs.ttl.two_fa_login_timeout)
                .await?;

            println!("{:?}", &key);

            let obj = json!(
                {
                    "message": "2FA required before proceeding!",
                    "two_fa_required": true,
                }
            );

            let cookie = Cookie::build("session", &token)
                .domain(&configs.application.domain)
                .path("/")
                .secure(if configs.application.protocal == "https" {
                    true
                } else {
                    false
                })
                .http_only(true)
                .finish();

            return Ok(HttpResponse::Ok().cookie(cookie).json(obj));
        }
        None => {
            let session_token = generate_session_token()?;

            let mut redis_client = redis_pool.get().await?;

            let ttl = if form_data.remember_me.unwrap_or(false) {
                configs.ttl.session_token_long
            } else {
                configs.ttl.session_token_short
            };

            redis_client
                .set_ex(&session_token, user.id.to_string(), ttl)
                .await?;

            let obj = json!(
                {
                    "message": "Successfully logged in!",
                    "two_fa_required": false,
                }
            );

            let cookie = Cookie::build("session", &session_token)
                .domain(&configs.application.domain)
                .path("/")
                .secure(if configs.application.protocal == "https" {
                    true
                } else {
                    false
                })
                .http_only(true)
                .finish();

            return Ok(HttpResponse::Ok().cookie(cookie).json(obj));
        }
    }
}
