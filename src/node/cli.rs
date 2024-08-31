use std::net::{IpAddr, Ipv4Addr};
// use super::log::LogLevel;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[arg(long, require_equals = true, default_value_t = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))]
    webui_ip: IpAddr,

    #[arg(long, require_equals = true, default_value_t = 9000)]
    webui_port: u16,

    #[arg(long, require_equals = true, default_value_t = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)))]
    server_ip: IpAddr,

    #[arg(long, require_equals = true, default_value_t = 8080)]
    server_port: u16,

    #[arg(long, require_equals = true,default_value_t = String::from("./parrot-node.toml"))]
    config_file: String,

    #[arg(long, default_value_t = false)]
    verbose: bool,
}

impl Cli {
    pub(crate) fn webui_ip(&self) -> IpAddr {
        self.webui_ip
    }

    pub(crate) fn webui_port(&self) -> u16 {
        self.webui_port
    }

    pub(crate) fn server_ip(&self) -> IpAddr {
        self.server_ip
    }

    pub(crate) fn server_port(&self) -> u16 {
        self.server_port
    }

    pub(crate) fn config_file(&self) -> &str {
        &self.config_file
    }

    pub(crate) fn verbose(&self) -> bool {
        self.verbose
    }
}
