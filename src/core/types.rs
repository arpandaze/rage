pub type Response<T> = Result<T, crate::core::Errors>;

pub type Mailer = lettre::AsyncSmtpTransport<lettre::Tokio1Executor>;
