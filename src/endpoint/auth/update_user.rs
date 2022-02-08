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
use validator::Validate;

pub async fn update_user(
    db_pool: actix_web::web::Data<PgPool>,
    mail_client: actix_web::web::Data<Mailer>,
    redis_pool: actix_web::web::Data<RedisPool>,
    configs: actix_web::web::Data<Settings>,
) -> Response {
    

    let obj = json!(
        {
            "message": ""
        }
    );

    return Ok(
        HttpResponse::Ok()
              .json(obj)
    );
    
}

