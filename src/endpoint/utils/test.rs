use crate::types::*;

use actix_web::{web::Data, HttpResponse};
use std::time::SystemTime;
use totp_rs::{Algorithm, TOTP};

pub async fn test_endpoint(_redis_client: Data<RedisPool>) -> Response {
    let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, "supersecret");
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let url = totp.get_url("user@example.com", "my-org.com");
    println!("{}", url);
    let token = totp.generate(time);
    println!("{}", token);

    return Ok(HttpResponse::Ok().body("hello"));
}
