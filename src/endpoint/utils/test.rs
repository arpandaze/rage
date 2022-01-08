use crate::core::security::hash_password;
use crate::core::Response;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde_json::json;

pub async fn test_endpoint(_req: HttpRequest) -> Response<impl Responder> {
    let hash = hash_password(&String::from("testpassword"))?;
    let data = json!({"status":"ok", "hash": hash});
    return Ok(HttpResponse::Ok()
        .content_type("application/health+json")
        .body(serde_json::to_string(&data).unwrap()));
}
