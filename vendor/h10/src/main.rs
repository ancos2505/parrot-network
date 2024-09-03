mod pages;
mod server;

use std::{
    process::ExitCode,
    sync::{atomic::AtomicUsize, OnceLock},
};

use server::ServerError;

use crate::server::{Cli, HttpServer, ServerResult};

// Unsafe
static ROOT_PAGER_COUNTER: AtomicUsize = AtomicUsize::new(0);

static HTTP10_STRICT_MODE: OnceLock<bool> = OnceLock::new();

static CLI_ARGS: OnceLock<Cli> = OnceLock::new();

pub(crate) const FOUR_KBYTES: usize = 1024 * 4;

pub(crate) const MAX_ACTIVE_SESSIONS: usize = 5_000;

fn main() -> ExitCode {
    match smain() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => match err {
            ServerError::InvalidCLiArgs(arg) => {
                eprintln!("Cli Parsing: unexpected argument '{arg}'");
                Cli::usage();
                ExitCode::FAILURE
            }
            // ServerError::H10LibError(err) => {
            //     eprintln!("H10LibError:'{}'\n", err.to_string());
            //     ExitCode::from(2)
            // }
            ServerError::StdIoError(err)
            | ServerError::AddrParseError(err)
            | ServerError::PoisonErrorRwLockReadGuard(err)
            | ServerError::PortParseError(err)
            | ServerError::InvalidLogLevel(err)
            | ServerError::Custom(err) => {
                eprintln!("Server Error: '{err}'");
                ExitCode::from(3)
            }
        },
    }
}

fn smain() -> ServerResult<()> {
    let cli = Cli::parse()?;
    CLI_ARGS.get_or_init(|| cli);

    HttpServer::run()?;
    Ok(())
}
