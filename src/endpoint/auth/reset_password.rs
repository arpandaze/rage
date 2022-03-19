use crate::core::constants;
use crate::core::security::hash_password;
use crate::types::*;

use actix_web::{
    http::StatusCode,
    web::{Data, Form},
    HttpResponse,
};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct ResetData {
    token: String,

    #[validate(length(min = 8, max = 32))]
    password: String,
}

pub async fn reset_password(
    form_data: Form<ResetData>,
    db_pool: Data<PgPool>,
    mail_client: Data<Mailer>,
    redis_pool: Data<RedisPool>,
    configs: Data<Settings>,
) -> Response {
    let mut redis_conn = redis_pool.get().await?;

    let key = format!("{}_{}", constants::PASSWORD_RESET_PREFIX, &form_data.token);

    let user_id_opt: Option<String> = redis_conn.get_del(key).await?;

    match user_id_opt {
        None => {
            return Err(Errors::standard(
                "Invalid or expired token",
                StatusCode::UNAUTHORIZED,
            ));
        }

        Some(user_id) => {
            sqlx::query!(
                "\
                    UPDATE users \
                    SET hashed_password=$1 \
                    WHERE id=$2::uuid;\
                ",
                hash_password(&form_data.password)?,
                Uuid::parse_str(&user_id)?,
            )
            .execute(db_pool.as_ref())
            .await?;

            let obj = json!(
                {
                    "message": "Your password has been successfully reset"
                }
            );

            return Ok(HttpResponse::Ok().json(obj));
        }
    }
}
