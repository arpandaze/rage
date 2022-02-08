pub type Response = Result<actix_web::HttpResponse, crate::core::Errors>;

pub type Mailer = lettre::AsyncSmtpTransport<lettre::Tokio1Executor>;

pub type RedisPool = deadpool_redis::Pool;

pub type Errors = crate::core::Errors;

pub type Settings = crate::core::config::Settings;
