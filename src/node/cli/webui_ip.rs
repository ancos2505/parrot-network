use crate::node::webui::ServerError;

use super::traits::ArgFields;

use std::{
    fmt::Display,
    net::{IpAddr, Ipv4Addr},
    ops::Deref,
    str::FromStr,
};

#[derive(Debug)]
pub(crate) struct CliWebUiIpAddress(IpAddr);
impl Default for CliWebUiIpAddress {
    fn default() -> Self {
        Self(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
    }
}

impl ArgFields for CliWebUiIpAddress {
    fn long() -> &'static str {
        "--webui-ip"
    }

    fn description() -> &'static str {
        "IPv4 or IPv6 to listening"
    }
}

impl Display for CliWebUiIpAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for CliWebUiIpAddress {
    type Err = ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ip_addr = s.parse()?;
        Ok(Self(ip_addr))
    }
}

impl Deref for CliWebUiIpAddress {
    type Target = IpAddr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
