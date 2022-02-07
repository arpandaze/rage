use serde_json::json;

#[inline(always)]
async fn clear_emails() {
    let client = reqwest::Client::new();

    let _ = client
        .delete("http://localhost:8025/api/v1/messages")
        .send()
        .await
        .unwrap();
}

#[actix_rt::test]
async fn test_signup() {
    clear_emails().await;

    let signup_data = json!({
        "first_name": "Test",
        "last_name": "User",
        "email": "testuser@app.local",
        "password": "testpassword",
    });

    let signup_data_middle_name = json!({
        "first_name": "Test",
        "middle_name": "Middle",
        "last_name": "User",
        "email": "testuser2@app.local",
        "password": "testpassword",
    });

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

    clear_emails().await;

    let signup_with_middle_name = client
        .post("http://localhost:8000/auth/register")
        .form(&signup_data_middle_name)
        .send()
        .await
        .unwrap();

    assert!(
        signup_with_middle_name.status() == reqwest::StatusCode::CREATED,
        "(Middle) Response Text: {:?}",
        &regular_signup.text().await.unwrap(),
    );

    let already_exist_signup = client
        .post("http://localhost:8000/auth/register")
        .form(&signup_data)
        .send()
        .await
        .unwrap();

    assert!(
        already_exist_signup.status() == reqwest::StatusCode::CONFLICT,
        "(Conflict) Response Text: {:?}",
        &regular_signup.text().await.unwrap(),
    );
}

#[actix_rt::test]
async fn test_verify(){
    todo!()
}
