use actix_web::web;

pub mod register;
pub mod verify;
pub mod login;
pub mod forgot_password;
pub mod reset_password;
pub mod update_user;

pub use register::register_endpoint;
pub use verify::verify_endpoint;
pub use login::login_endpoint;
pub use forgot_password::forgot_password;
pub use reset_password::reset_password;

pub fn auth_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(web::resource("/register").route(web::post().to(register_endpoint)))
            .service(web::resource("/verify").route(web::post().to(verify_endpoint)))
            .service(web::resource("/login").route(web::post().to(login_endpoint)))
            .service(web::resource("/forgot").route(web::post().to(forgot_password)))
            .service(web::resource("/reset").route(web::post().to(reset_password)))
    );
}
