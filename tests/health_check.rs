use actix_backend::health_check;
use actix_web::http;
use actix_web::test::TestRequest;

#[actix_rt::test]
async fn health_check_works() {
    let req = TestRequest::with_header("content-type", "text/plain").to_http_request();
    let resp = health_check(req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
}
