use actix_backend::endpoint::utils::health_check_endpoint;
use actix_web::http;
use actix_web::test::TestRequest;

#[actix_rt::test]
async fn health_check_works() {
    let req = TestRequest::default()
        .insert_header(("content-type", "text/plain"))
        .to_http_request();
    let resp = health_check_endpoint(req).await.unwrap();
    assert_eq!(resp.status(), http::StatusCode::OK);
}
