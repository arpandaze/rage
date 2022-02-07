use crate::types::*;

use crate::core::security::get_user;
use actix_web::{http::StatusCode, web::HttpRequest, HttpResponse, web::Data};

pub async fn protected_endpoint(req: HttpRequest, redis_pool: Data<RedisPool>) -> Response {
    let user_id = get_user(&req, &redis_pool).await?;
    return Ok(HttpResponse::Ok()
        .content_type("application/health+json")
        .body(r#"{ "status": "ok", "protected": true }"#));
}
