use std::{fmt::Display, fs::File};

use serde::Deserialize;
use server::result::{ServerError, ServerResult};

use self::cli::Cli;

pub(crate) mod attic;
pub(crate) mod cli;
pub(crate) mod client;
pub(crate) mod constants;
pub(crate) mod db;
pub(crate) mod log;
pub(crate) mod server;
pub(crate) mod traits;
pub(crate) mod webui;

pub(crate) struct NodeConfig {
    cli: Cli,
    toml: ConfigFromToml,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ConfigFromToml {
    // global_string: Option<String>,
    // global_integer: Option<u64>,
    server: ServerConfig,
    peers: Vec<PeerConfig>,
}

impl ConfigFromToml {
    pub(crate) fn server(&self) -> &ServerConfig {
        &self.server
    }

    pub(crate) fn peers(&self) -> &[PeerConfig] {
        &self.peers
    }
}

impl NodeConfig {
    pub(crate) fn load(cli: Cli) -> ServerResult<Self> {
        use std::{io::Read, os::unix::fs::MetadataExt};

        let mut toml_str = "".to_string();
        let config_file = cli.config_file();
        let mut file = File::open(&config_file)
            .map_err(|_| ServerError::Custom(format!("Config file not found `{config_file}`.")))?;
        let metadata = file.metadata()?;

        if metadata.size() > 1024 * 1024 {
            return Err(ServerError::Custom(
                "Server config file is larger than 1MByte.".into(),
            ));
        }

        file.read_to_string(&mut toml_str)?;

        Ok(Self {
            cli: cli,
            toml: toml::from_str(&toml_str)?,
        })
    }

    pub(crate) fn cli(&self) -> &Cli {
        &self.cli
    }

    pub(crate) fn toml(&self) -> &ConfigFromToml {
        &self.toml
    }
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    ip: String,
    port: u16,
}

impl Display for ServerConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}

#[derive(Debug, Deserialize)]
struct PeerConfig {
    ip: String,
    port: u16,
}

impl Display for PeerConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}
