mod config_file;
mod help;
mod traits;
mod verbose;
mod webui_ip;
mod webui_port;

use std::{collections::BTreeMap, env::Args};

use config_file::CliConfigFile;
use traits::ArgFields;

use crate::node::result::{ServerError, ServerResult};
// use super::log::LogLevel;

pub(crate) use self::{
    help::CliHelp,
    verbose::CliVerboseMode,
    //  http10_strict_mode::CliHttp10StrictMode,
    webui_ip::CliWebUiIpAddress,
    webui_port::CliWebUiPortNumber,
};

const MAX_CLI_NUM_FIELDS: usize = 4;

#[derive(Debug, Default)]
pub(crate) struct Cli {
    // TODO
    // pub(crate) log_level: LogLevel,
    pub(crate) is_help: bool,
    pub(crate) ip_address: CliWebUiIpAddress,
    pub(crate) port: CliWebUiPortNumber,
    pub(crate) config_file: CliConfigFile,
    // pub(crate) h10_strict_mode: CliHttp10StrictMode,
    pub(crate) verbose: CliVerboseMode,
}

impl Cli {
    pub(crate) fn parse() -> ServerResult<Self> {
        Self::try_from(std::env::args())
    }
    pub(crate) fn usage() {
        eprintln!(
            "{name} (v{version}): {desc} - {repo}",
            name = env!("CARGO_PKG_NAME"),
            version = env!("CARGO_PKG_VERSION"),
            desc = env!("CARGO_PKG_DESCRIPTION"),
            repo = env!("CARGO_PKG_REPOSITORY")
        );
        eprintln!("");
        eprintln!("Usage: parrot-node [OPTION]");
        eprintln!("");
        eprintln!("Options:");
        eprintln!(r#"  {:<28}{}"#, CliHelp::long(), CliHelp::description());
        eprintln!("");
        eprintln!(
            r#"  {:<28}{}"#,
            CliVerboseMode::long(),
            CliVerboseMode::description()
        );
        eprintln!("");
        eprintln!(
            r#"  {:<28}{}"#,
            format!("{}=<IP ADDRESS>", CliWebUiIpAddress::long()),
            CliWebUiIpAddress::description()
        );
        eprintln!("");
        eprintln!(
            r#"  {:<28}{}"#,
            format!("{}=<PORT NUMBER>", CliWebUiPortNumber::long()),
            CliWebUiPortNumber::description()
        );
        eprintln!("");
        eprintln!(
            r#"  {:<28}{}"#,
            format!("{}=<PATH>", CliConfigFile::long()),
            CliConfigFile::description()
        );
        eprintln!("");
    }
}
// --webui-port=<PORT NUMBER>
impl TryFrom<CliArgs> for Cli {
    type Error = ServerError;
    fn try_from(mut value: CliArgs) -> Result<Self, Self::Error> {
        let is_help = value.0.contains_key(CliHelp::long());

        let ip_address = {
            if let Some(arg) = value.0.remove(CliWebUiIpAddress::long()) {
                arg.parse::<CliWebUiIpAddress>()?
            } else {
                CliWebUiIpAddress::default()
            }
        };

        let port = {
            if let Some(arg) = value.0.remove(CliWebUiPortNumber::long()) {
                arg.parse::<CliWebUiPortNumber>()?
            } else {
                CliWebUiPortNumber::default()
            }
        };

        // let h10_strict_mode = value
        //     .0
        //     .remove(&CliHttp10StrictMode::long())
        //     .map(|_| CliHttp10StrictMode::Enabled)
        //     .unwrap_or_default();
        let verbose = value
            .0
            .remove(CliVerboseMode::long())
            .map(|_| CliVerboseMode::Enabled)
            .unwrap_or_default();

        let config_file = {
            if let Some(arg) = value.0.remove(CliConfigFile::long()) {
                arg.parse::<CliConfigFile>()?
            } else {
                CliConfigFile::default()
            }
        };

        Ok(Self {
            is_help,
            ip_address,
            port,
            // h10_strict_mode,
            verbose,
            config_file,
        })
    }
}

impl TryFrom<Args> for Cli {
    type Error = ServerError;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        let args_len = args.len();

        if !(args_len > 0 && args_len < (MAX_CLI_NUM_FIELDS + 2)) {
            return Err(ServerError::InvalidCLiArgs(format!(
                "Invalid args length: {args_len}"
            )));
        }

        let mut cli_args = CliArgs::new();

        for arg_to_parse in args.skip(1) {
            let mut arg = arg_to_parse.split('=');
            let maybe_key = arg.next();
            let value = arg.collect::<Vec<_>>().join("=");
            if let Some(key) = maybe_key {
                if key == CliHelp::long() {
                    cli_args.add((CliHelp::long().into(), value));
                } else if key == CliWebUiIpAddress::long() {
                    cli_args.add((CliWebUiIpAddress::long().into(), value));
                } else if key == CliWebUiPortNumber::long() {
                    cli_args.add((CliWebUiPortNumber::long().into(), value));
                } else if key == CliConfigFile::long() {
                    cli_args.add((CliConfigFile::long().into(), value));
                // } else if key == CliHttp10StrictMode::long() {
                //     cli_args.add((CliHttp10StrictMode::long(), value));
                } else if key == CliVerboseMode::long() {
                    cli_args.add((CliVerboseMode::long().into(), value));
                } else {
                    return Err(ServerError::InvalidCLiArgs(key.into()));
                }
            } else {
                println!(
                    "{}: Error: unknown option: {}",
                    env!("CARGO_PKG_NAME"),
                    arg_to_parse
                );
                println!("Use --help for a list of options.");
                return Err(ServerError::InvalidCLiArgs(arg_to_parse));
            }
        }
        cli_args.try_into()
    }
}

// CliArg input sample:
//
//
//
//  --ip-address=127.0.0.1
//  ^^          ^
//  ||          |
//  ++----------+----- Required characters

#[derive(Debug)]
pub(crate) struct CliArgs(BTreeMap<String, String>);

impl CliArgs {
    pub(crate) fn new() -> Self {
        Self(BTreeMap::new())
    }
    pub(crate) fn add(&mut self, (k, v): (String, String)) {
        self.0.insert(k, v);
    }
}
