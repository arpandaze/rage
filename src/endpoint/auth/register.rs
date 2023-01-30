use crate::core::security::{generate_email_token, hash_password};
use crate::types::*;

use crate::utils::validator::password_validator;
use actix_web::{
    http::StatusCode,
    web::{Data, Form},
    HttpResponse,
};
use handlebars::Handlebars;
use lettre::{AsyncTransport, Message};
use redis::AsyncCommands;
use secrecy::{ExposeSecret, SecretString};
use secrecy::{Secret, SerializableSecret};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use std::option::Option;
use validator::Validate;

#[derive(Validate, Deserialize)]
pub struct RegisterData {
    #[validate(length(max = 15))]
    first_name: String,

    #[validate(length(max = 15))]
    middle_name: Option<String>,

    #[validate(length(max = 15))]
    last_name: String,

    #[validate(email)]
    email: String,

    password: SecretString,
}

// TODO: Verification link should point to frontend verification page
pub async fn register_endpoint(
    form_data: Form<RegisterData>,
    db_pool: Data<PgPool>,
    mail_client: Data<Mailer>,
    redis_pool: Data<RedisPool>,
    configs: Data<Settings>,
) -> Response {
    form_data.validate()?;
    password_validator(&form_data.password)?;

    let existing_user = sqlx::query!("SELECT id FROM users WHERE email=$1", &form_data.email)
        .fetch_optional(db_pool.as_ref())
        .await?;

    if existing_user.is_some() {
        return Err(Errors::standard(
            "User with the same email already exists!",
            StatusCode::CONFLICT,
        ));
    }

    let user = sqlx::query!(
        "\
            INSERT INTO users \
            (first_name, middle_name, last_name, email, hashed_password) \
            VALUES($1,$2,$3,$4,$5) \
            RETURNING id;\
        ",
        form_data.first_name,
        form_data.middle_name,
        form_data.last_name,
        form_data.email,
        hash_password(form_data.password.expose_secret())?,
    )
    .fetch_one(db_pool.as_ref())
    .await?;

    let verification_token = generate_email_token()?;

    let mut redis_connection = redis_pool.get().await?;

    let key = format!("evtoken_{verification_token}");

    redis_connection
        .set_ex(&key, &user.id.to_string(), configs.ttl.verification_token)
        .await?;

    let template_handler = Handlebars::new();

    let content = template_handler
        .render_template(
            include_str!("../../../templates/verification-email-web.html"),
            &json!({
                "username": form_data.first_name.clone(),
                "link": format!("{}/verify?token={}", configs.application.get_base_url(), verification_token),
            }),
        )
        .unwrap();

    let email = Message::builder()
        .from(configs.email.noreply_email.parse().unwrap())
        .to(form_data.email.parse().unwrap())
        .subject("Verification Email")
        .header(lettre::message::header::ContentType::TEXT_HTML)
        .body(content)
        .unwrap();

    mail_client.as_ref().send(email).await?;

    let obj = json!(
        {
            "message": "Account successfully created. Check your email for verification link!"
        }
    );

    Ok(HttpResponse::Created().json(obj))
}
