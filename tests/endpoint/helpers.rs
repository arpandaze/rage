use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{thread, time};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[inline(always)]
pub async fn clear_emails() {
    let client = reqwest::Client::new();

    let _ = client
        .delete("http://localhost:8025/api/v1/messages")
        .send()
        .await
        .unwrap();
}

#[inline(always)]
pub async fn get_emails() -> serde_json::Value {
    let client = reqwest::Client::new();

    let test = client
        .get("http://localhost:8025/api/v2/messages")
        .send()
        .await
        .unwrap();

    return serde_json::from_str(&test.text().await.unwrap().as_str()).unwrap();
}

#[inline(always)]
pub async fn get_email(for_user: String) -> Option<serde_json::Value> {
    let emails = get_emails().await;

    for index in 0..emails["count"].as_u64().unwrap() {
        let email = &emails["items"][index as usize];

        if email["To"][0]["Mailbox"]
            .as_str()
            .unwrap()
            .to_string()
            .eq(for_user.get(0..36).unwrap())
        {
            return Some(email.to_owned());
        }
    }
    return None;
}

pub async fn get_unverified_user() -> User {
    let user_name = uuid::Uuid::new_v4().to_string();
    let signup_data = User {
        first_name: user_name.get(0..10).clone().unwrap().to_string(),
        middle_name: "Test".to_string(),
        last_name: "User".to_string(),
        email: format!("{}@app.local", user_name),
        password: "testpassword".to_string(),
    };

    let client = reqwest::Client::new();

    let regular_signup = client
        .post("http://localhost:8000/auth/register")
        .form(&signup_data)
        .send()
        .await
        .unwrap();

    assert!(
        regular_signup.status() == reqwest::StatusCode::CREATED,
        "(Regular) Response Text: {:?}",
        &regular_signup.text().await.unwrap(),
    );

    return signup_data;
}

pub async fn get_verified_user() -> User {
    let user = get_unverified_user().await;

    let sleep_time = time::Duration::from_secs(1);
    thread::sleep(sleep_time);

    let client = reqwest::Client::new();

    let email = get_email(user.email.clone()).await.unwrap();

    let verify_email_string = String::from(email["Content"]["Body"].as_str().unwrap());

    let start_point = verify_email_string.find("token&#x3D;").unwrap() + 11; // &#x3D; is "="
    let end_point = start_point + 86; // Token length is 86

    let mut token = verify_email_string.get(start_point..end_point);

    let verify_data = json!({
        "token": token.take(),
    });

    let verify_request = client
        .post("http://localhost:8000/auth/verify")
        .form(&verify_data)
        .send()
        .await
        .unwrap();

    assert!(
        verify_request.status() == reqwest::StatusCode::OK,
        "(Verify) Response Text: {:?}",
        &verify_request.text().await.unwrap(),
    );

    return user;
}
