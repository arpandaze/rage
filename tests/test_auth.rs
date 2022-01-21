use actix_backend::endpoint::auth::register_endpoint;
use actix_web::http;
use actix_web::test::TestRequest;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[actix_rt::test]
async fn test_signup() {
    let signup_data = json!(
        {
            "first_name": "Arpan",
            "middle_name": "",
            "last_name": "Koirala",
            "email": "",
            "date_of_birth": "",
        }
    );
    let req = TestRequest::default()
        .insert_header(("content-type", "application/json"))
        .data(signup_data)
        .to_http_request();

    let resp = register_endpoint(req).await?;
    assert_eq!(resp.status(), http::StatusCode::OK);
}
