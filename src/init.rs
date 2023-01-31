use crate::core::telemetry::{get_subscriber, init_subscriber};
use crate::endpoint::auth::auth_endpoints;
use crate::endpoint::utils::utils_endpoints;
use crate::types::*;

use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{dev::Server, App, HttpServer};
// use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(
    configs: Settings,
    tcp_listener: TcpListener,
    db_pool: PgPool,
    redis_pool: RedisPool,
    mailer: Mailer,
    logging: bool,
) -> Result<Server, std::io::Error> {
    if logging {
        let subscriber = get_subscriber("rage".to_string(), "info".to_string(), std::io::stdout);
        init_subscriber(subscriber);
    }

    let configs_data = Data::new(configs);
    let db_pool_data = Data::new(db_pool);
    let redis_pool_data = Data::new(redis_pool);
    let mailer = Data::new(mailer);

    #[allow(clippy::let_unit_value)]
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
    .run();

    Ok(server)
}
