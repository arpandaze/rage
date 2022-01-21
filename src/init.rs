use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};

use crate::core::config::Settings;
use crate::endpoint::auth::auth_endpoints;
use crate::endpoint::utils::utils_endpoints;
use crate::types::{ Mailer, RedisPool };
use sqlx::PgPool;
use std::net::TcpListener;

pub async fn run(
    configs: Settings,
    tcp_listener: TcpListener,
    db_pool: PgPool,
    redis_pool: RedisPool,
    mailer: Mailer,
) -> Result<(), std::io::Error> {
    let configs_data = Data::new(configs);
    let db_pool_data = Data::new(db_pool);
    let redis_pool_data = Data::new(redis_pool);
    let mailer = Data::new(mailer);

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
