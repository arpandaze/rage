use crate::core::Response;
use actix_web::{HttpRequest, HttpResponse, Responder};

use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

pub async fn test_endpoint(_req: HttpRequest) -> Response<impl Responder> {
    let email_temp = std::fs::read_to_string("templates/verification-email.html").unwrap();
    let email = Message::builder()
        .from("noreply@domain.tld".parse().unwrap())
        .to("hei@domain.tld".parse().unwrap())
        .subject("Verification Email")
        .header(lettre::message::header::ContentType::TEXT_HTML)
        .body(email_temp)
        .unwrap();

    let creds = Credentials::new("smtp_username".to_string(), "smtp_password".to_string());

    // Open a remote connection to gmail
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous("localhost")
        .port(1025)
        .credentials(creds)
        .build();

    // Send the email
    let _ = mailer.send(email).await;

    return Ok(HttpResponse::Ok());
}
