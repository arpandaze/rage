use rage::core::config::get_config;
use rage::init::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_config().expect("Couldn't read the configuration file!");

    let address = format!("localhost:{}", configuration.application.port);
    let listener = TcpListener::bind(address)?;

    let db_connection_pool = configuration.database.get_db_pool().await;
    let redis_connection_pool = configuration.redis.get_redis_pool().await;
    let mailer = configuration.email.get_client().await;

    let _ = run(
        configuration,
        listener,
        db_connection_pool,
        redis_connection_pool,
        mailer,
    )
    .await?;

    Ok(())
}
