use crate::core::Response;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde_json::json;

pub async fn health_check_endpoint(_req: HttpRequest) -> Response<impl Responder> {
    let data = json!({"status":"ok"});
    return Ok(HttpResponse::Ok()
        .content_type("application/health+json")
        .body(serde_json::to_string(&data).unwrap()));
}
