use crate::core::constants;
use crate::core::security::{generate_2fa_secret_token, generate_session_token, get_user};
use crate::types::*;

use actix_web::{
    cookie::Cookie,
    http::StatusCode,
    web::{Data, Form},
    HttpRequest, HttpResponse,
};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use std::time::SystemTime;
use totp_rs::{Algorithm, TOTP};
use uuid::Uuid;
use validator::Validate;

pub async fn enable_2fa_request(
    req: HttpRequest,
    db_pool: Data<PgPool>,
    mail_client: Data<Mailer>,
    redis_pool: Data<RedisPool>,
    configs: Data<Settings>,
) -> Response {
    let user_id = get_user(&req, &redis_pool).await?;

    let db_user = sqlx::query!(
        "SELECT email, two_fa_secret FROM users WHERE id=$1::uuid;",
        &user_id
    )
    .fetch_one(db_pool.as_ref())
    .await?;

    if db_user.two_fa_secret.is_some() {
        return Err(Errors::standard(
            "2FA already active!",
            StatusCode::CONFLICT,
        ));
    }

    let two_fa_secret = crate::core::security::generate_2fa_secret_token()?;
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        two_fa_secret.clone().into(),
        Some(db_user.email),
        configs.application.name.clone(),
    )?;

    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let url = totp.get_url();

    let temp_token = crate::core::security::generate_2fa_secret_token()?;

    let mut redis_client = redis_pool.get().await?;

    redis::pipe()
        .set_ex::<String, String>(
            format!("{}_{}", constants::TWO_FA_USER_PREFIX, temp_token),
            user_id.to_string(),
            configs.ttl.two_fa_enable_timeout,
        )
        .set_ex::<String, String>(
            format!("{}_{}", constants::TWO_FA_SECRET_PREFIX, temp_token),
            two_fa_secret,
            configs.ttl.two_fa_enable_timeout,
        )
        .query_async(&mut redis_client)
        .await?;

    let obj = json!(
        {
            "two_fa_url": url,
            "token": temp_token,
            "message": "2FA url has been successfully generated!",
        }
    );

    Ok(HttpResponse::Ok().json(obj))
}

#[derive(Validate, Serialize, Deserialize)]
pub struct TwoFAConfirmForm {
    #[validate(length(max = 15))]
    token: String,

    #[validate(length(max = 15))]
    totp: String,
}

pub async fn enable_2fa_confirm(
    req: HttpRequest,
    form_data: Form<TwoFAConfirmForm>,
    db_pool: Data<PgPool>,
    mail_client: Data<Mailer>,
    redis_pool: Data<RedisPool>,
    configs: Data<Settings>,
) -> Response {
    let user_id = get_user(&req, &redis_pool).await?;

    let mut redis_client = redis_pool.get().await?;

    let (two_fa_user_opt, two_fa_secret_opt): (Option<String>, Option<String>) = redis::pipe()
        .get::<String>(format!(
            "{}_{}",
            constants::TWO_FA_USER_PREFIX,
            form_data.token
        ))
        .get::<String>(format!(
            "{}_{}",
            constants::TWO_FA_SECRET_PREFIX,
            form_data.token
        ))
        .query_async(&mut redis_client)
        .await?;

    match (two_fa_secret_opt, two_fa_user_opt) {
        (Some(two_fa_secret), Some(two_fa_user)) => {
            let totp = TOTP::new(
                Algorithm::SHA1,
                6,
                1,
                30,
                two_fa_secret.clone().into(),
                None,
                String::new(),
            )?;

            let time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let totp_valid = totp.check(&form_data.totp, time);

            if !totp_valid {
                return Err(Errors::standard(
                    "Invalid TOTP token",
                    StatusCode::FORBIDDEN,
                ));
            }

            redis::pipe()
                .del(format!(
                    "{}_{}",
                    constants::TWO_FA_USER_PREFIX,
                    form_data.token
                ))
                .del(format!(
                    "{}_{}",
                    constants::TWO_FA_SECRET_PREFIX,
                    form_data.token
                ))
                .query_async(&mut redis_client)
                .await?;

            let db_user = sqlx::query!(
                "UPDATE users SET two_fa_secret=$1 WHERE id=$2::uuid;",
                two_fa_secret,
                Uuid::parse_str(&two_fa_user).unwrap()
            )
            .execute(db_pool.as_ref())
            .await?;
        }
        _ => {
            return Err(Errors::standard(
                "Invalid or expired token",
                StatusCode::FORBIDDEN,
            ));
        }
    }

    let obj = json!(
        {
            "message": "2FA has been successfully enabled!",
        }
    );

    Ok(HttpResponse::Ok().json(obj))
}

