use crate::core::Response;
use actix_web::{HttpRequest, HttpResponse, Responder};
use redis::{cmd, AsyncCommands};

pub async fn test_endpoint(
    _req: HttpRequest,
    redis_client: actix_web::web::Data<deadpool_redis::Pool>,
) -> Response<impl Responder> {
    let mut conn: deadpool_redis::Connection = redis_client.get().await.unwrap();

    // let _: String = conn.get("hello").await.unwrap();
    // let _: String = conn.get("hello").await.unwrap();
    let _: String = redis::cmd("GET")
        .arg("hello")
        .query_async(&mut conn)
        .await
        .unwrap();
    // let _: String = cmd("PING").query_async(&mut conn).await.unwrap();

    return Ok(HttpResponse::Ok().body("hello"));
}
