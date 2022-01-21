pub type Response<T> = Result<T, crate::core::Errors>;

pub type Mailer = lettre::AsyncSmtpTransport<lettre::Tokio1Executor>;

pub type RedisPool = r2d2::Pool<redis::Client>;

pub type RedisPooledConnection = r2d2::PooledConnection<redis::Client>;

