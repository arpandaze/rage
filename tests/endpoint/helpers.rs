use serde::{Deserialize, Serialize};
use std::net::TcpListener;

use rage::core::config::CONFIG;
use rage::core::security;
pub use rage::core::Errors;
use rage::init::run;
use redis::AsyncCommands;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Option<uuid::Uuid>,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

pub async fn spawn_app() -> String {
    // Port zero chooses random available port automatically
    let address = format!("{}:0", CONFIG.application.host);

    let listener = TcpListener::bind(address).expect("Failed to bind address!");
    let assigned_port = listener.local_addr().unwrap().port();

    let db_connection_pool = CONFIG.database.get_db_pool().await;
    let redis_connection_pool = CONFIG.redis.get_redis_pool().await;
    let mailer = CONFIG.email.get_client().await;

    let _ = actix_web::rt::spawn(
        run(
            CONFIG.clone(),
            listener,
            db_connection_pool,
            redis_connection_pool,
            mailer,
            false,
        )
        .expect("Failed to launch testing server!"),
    );

    format!("http://{}:{}", CONFIG.application.host, assigned_port,)
}

#[inline(always)]
pub async fn clear_emails() {
    let client = reqwest::Client::new();

    client
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

    return serde_json::from_str(test.text().await.unwrap().as_str()).unwrap();
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
    None
}

pub async fn get_verified_user() -> Result<User, Errors> {
    let db_pool = CONFIG.database.get_db_pool().await;

    let user_name = uuid::Uuid::new_v4().to_string();
    let mut user = User {
        id: None,
        first_name: user_name.get(0..10).unwrap().to_string(),
        middle_name: "Test".to_string(),
        last_name: "User".to_string(),
        email: format!("{user_name}@app.local"),
        password: "testpassword".to_string(),
    };

    let user_id = sqlx::query!(
        "\
            INSERT INTO users \
            (first_name, middle_name, last_name, email, hashed_password, is_verified, is_active) \
            VALUES($1,$2,$3,$4,$5,$6,$7)\
            RETURNING id;\
        ",
        user.first_name,
        user.middle_name,
        user.last_name,
        user.email,
        security::hash_password(&user.password).unwrap(),
        true,
        true,
    )
    .fetch_one(&db_pool)
    .await?;

    user.id = Some(user_id.id);

    Ok(user)
}

pub async fn get_verified_user_with_token() -> Result<(User, String), Errors> {
    let user = get_verified_user().await?;

    let session_token = security::generate_session_token()?;

    let redis_pool = CONFIG.redis.get_redis_pool().await;
    let mut redis_client = redis_pool.get().await?;

    redis_client
        .set_ex::<String, String, ()>(
            session_token.clone(),
            user.id.unwrap().to_string(),
            CONFIG.ttl.session_token_long,
        )
        .await
        .unwrap();

    Ok((user, session_token))
}
