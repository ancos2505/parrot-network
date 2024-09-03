pub(crate) mod attic;
pub(crate) mod cli;
pub(crate) mod client;
pub(crate) mod constants;
pub(crate) mod db;
pub(crate) mod log;
pub(crate) mod server;
pub(crate) mod traits;
pub(crate) mod webui;

use std::{fmt::Display, fs::File, str::FromStr};

use serde::{Deserialize, Deserializer};

use crate::proto::blockchain::wallet::SecretKey;

use self::{
    cli::Cli,
    client::result::ClientError,
    server::result::{ServerError, ServerResult},
};

pub(crate) struct NodeConfig {
    cli: Cli,
    toml: ConfigFromToml,
    secret_key: Option<SecretKey>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ConfigFromToml {
    peers: Vec<PeerConfig>,
}

impl ConfigFromToml {
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
            // TODO
            secret_key: None,
        })
    }

    pub(crate) fn cli(&self) -> &Cli {
        &self.cli
    }

    pub(crate) fn toml(&self) -> &ConfigFromToml {
        &self.toml
    }
    
    pub(crate) fn secret_key(&self) -> Option<&SecretKey> {
        self.secret_key.as_ref()
    }
    
    pub(crate) fn set_secret_key(&mut self, secret_key: SecretKey) {
        self.secret_key = Some(secret_key);
    }
}

// #[derive(Debug, Deserialize)]
// pub(crate) struct ServerConfig {
//     ip: IpAddr,
//     port: u16,
// }

// impl ServerConfig {
//     pub(crate) fn ip(&self) -> IpAddr {
//         self.ip
//     }

//     pub(crate) fn port(&self) -> u16 {
//         self.port
//     }
// }

// impl Display for ServerConfig {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}:{}", self.ip, self.port)
//     }
// }

#[derive(Debug, Deserialize)]
pub(crate) struct PeerConfig {
    #[serde(deserialize_with = "deserialize_ascii_hostname")]
    host: AsciiHostname,
    port: u16,
}

impl Display for PeerConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.host.0, self.port)
    }
}

#[derive(Debug, Deserialize)]
struct AsciiHostname(String);

// TODO: Implement more sophisticated validations rather for IP or Hostname
impl FromStr for AsciiHostname {
    type Err = ClientError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for c in s.chars() {
            if !(c.is_ascii_alphanumeric() || c == '-' || c == '.') {
                return Err(ClientError::ParseAsciiHostname(
                    "Not a valid ASCII payload for AsciiHostname".into(),
                ));
            }
        }
        Ok(Self(s.to_owned()))
    }
}

fn deserialize_ascii_hostname<'de, D>(deserializer: D) -> Result<AsciiHostname, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;

    AsciiHostname::from_str(&buf).map_err(serde::de::Error::custom)
}
