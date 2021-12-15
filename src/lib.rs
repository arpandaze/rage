use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest};

pub async fn health_check(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run();

    return Ok(server);
}