// TODO: Send email on 2FA disable ??
pub async fn disable_2fa(
    req: HttpRequest,
    db_pool: Data<PgPool>,
    mail_client: Data<Mailer>,
    redis_pool: Data<RedisPool>,
    configs: Data<Settings>,
) -> Response {
    let user_id = get_user(&req, &redis_pool).await?;

    let db_user = sqlx::query!(
        "SELECT two_fa_secret FROM users WHERE id=$1::uuid;",
        &user_id
    )
    .fetch_one(db_pool.as_ref())
    .await?;

    if db_user.two_fa_secret.is_none() {
        return Err(Errors::standard("2FA not enabled", StatusCode::CONFLICT));
    }

    sqlx::query!(
        "UPDATE users SET two_fa_secret=$1 WHERE id=$2::uuid;",
        Option::<&str>::None,
        user_id,
    )
    .execute(db_pool.as_ref())
    .await?;

    let obj = json!(
        {
            "message": "2FA has been successfully disabled!",
        }
    );

    Ok(HttpResponse::Ok().json(obj))
}

#[derive(Validate, Serialize, Deserialize)]
pub struct TwoFALoginVerifyForm {
    totp: String,
}

pub async fn two_fa_login_verify(
    req: HttpRequest,
    form_data: Form<TwoFALoginVerifyForm>,
    db_pool: Data<PgPool>,
    mail_client: Data<Mailer>,
    redis_pool: Data<RedisPool>,
    configs: Data<Settings>,
) -> Response {
    match req.cookie("session") {
        Some(cookie) => {
            let mut redis_client = redis_pool.get().await?;

            let key = format!("{}_{}", constants::TWO_FA_LOGIN_PREFIX, cookie.value());
            println!("{:?}", &key);

            #[derive(Deserialize)]
            struct Login2FATemp {
                user: String,
                ttl: usize,
            }

            let value = redis_client.get::<&String, String>(&key).await?;

            let value_struct = serde_json::from_str::<Login2FATemp>(value.as_str()).unwrap();

            let user = sqlx::query!(
                "\
                    SELECT two_fa_secret \
                    FROM users \
                    WHERE id=$1::uuid;\
                ",
                &uuid::Uuid::parse_str(value_struct.user.as_str())?
            )
            .fetch_one(db_pool.as_ref())
            .await?;

            let two_fa_secret = user.two_fa_secret.as_ref().unwrap();

            let totp = TOTP::new(
                Algorithm::SHA1,
                6,
                1,
                30,
                two_fa_secret.clone().into(),
                None,
                String::new(),
            )?;

            let time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let totp_valid = totp.check(&form_data.totp, time);

            if !totp_valid {
                return Err(Errors::standard(
                    "Invalid TOTP token",
                    StatusCode::FORBIDDEN,
                ));
            }

            let session_token = generate_session_token()?;

            redis::pipe()
                .set_ex(&session_token, value_struct.user, value_struct.ttl)
                .del(&key)
                .query_async(&mut redis_client)
                .await?;

            let obj = json!(
                {
                    "message": "Successfully logged in!",
                }
            );

            let cookie = Cookie::build("session", &session_token)
                .domain(&configs.application.domain)
                .path("/")
                .secure(configs.application.protocal == "https")
                .http_only(true)
                .finish();

            return Ok(HttpResponse::Ok().cookie(cookie).json(obj));
        }

        None => Err(Errors::standard("Invalid request!", StatusCode::FORBIDDEN)),
    }
}
