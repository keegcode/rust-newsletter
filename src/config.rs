use std::{num::ParseIntError, sync::LazyLock};

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    dotenvy::dotenv().unwrap();
    Config::from_env().unwrap()
});

#[derive(Debug)]
pub struct DbConfig {
    pub user: String,
    pub password: String,
    pub database: String,
    pub port: u32,
    pub host: String,
    pub uri: String,
}

#[derive(Debug)]
pub struct OtpConfig {
    pub secret: String,
    pub ttl_sec: u32,
    pub size: u32,
}

#[derive(Debug)]
pub struct RedisConfig {
    pub password: String,
}

#[derive(Debug)]
pub struct Config {
    pub db: DbConfig,
    pub port: u32,
    pub otp: OtpConfig,
    pub redis: RedisConfig,
}

#[derive(Debug, Clone)]
pub enum ConfigError {
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for ConfigError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Config {
    pub fn from_env() -> Result<Config, ConfigError> {
        let mut db_user = String::default();
        let mut db_password = String::default();
        let mut db_database = String::default();
        let mut db_port = 5432;
        let mut db_host = String::from("localhost");

        let mut otp_secret = String::default();
        let mut otp_ttl_sec = 30;
        let mut otp_size = 8;

        let mut port = 3000;

        let mut redis_password = String::default();

        for (k, v) in std::env::vars() {
            match k.as_str() {
                "DB_USER" => db_user = v,
                "DB_PASSWORD" => db_password = v,
                "DB_DATABASE" => db_database = v,
                "DB_HOST" => db_host = v,
                "DB_PORT" => db_port = v.parse()?,
                "OTP_SECRET" => otp_secret = v,
                "OTP_TTL_SEC" => otp_ttl_sec = v.parse()?,
                "OTP_SIZE" => otp_size = v.parse()?,
                "PORT" => port = v.parse()?,
                "REDIS_PASSWORD" => redis_password = v,
                _ => (),
            };
        }

        let otp_config = OtpConfig {
            secret: otp_secret,
            ttl_sec: otp_ttl_sec,
            size: otp_size,
        };

        let db_uri = format!(
            "postgres://{}:{}@{}:{}/{}",
            db_user, db_password, db_host, db_port, db_database
        );

        let db_config = DbConfig {
            user: db_user,
            password: db_password,
            database: db_database,
            port: db_port,
            host: db_host,
            uri: db_uri,
        };

        let redis_config = RedisConfig {
            password: redis_password,
        };

        Ok(Config {
            port,
            db: db_config,
            otp: otp_config,
            redis: redis_config,
        })
    }
}
