use crate::core::config::Settings;
use crate::core::errors::StandardError;
use crate::core::security::hash_password;
use crate::core::Response;
use crate::types::Mailer;

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, Responder};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use std::option::Option;
use validator::Validate;

use lettre::{AsyncTransport, Message};

#[derive(Validate, Serialize, Deserialize)]
pub struct RegisterData {
    #[validate(length(max = 15))]
    first_name: String,

    #[validate(length(max = 15))]
    middle_name: Option<String>,

    #[validate(length(max = 15))]
    last_name: String,

    #[validate(email)]
    email: String,

    #[validate(length(min = 8, max = 32))]
    password: String,
}

// TODO: Verification link should point to frontend verification page
pub async fn register_endpoint(
    form_data: actix_web::web::Form<RegisterData>,
    db_pool: actix_web::web::Data<PgPool>,
    mail_client: actix_web::web::Data<Mailer>,
    configs: actix_web::web::Data<Settings>,
) -> Response<impl Responder> {
    form_data.validate()?;

    let existing_user = sqlx::query!(
        "SELECT id, is_verified, is_active FROM users WHERE email=$1",
        &form_data.email
    )
    .fetch_optional(db_pool.as_ref())
    .await?;

    if existing_user.is_some() {
        return Err(StandardError {
            id: 1,
            detail: String::from("User with the same email already exists!"),
            status_code: StatusCode::CONFLICT,
        }
        .into());
    }

    sqlx::query!(
        r#"INSERT INTO public.users
        (first_name, middle_name, last_name, email, hashed_password)
        VALUES($1,$2,$3,$4,$5)
    "#,
        form_data.first_name,
        form_data.middle_name,
        form_data.last_name,
        form_data.email,
        hash_password(&form_data.password)?,
    )
    .execute(db_pool.as_ref())
    .await?;

    let template_handler = Handlebars::new();

    let content = template_handler
        .render_template(
            include_str!("../../../templates/verification-email.html"),
            &json!({
                "username": form_data.first_name.clone(),
                "link": format!("{}/verify?token={}", configs.application.get_base_url(), "token"),
            }),
        )
        .unwrap();

    let email = Message::builder()
        .from("noreply@domain.tld".parse().unwrap())
        .to(form_data.email.parse().unwrap())
        .subject("Verification Email")
        .header(lettre::message::header::ContentType::TEXT_HTML)
        .body(content)
        .unwrap();

    let _ = mail_client.as_ref().send(email).await?;

    return Ok(HttpResponse::Ok().finish());
}
