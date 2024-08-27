mod help;
// mod http10_strict_mode;
mod ip_address;
mod port_number;
mod traits;
mod verbose;

use std::{collections::BTreeMap, env::Args};

use crate::node::{
    cli::traits::ArgName,
    result::{ServerError, ServerResult},
};
// use super::log::LogLevel;

pub(crate) use self::{
    help::CliHelp,
    //  http10_strict_mode::CliHttp10StrictMode,
    ip_address::CliWebUiIpAddress,
    port_number::CliWebUiPortNumber,
    verbose::CliVerboseMode,
};

const MAX_CLI_NUM_FIELDS: usize = 4;

#[derive(Debug, Default)]
pub(crate) struct Cli {
    // TODO
    // pub(crate) log_level: LogLevel,
    pub(crate) is_help: bool,
    pub(crate) ip_address: CliWebUiIpAddress,
    pub(crate) port: CliWebUiPortNumber,
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
        eprintln!(
            "Usage: {pkg_name}-node [OPTION]",
            pkg_name = env!("CARGO_PKG_NAME")
        );
        eprintln!("");
        eprintln!("Options:");
        eprintln!(
            r#"  {}                      Display this message"#,
            CliHelp::arg_name()
        );
        eprintln!("");
        eprintln!(
            r#"  {}                   Show raw contents from both Request and Response"#,
            CliVerboseMode::arg_name()
        );
        eprintln!("");
        eprintln!(
            r#"  {}=<IP ADDRESS>     IPv4 or IPv6 to listening"#,
            CliWebUiIpAddress::arg_name()
        );
        eprintln!("");
        eprintln!(
            r#"  {}=<PORT NUMBER>  Port to listen [1024-65535] (RFC7605#section-4)"#,
            CliWebUiPortNumber::arg_name()
        );
        eprintln!("");
    }
}

impl TryFrom<CliArgs> for Cli {
    type Error = ServerError;
    fn try_from(mut value: CliArgs) -> Result<Self, Self::Error> {
        let is_help = value.0.contains_key(&CliHelp::arg_name());

        let ip_address = {
            if let Some(arg) = value.0.remove(&CliWebUiIpAddress::arg_name()) {
                arg.parse::<CliWebUiIpAddress>()?
            } else {
                CliWebUiIpAddress::default()
            }
        };

        let port = {
            if let Some(arg) = value.0.remove(&CliWebUiPortNumber::arg_name()) {
                arg.parse::<CliWebUiPortNumber>()?
            } else {
                CliWebUiPortNumber::default()
            }
        };

        // let h10_strict_mode = value
        //     .0
        //     .remove(&CliHttp10StrictMode::arg_name())
        //     .map(|_| CliHttp10StrictMode::Enabled)
        //     .unwrap_or_default();
        let verbose = value
            .0
            .remove(&CliVerboseMode::arg_name())
            .map(|_| CliVerboseMode::Enabled)
            .unwrap_or_default();
        Ok(Self {
            is_help,
            ip_address,
            port,
            // h10_strict_mode,
            verbose,
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
                if key == CliHelp::arg_name() {
                    cli_args.add((CliHelp::arg_name(), value));
                } else if key == CliWebUiIpAddress::arg_name() {
                    cli_args.add((CliWebUiIpAddress::arg_name(), value));
                } else if key == CliWebUiPortNumber::arg_name() {
                    cli_args.add((CliWebUiPortNumber::arg_name(), value));
                // } else if key == CliHttp10StrictMode::arg_name() {
                //     cli_args.add((CliHttp10StrictMode::arg_name(), value));
                } else if key == CliVerboseMode::arg_name() {
                    cli_args.add((CliVerboseMode::arg_name(), value));
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
