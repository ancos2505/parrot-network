use crate::node::webui::ServerError;

use super::traits::ArgName;
use std::{fmt::Display, ops::Deref, str::FromStr};

#[derive(Debug)]
pub(crate) struct CliWebUiPortNumber(u16);
impl Default for CliWebUiPortNumber {
    fn default() -> Self {
        Self(8080)
    }
}

impl ArgName for CliWebUiPortNumber {
    fn arg_name() -> String {
        "--webui-port".into()
    }
}

impl Display for CliWebUiPortNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for CliWebUiPortNumber {
    type Err = ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ip_addr = s.parse().map_err(|_| ServerError::PortParseError)?;
        if ip_addr > 1024 {
            Ok(Self(ip_addr))
        } else {
            Err(ServerError::PortParseError)
        }
    }
}

impl Deref for CliWebUiPortNumber {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
