use crate::endpoint::auth::auth_endpoints;
use crate::endpoint::utils::utils_endpoints;
use crate::types::*;

use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
// use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub async fn run(
    configs: Settings,
    tcp_listener: TcpListener,
    db_pool: PgPool,
    redis_pool: RedisPool,
    mailer: Mailer,
) -> Result<(), std::io::Error> {
    LogTracer::init().expect("Failed to set logger");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let formatting_layer = BunyanFormattingLayer::new("rage".into(), std::io::stdout);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber).expect("Failed to set subscriber");

    let configs_data = Data::new(configs);
    let db_pool_data = Data::new(db_pool);
    let redis_pool_data = Data::new(redis_pool);
    let mailer = Data::new(mailer);

    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(auth_endpoints)
            .configure(utils_endpoints)
            .app_data(configs_data.clone())
            .app_data(db_pool_data.clone())
            .app_data(redis_pool_data.clone())
            .app_data(mailer.clone())
    })
    .listen(tcp_listener)?
    .run()
    .await?;

    return Ok(server);
}
