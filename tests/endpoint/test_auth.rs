use serde_json::json;

use crate::helpers::{get_unverified_user, get_verified_user};

#[actix_rt::test]
async fn test_signup() {
    get_unverified_user().await;
}

#[actix_rt::test]
async fn test_verify() {
    get_verified_user().await;
}

#[actix_rt::test]
async fn test_login() {
    let user = get_verified_user().await;

    let login_data = json!({
        "email": user.email,
        "password": user.password,
    });

    let client = reqwest::Client::new();

    let login_request = client
        .post("http://localhost:8000/auth/login")
        .form(&login_data)
        .send()
        .await
        .unwrap();

    assert!(
        login_request.status() == reqwest::StatusCode::OK,
        "(Login) Response Text: {:?}",
        &login_request.text().await.unwrap(),
    );
}
