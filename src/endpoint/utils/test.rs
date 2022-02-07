use crate::types::*;

use actix_web::{web::Data, HttpResponse};

pub async fn test_endpoint(_redis_client: Data<RedisPool>) -> Response {
    return Ok(HttpResponse::Ok().body("hello"));
}
