mod node;
mod proto;

use std::{
    process::ExitCode,
    sync::{atomic::AtomicUsize, OnceLock},
    thread,
};

use self::node::{
    server::NodeServer,
    webui::{Cli, ServerError, ServerResult, WebuiServer},
};

// Unsafe
static ROOT_PAGER_COUNTER: AtomicUsize = AtomicUsize::new(0);

static HTTP10_STRICT_MODE: OnceLock<bool> = OnceLock::new();

static CLI_ARGS: OnceLock<Cli> = OnceLock::new();

pub(crate) const MAX_ACTIVE_SESSIONS: usize = 5_000;

fn main() -> ExitCode {
    match smain() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: '{err}'\n");

            match err {
                ServerError::InvalidCLiArgs(arg) => {
                    eprintln!("Error: unexpected argument '{arg}'\n");
                    Cli::usage();
                    ExitCode::FAILURE
                }
                ServerError::TomlFileError(_)
                | ServerError::H10LibError(_)
                | ServerError::StdIoError(_)
                | ServerError::AddrParseError(_)
                | ServerError::PoisonErrorRwLockReadGuard
                | ServerError::PortParseError
                | ServerError::InvalidLogLevel
                | ServerError::Custom(_) => ExitCode::from(2),
            }
        }
    }
}

fn smain() -> ServerResult<()> {
    let cli = Cli::parse()?;
    CLI_ARGS.get_or_init(|| cli);

    thread::spawn(|| -> ServerResult<()> { WebuiServer::run() });

    NodeServer::run()?;
    Ok(())
}
