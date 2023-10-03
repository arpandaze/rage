use serde::{Deserialize, Serialize};

use lazy_static::lazy_static;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, Tokio1Executor};

use deadpool_redis::{Config as RedisConfig, PoolConfig};

use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub redis: RedisSettings,
    pub application: ApplicationSettings,
    pub ttl: TTLSettings,
    pub email: EmailSettings,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationSettings {
    pub name: String,
    pub protocal: String,
    pub domain: String,
    pub host: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TTLSettings {
    pub session_token_short: usize,
    pub session_token_long: usize,
    pub verification_token: usize,
    pub password_reset: usize,
    pub two_fa_enable_timeout: usize,
    pub two_fa_login_timeout: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RedisSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailSettings {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub noreply_email: String,
}

impl ApplicationSettings {
    pub fn get_base_url(&self) -> String {
        if self.port == 80 {
            format!("{}://{}", self.protocal, self.domain)
        } else {
            format!("{}://{}:{}", self.protocal, self.domain, self.port)
        }
    }
}

impl DatabaseSettings {
    pub fn get_uri(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub async fn get_db_pool(&self) -> PgPool {
        let connection_pool = PgPool::connect(&self.get_uri())
            .await
            .expect("Failed to connect to database!");

        connection_pool
    }
}

impl RedisSettings {
    pub fn get_uri(&self) -> String {
        format!(
            "redis://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }

    pub async fn get_redis_pool(&self) -> deadpool_redis::Pool {
        let mut redis_config = RedisConfig::from_url(self.get_uri());
        redis_config.pool = Some(PoolConfig::new(32));
        redis_config
            .create_pool(Some(deadpool_redis::Runtime::Tokio1))
            .unwrap()
    }
}

impl EmailSettings {
    pub async fn get_client(&self) -> crate::types::Mailer {
        let creds = Credentials::new(self.smtp_username.clone(), self.smtp_password.clone());

        AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(self.smtp_host.clone())
            .port(self.smtp_port)
            .credentials(creds)
            .build()
    }
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();

    let base_path = std::env::current_dir().expect("Couldn't determine current directory");
    let config_dir = base_path.join("etc");

    let environment: Environment = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "dev".into())
        .try_into()
        .expect("Failed to detect environment");

    let config_builder = config::Config::builder()
        .add_source(config::File::from(config_dir.join("base")).required(true))
        // Add in the config file for the running environment
        .add_source(config::File::from(config_dir.join(environment.as_str())).required(true))
        // Add in settings from environment variables (with a prefix of APP and '__' as separator)
        // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
        .add_source(config::Environment::with_prefix("app").separator("__"));

    config_builder.build()?.try_deserialize()
}

pub enum Environment {
    Development,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "dev",
            Environment::Production => "prod",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            "dev" => Ok(Self::Development),
            "prod" => Ok(Self::Production),
            other => Err(format!(
                "{other} is not a supported environment. Use either `dev` or `prod`."
            )),
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Settings = get_config().expect("Couldn't read the configuration file!");
}
