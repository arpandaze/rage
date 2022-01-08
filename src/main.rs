use actix_backend::core::config::get_config;
use actix_backend::init::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_config().expect("Couldn't read the configuration file!");

    let address = format!("localhost:{}", configuration.application.port);
    let listener = TcpListener::bind(address)?;

    let connection_pool = PgPool::connect(&configuration.database.get_uri())
        .await
        .expect("Failed to connect to database!");

    let _ = run(listener, connection_pool).await?;
    Ok(())
}
