use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
    path::PathBuf,
    sync::LazyLock,
};

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| Config::from_env().unwrap());

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
pub struct Config {
    pub db: DbConfig,
    pub port: u32,
    pub otp: OtpConfig,
}

fn parse_port(port: &str) -> Result<u32, std::io::Error> {
    Ok(port.parse().map_err(|e: ParseIntError| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Can't parse port: {}", e.to_string()),
        )
    })?)
}

fn parse_otp_ttl_sec(ttl: &str) -> Result<u32, std::io::Error> {
    Ok(ttl.parse().map_err(|e: ParseIntError| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Can't parse otp ttl: {}", e.to_string()),
        )
    })?)
}

fn parse_otp_size(size: &str) -> Result<u32, std::io::Error> {
    Ok(size.parse().map_err(|e: ParseIntError| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Can't parse otp size: {}", e.to_string()),
        )
    })?)
}

impl Config {
    pub fn from_env() -> Result<Config, std::io::Error> {
        let mut db_user = String::default();
        let mut db_password = String::default();
        let mut db_database = String::default();
        let mut db_port = 5432;
        let mut db_host = String::from("localhost");

        let mut otp_secret = String::default();
        let mut otp_ttl_sec = 30;
        let mut otp_size = 8;

        let mut port = 3000;

        let current_dir = env::current_dir()?;
        let filename = ".env";

        let mut path = PathBuf::from(current_dir);
        path.push(filename);

        let f = File::open(path)?;
        let f = BufReader::new(f);

        for line in f.lines() {
            let l = String::from(line?);
            let s: Vec<&str> = l.split("=").collect();

            if s.len() < 2 {
                continue;
            }

            let (k, v) = (s[0], s[1]);

            match k {
                "DB_USER" => db_user = v.into(),
                "DB_PASSWORD" => db_password = v.into(),
                "DB_DATABASE" => db_database = v.into(),
                "DB_HOST" => db_host = v.into(),
                "DB_PORT" => db_port = parse_port(v)?,
                "OTP_SECRET" => otp_secret = v.into(),
                "OTP_TTL_SEC" => otp_ttl_sec = parse_otp_ttl_sec(v)?,
                "OTP_SIZE" => otp_size = parse_otp_size(v)?,
                "PORT" => port = parse_port(v)?,
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

        Ok(Config {
            port: port,
            db: db_config,
            otp: otp_config,
        })
    }
}
