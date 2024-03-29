use rage::core::config::CONFIG;
use rage::init::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = format!("{}:{}", CONFIG.application.host, CONFIG.application.port);
    let listener = TcpListener::bind(address)?;

    let db_connection_pool = CONFIG.database.get_db_pool().await;
    let redis_connection_pool = CONFIG.redis.get_redis_pool().await;
    let mailer = CONFIG.email.get_client().await;

    run(
        CONFIG.clone(),
        listener,
        db_connection_pool,
        redis_connection_pool,
        mailer,
        true,
    )?
    .await
}
