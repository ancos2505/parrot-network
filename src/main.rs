mod node;
mod proto;

use std::{
    process::ExitCode,
    sync::{atomic::AtomicUsize, OnceLock},
    thread::{self, sleep},
    time::Duration,
};

use clap::Parser;
use proto::blockchain::block::Block;

use self::node::{
    db::ParrotDb,
    server::NodeServer,
    webui::{Cli, ServerResult, WebuiServer},
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
            ExitCode::from(2)
        }
    }
}

fn smain() -> ServerResult<()> {
    let cli = Cli::parse();
    CLI_ARGS.get_or_init(|| cli);

    thread::spawn(|| -> ServerResult<()> {
        let mut db = ParrotDb::open()?;
        let mut genesis_block = Block::genesis_block()?;
        println!("Db: Genesis Block to before mining: {:?}", &genesis_block);
        genesis_block.mine()?;

        println!("Db: Genesis Block mined to be saved: {:?}", &genesis_block);
        let id = db.save_block(genesis_block)?;

        let loaded_block = db.get_block(id)?;
        println!("Db: Genesis Block loaded.: {:?}", &loaded_block);

        Ok(())
    });
    sleep(Duration::from_millis(50));
    thread::spawn(|| -> ServerResult<()> { WebuiServer::run() });
    sleep(Duration::from_millis(100));
    NodeServer::run()?;
    Ok(())
}
