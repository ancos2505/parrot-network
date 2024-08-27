use crate::node::webui::ServerError;

use super::traits::ArgFields;

use std::{fmt::Display, ops::Deref, path::PathBuf, str::FromStr};

#[derive(Debug)]
pub(crate) struct CliConfigFile(PathBuf);
impl Default for CliConfigFile {
    fn default() -> Self {
        let mut path = PathBuf::new();
        path.push("parrot-node.toml");
        Self(path)
    }
}

impl ArgFields for CliConfigFile {
    fn long() -> &'static str {
        "--config-file"
    }

    fn description() -> &'static str {
        "Config file (parrot-node.toml)"
    }
}

impl Display for CliConfigFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl FromStr for CliConfigFile {
    type Err = ServerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(PathBuf::from(s)))
    }
}

impl Deref for CliConfigFile {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
