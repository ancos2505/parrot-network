use crate::server::ServerError;

use super::traits::ArgName;
use std::{
    fmt::Display,
    net::{IpAddr, Ipv4Addr},
    ops::Deref,
    str::FromStr,
};

#[derive(Debug)]
pub(crate) struct CliIpAddress(IpAddr);
impl Default for CliIpAddress {
    fn default() -> Self {
        Self(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)))
    }
}

impl ArgName for CliIpAddress {
    fn arg_name() -> String {
        "--ip-address".into()
    }
}

impl Display for CliIpAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for CliIpAddress {
    type Err = ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ip_addr = s.parse()?;
        Ok(Self(ip_addr))
    }
}

impl Deref for CliIpAddress {
    type Target = IpAddr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
