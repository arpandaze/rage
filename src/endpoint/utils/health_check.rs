use crate::types::*;

use actix_web::HttpResponse;

pub async fn health_check_endpoint() -> Response {
    return Ok(HttpResponse::Ok()
        .content_type("application/health+json")
        .body(r#"{ "status": "ok" }"#));
}
