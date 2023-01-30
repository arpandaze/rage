use crate::core::constants;
use crate::types::*;

use crate::core::security::generate_reset_token;
use actix_web::{
    http::StatusCode,
    web::{Data, Form},
    HttpResponse,
};
use handlebars::Handlebars;
use lettre::{AsyncTransport, Message};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct ForgotPasswordForm {
    #[validate(email)]
    email: String,
}

pub async fn forgot_password(
    form_data: Form<ForgotPasswordForm>,
    db_pool: actix_web::web::Data<PgPool>,
    mail_client: actix_web::web::Data<Mailer>,
    redis_pool: actix_web::web::Data<RedisPool>,
    configs: actix_web::web::Data<Settings>,
) -> Response {
    let user_opt = sqlx::query!(
        "SELECT id, first_name FROM users WHERE email=$1",
        &form_data.email
    )
    .fetch_optional(db_pool.as_ref())
    .await?;

    match user_opt {
        None => Err(Errors::standard(
            "Account associated with the provided email not found",
            StatusCode::NOT_FOUND,
        )),

        Some(user) => {
            let reset_token = generate_reset_token()?;

            let key = format!("{}_{}", constants::PASSWORD_RESET_PREFIX, reset_token);

            let mut redis_client = redis_pool.get().await?;

            redis_client
                .set_ex(key, &user.id.to_string(), configs.ttl.password_reset)
                .await?;

            let template_handler = Handlebars::new();

            let content = template_handler.render_template(
                include_str!("../../../templates/forgot-password.html"),
                &json!(
                    {
                    "username": user.first_name,
                    "link": format!("{}/reset?token={}", configs.application.get_base_url(), reset_token),
                    }
                ),
            )
            .unwrap();

            let email = Message::builder()
                .from(configs.email.noreply_email.parse().unwrap())
                .to(form_data.email.parse().unwrap())
                .subject("Account Recovery")
                .header(lettre::message::header::ContentType::TEXT_HTML)
                .body(content)
                .unwrap();

            mail_client.as_ref().send(email).await?;

            let obj = json!(
                {
                    "message": "Check your email for password reset link"
                }
            );

            Ok(HttpResponse::Ok().json(obj))
        }
    }
}
