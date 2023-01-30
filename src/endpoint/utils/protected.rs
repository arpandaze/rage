use crate::types::*;

use crate::core::security::get_user;
use actix_web::{web::Data, HttpRequest, HttpResponse};

pub async fn protected_endpoint(req: HttpRequest, redis_pool: Data<RedisPool>) -> Response {
    let user_id = get_user(&req, &redis_pool).await?;
    return Ok(HttpResponse::Ok()
        .content_type("application/health+json")
        .body(format!(
            r#"{{ "status": "ok", "protected": true, "user": "{user_id}" }}"#
        )));
}
