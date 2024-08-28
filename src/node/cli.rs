use std::net::{IpAddr, Ipv4Addr};
// use super::log::LogLevel;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[arg(long, require_equals = true, default_value_t = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))]
    pub(crate) webui_ip: IpAddr,

    #[arg(long, require_equals = true, default_value_t = 9000)]
    pub(crate) webui_port: u16,

    #[arg(long, require_equals = true, default_value_t = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)))]
    pub(crate) server_ip: IpAddr,

    #[arg(long, require_equals = true, default_value_t = 8080)]
    pub(crate) server_port: u16,

    #[arg(long, require_equals = true,default_value_t = String::from("./parrot-node.toml"))]
    pub(crate) config_file: String,

    #[arg(long, default_value_t = false)]
    pub(crate) verbose: bool,
}
