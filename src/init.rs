use actix_web::web::Data;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;

use crate::endpoint::auth::auth_endpoints;
use crate::endpoint::utils::utils_endpoints;
use sqlx::PgPool;
use std::net::TcpListener;

pub async fn run(tcp_listener: TcpListener, db_pool: PgPool) -> Result<(), std::io::Error> {
    let db_pool_data = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(auth_endpoints)
            .configure(utils_endpoints)
            .app_data(db_pool_data.clone())
    })
    .listen(tcp_listener)?
    .run()
    .await?;
    return Ok(server);
}
