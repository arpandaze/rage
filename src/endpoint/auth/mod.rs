use actix_web::web;

pub mod register;
pub mod verify;

pub use register::register_endpoint;
pub use verify::verify_endpoint;

pub fn auth_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(web::resource("/register").route(web::post().to(register_endpoint)))
            .service(web::resource("/verify").route(web::post().to(verify_endpoint))),
    );
}
