use std::str::FromStr;

use super::ServerError;

#[derive(Debug)]
pub(crate) enum LogLevel {
    ERROR,
    WARN,
    INFO,
    DEBUG,
    TRACE,
}
impl FromStr for LogLevel {
    type Err = ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let level = match s {
            "ERROR" => Self::ERROR,
            "WARN" => Self::WARN,
            "INFO" => Self::INFO,
            "DEBUG" => Self::DEBUG,
            "TRACE, " => Self::TRACE,
            _ => return Err(ServerError::InvalidLogLevel("Invalid log level".into())),
        };
        Ok(level)
    }
}
