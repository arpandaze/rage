use actix_web::web;

pub mod health_check;
pub mod test;

pub use health_check::health_check_endpoint;
pub use test::test_endpoint;

pub fn utils_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/utils")
            .service(web::resource("/health").route(web::get().to(health_check_endpoint)))
            .service(web::resource("/test").route(web::get().to(test_endpoint))),
    );
}
