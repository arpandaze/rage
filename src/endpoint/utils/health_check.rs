use crate::core::Response;
use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn health_check_endpoint(_req: HttpRequest) -> Response<impl Responder> {
    return Ok(HttpResponse::Ok()
        .content_type("application/health+json")
        .body(r#"{ "status": "ok" }"#));
}
