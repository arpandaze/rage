use actix_web::web;

pub mod forgot_password;
pub mod login;
pub mod register;
pub mod reset_password;
pub mod two_fa;
pub mod update_user;
pub mod verify;

pub use forgot_password::forgot_password;
pub use login::web_login_endpoint;
pub use register::register_endpoint;
pub use reset_password::reset_password;
pub use two_fa::{disable_2fa, enable_2fa_confirm, enable_2fa_request, two_fa_login_verify};
pub use verify::verify_endpoint;

pub fn auth_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(web::resource("/register").route(web::post().to(register_endpoint)))
            .service(web::resource("/verify").route(web::post().to(verify_endpoint)))
            .service(web::resource("/login").route(web::post().to(web_login_endpoint)))
            .service(web::resource("/forgot").route(web::post().to(forgot_password)))
            .service(web::resource("/reset").route(web::post().to(reset_password)))
            .service(
                web::resource("/2fa")
                    .route(web::get().to(enable_2fa_request))
                    .route(web::post().to(enable_2fa_confirm))
                    .route(web::delete().to(disable_2fa)),
            )
            .service(web::resource("/2fa/verify").route(web::post().to(two_fa_login_verify))),
    );
}
