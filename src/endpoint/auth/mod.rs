use actix_web::web;

pub mod register;

pub use register::register_endpoint;

#[rustfmt::skip]
pub fn auth_endpoints(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::resource("/register")
            .route(web::post().to(register_endpoint))
    );
}
