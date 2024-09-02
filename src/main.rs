mod node;
mod proto;

use std::{
    process::ExitCode,
    sync::{atomic::AtomicUsize, OnceLock},
    thread::{self, sleep},
    time::Duration,
};

use crate::node::{
    client::{result::ClientResult, NodeClient},
    db::ParrotDb,
    server::result::ServerResult,
    server::NodeServer,
    webui::Cli,
    webui::{result::WebUiResult, WebUiServer},
    NodeConfig,
};

use clap::Parser;
use ed25519_dalek::{SecretKey, SECRET_KEY_LENGTH};
use proto::blockchain::block::Block;

// Unsafe
static ROOT_PAGER_COUNTER: AtomicUsize = AtomicUsize::new(0);

static HTTP10_STRICT_MODE: OnceLock<bool> = OnceLock::new();

static NODE_CONFIG: OnceLock<NodeConfig> = OnceLock::new();

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

    let mut node_config = NodeConfig::load(cli)?;

    // TODO
    {
        // if let Some(secret_key_file) = node_config.toml().server().secret_key_file() {}

        let secret_key: SecretKey = {
            use rand::{rngs::OsRng, RngCore};
            let mut inner_buf = [0u8; SECRET_KEY_LENGTH];
            OsRng.fill_bytes(&mut inner_buf);
            inner_buf
        }
        .into();

        node_config.set_secret_key(secret_key);
    }

    NODE_CONFIG.get_or_init(|| node_config);

    let th_genesis_block = thread::spawn(|| -> ServerResult<()> {
        // TODO: WIP.
        // TODO: In the future must be pre-generated.
        let mut genesis_block = Block::genesis_block()?;
        println!("Db: Genesis Block to before mining: {:?}", &genesis_block);
        println!("\n\n\n\n");
        genesis_block.mine()?;

        genesis_block.verify()?;

        let mut db = ParrotDb::open()?;
        println!("Db: Genesis Block mined to be saved: {:?}", &genesis_block);
        let id = db.save_block(&genesis_block)?;

        let retrieved_block = db.get_block(id)?;
        println!("Db: Genesis Block retrieved.: {:?}", &retrieved_block);

        assert_eq!(&genesis_block, &retrieved_block);
        Ok(())
    });

    let th_node_webui = thread::spawn(|| -> WebUiResult<()> {
        sleep(Duration::from_millis(50));
        WebUiServer::run()
    });

    let th_node_client = thread::spawn(|| -> ClientResult<()> {
        sleep(Duration::from_millis(2000));
        NodeClient::run()
    });

    NodeServer::run()?;

    Ok(())
}
