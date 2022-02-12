use crate::types::*;

use crate::types::RedisPool;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct VerificationData {
    token: String,
}

// TODO: Send email informing account has been verified
pub async fn verify_endpoint(
    form_data: actix_web::web::Form<VerificationData>,
    db_pool: actix_web::web::Data<PgPool>,
    redis_pool: actix_web::web::Data<RedisPool>,
) -> Response {
    let mut redis_conn = redis_pool.get().await?;

    let key = format!("evtoken_{}", &form_data.token);

    let user_id_opt: Option<String> = redis_conn.get_del(key).await?;

    match user_id_opt {
        None => {
            return Err(Errors::standard(
                "Invalid verification token",
                StatusCode::UNAUTHORIZED,
            ));
        }

        Some(user_id) => {
            sqlx::query!(
                r#"
                    UPDATE users
                    SET is_verified=$1
                    WHERE id=$2::uuid;
                "#,
                true,
                Uuid::parse_str(&user_id).unwrap(),
            )
            .execute(db_pool.as_ref())
            .await?;

            let obj = json!(
                {
                    "message": "Account has been successfully verified!"
                }
            );

            return Ok(HttpResponse::Ok().json(obj));
        }
    }
}
