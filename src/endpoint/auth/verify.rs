use crate::core::config::Settings;
use crate::core::errors::Errors;
use crate::core::security::{generate_email_token, hash_password};
use crate::core::Response;
use crate::types::Mailer;

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, Responder};
use handlebars::Handlebars;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use std::option::Option;
use validator::Validate;
use uuid::Uuid;

use lettre::{AsyncTransport, Message};

#[derive(Validate, Serialize, Deserialize)]
pub struct VerificationData {
    token: String,
}

pub async fn verify_endpoint(
    form_data: actix_web::web::Form<VerificationData>,
    db_pool: actix_web::web::Data<PgPool>,
    mail_client: actix_web::web::Data<Mailer>,
    redis_pool: actix_web::web::Data<deadpool_redis::Pool>,
    configs: actix_web::web::Data<Settings>,
) -> Response<impl Responder> {
    let mut redis_conn = redis_pool.get().await?;

    let user_id: Option<String> = redis_conn.get(&form_data.token).await?;

    if user_id.is_none() {
        return Err(Errors::standard(
            123,
            "Invalid verification token",
            StatusCode::UNAUTHORIZED,
        ));
    }

    sqlx::query!(
        r#"
            UPDATE users
            SET is_verified=$1
            WHERE id=$2::uuid;"#,
            true,
            Uuid::parse_str(&user_id.unwrap()).unwrap(),
    )
    .execute(db_pool.as_ref())
    .await?;

    return Ok(HttpResponse::Ok().finish());
}
