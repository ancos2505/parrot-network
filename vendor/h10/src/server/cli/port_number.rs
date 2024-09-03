use crate::server::ServerError;

use super::traits::ArgName;
use std::{fmt::Display, ops::Deref, str::FromStr};

#[derive(Debug)]
pub(crate) struct CliPortNumber(u16);
impl Default for CliPortNumber {
    fn default() -> Self {
        Self(8080)
    }
}

impl ArgName for CliPortNumber {
    fn arg_name() -> String {
        "--port".into()
    }
}

impl Display for CliPortNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for CliPortNumber {
    type Err = ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ip_addr = s
            .parse()
            .map_err(|_| ServerError::PortParseError("Invalid payload for Port number".into()))?;
        if ip_addr > 1024 {
            Ok(Self(ip_addr))
        } else {
            Err(ServerError::PortParseError(
                "Port must be higher than 1023".into(),
            ))
        }
    }
}

impl Deref for CliPortNumber {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
