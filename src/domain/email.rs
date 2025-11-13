use regex::Regex;
use serde::Deserialize;
use std::error;
use std::fmt;

#[derive(Debug)]
pub struct Email(String);

impl Email {
    const REGEX: &str = r"\A[a-z0-9!#$%&'*+\/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+\/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\z";

    pub fn parse(email: &str) -> Result<Self, InvalidEmailError> {
        let re = Regex::new(Email::REGEX)
            .map_err(|e| InvalidEmailError(format!("Failed to create email regex: {}", e)))?;

        match re.is_match(email) {
            false => Err(InvalidEmailError(email.into())),
            true => Ok(Self(email.into())),
        }
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        Email::parse(&raw).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct InvalidEmailError(String);

impl error::Error for InvalidEmailError {}

impl fmt::Display for InvalidEmailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Invalid email: {}", &self.0)
    }
}
