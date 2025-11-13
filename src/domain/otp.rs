use hmac::Hmac;
use hmac::Mac;
use serde::Deserialize;
use sha1::Sha1;
use std::error;
use std::fmt;

use crate::config::CONFIG;

type HmacSha1 = Hmac<Sha1>;

#[derive(Debug)]
pub struct Otp(String);

impl Otp {
    pub fn new(key: &str) -> Result<Self, std::io::Error> {
        let mut hash = HmacSha1::new_from_slice(CONFIG.otp.secret.as_bytes()).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to generate hash: {}", e),
            )
        })?;

        let now = std::time::UNIX_EPOCH
            .elapsed()
            .map_err(|e| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Failed to get unix time: {}", e),
                )
            })?
            .as_secs();

        let ttl: u64 = CONFIG.otp.ttl_sec.into();
        let t = now / ttl;

        hash.update(&t.to_ne_bytes());
        hash.update(key.as_bytes());

        //TODO: Add proper RFC offset
        let bytes = hash.finalize().into_bytes();
        let bytes: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
        let n = u32::from_ne_bytes(bytes);
        let n = n % (10 as u32).pow(CONFIG.otp.size);

        Ok(Self(n.to_string()))
    }

    pub fn parse(otp: &str) -> Result<Self, InvalidOtpError> {
        let otp_size: usize = usize::try_from(CONFIG.otp.size).unwrap_or_default();

        match otp.len() != otp_size {
            false => Err(InvalidOtpError(otp.into())),
            true => match otp.parse() {
                Ok(v) => Ok(Self(v)),
                Err(_) => Err(InvalidOtpError(otp.into())),
            },
        }
    }
}

impl fmt::Display for Otp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for Otp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        Otp::parse(&raw).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct InvalidOtpError(String);

impl error::Error for InvalidOtpError {}

impl fmt::Display for InvalidOtpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Invalid otp: {}", &self.0)
    }
}
