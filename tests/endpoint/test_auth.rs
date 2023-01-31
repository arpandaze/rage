use serde_json::json;

use crate::helpers::spawn_app;
use crate::helpers::{get_email, get_verified_user, get_verified_user_with_token, User};
use reqwest::cookie::Cookie;
use std::{thread, time};

async fn signup_random_user(base_address: String) -> User {
    let user_name = uuid::Uuid::new_v4().to_string();
    let signup_data = User {
        id: None,
        first_name: user_name.get(0..10).unwrap().to_string(),
        middle_name: "Test".to_string(),
        last_name: "User".to_string(),
        email: format!("{user_name}@app.local"),
        password: "testpassword".to_string(),
    };

    let client = reqwest::Client::new();

    let regular_signup = client
        .post(format!("{base_address}/auth/register"))
        .form(&signup_data)
        .send()
        .await
        .unwrap();

    assert!(
        regular_signup.status() == reqwest::StatusCode::CREATED,
        "(Regular) Response Text: {:?}",
        &regular_signup.text().await.unwrap(),
    );
    signup_data
}

#[actix_rt::test]
async fn test_signup() {
    let base_address = spawn_app().await;
    signup_random_user(base_address).await;
}

#[actix_rt::test]
async fn test_verify() {
    let base_address = spawn_app().await;

    let user = signup_random_user(base_address.clone()).await;

    let sleep_time = time::Duration::from_secs(1);
    thread::sleep(sleep_time);

    let client = reqwest::Client::new();

    let email = get_email(user.email.clone()).await.unwrap();

    let mut verify_email_string = String::from(email["Content"]["Body"].as_str().unwrap());
    verify_email_string = verify_email_string.replace("=\r\n", "");

    let start_point = verify_email_string.find("token&#x3D;").unwrap() + 11; // &#x3D; is "="
    let end_point = start_point + 86; // Token length is 86

    let mut token = verify_email_string.get(start_point..end_point);

    let verify_data = json!({
        "token": token.take(),
    });

    let verify_request = client
        .post(format!("{base_address}/auth/verify"))
        .form(&verify_data)
        .send()
        .await
        .unwrap();

    assert!(
        verify_request.status() == reqwest::StatusCode::OK,
        "(Verify) Response Text: {:?}",
        &verify_request.text().await.unwrap(),
    );
}

#[actix_rt::test]
async fn test_login() {
    let base_address = spawn_app().await;
    let user = get_verified_user().await.unwrap();

    let login_data = json!({
        "email": user.email,
        "password": user.password,
    });

    let client = reqwest::Client::new();

    let login_request = client
        .post(format!("{base_address}/auth/login"))
        .form(&login_data)
        .send()
        .await
        .unwrap();

    let cookies: Vec<Cookie> = login_request.cookies().collect();
    assert!(
        login_request.status() == reqwest::StatusCode::OK,
        "(Login) Response Text: {:?}",
        &login_request.text().await.unwrap(),
    );

    assert!(!cookies.is_empty(), "Login didn't return session cookie!");
}

#[actix_rt::test]
async fn test_totp_enable() {
    let base_address = spawn_app().await;
    let (_user, token) = get_verified_user_with_token().await.unwrap();

    let client = reqwest::Client::new();

    let login_request = client
        .get(format!("{base_address}/auth/2fa"))
        .header("Cookie", format!("session={};", token))
        .send()
        .await
        .unwrap();

    assert!(
        login_request.status() == reqwest::StatusCode::OK,
        "(Login) Response Text: {:?}",
        &login_request.text().await.unwrap(),
    );
}
