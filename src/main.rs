mod node;
mod proto;

use std::{
    process::ExitCode,
    sync::{atomic::AtomicUsize, OnceLock},
    thread::{self, sleep},
    time::{Duration, Instant},
};

use clap::Parser;
use node::server::result::ServerError;
use proto::blockchain::wallet::SecretKey;

use crate::{
    node::{
        client::{result::ClientResult, NodeClient},
        db::ParrotDb,
        server::result::ServerResult,
        server::NodeServer,
        webui::Cli,
        webui::{result::WebUiResult, WebUiServer},
        NodeConfig,
    },
    proto::blockchain::block::Block,
};

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
    println!("Main: Generating secret key for node...");
    let start = Instant::now();
    node_config.set_secret_key(SecretKey::random());
    println!("Main: Generated in {} secs.", start.elapsed().as_secs_f32());

    NODE_CONFIG.get_or_init(|| node_config);

    let th_genesis_block = thread::spawn(|| -> ServerResult<()> {
        // TODO: WIP.
        // TODO: In the future must be pre-generated.
        // let mut genesis_block = Block::genesis_block()?;
        // println!("Db: Genesis Block to before mining: {:?}", &genesis_block);
        // println!("\n\n\n\n");
        // genesis_block.mine()?;

        // genesis_block.verify()?;

        // let mut db = ParrotDb::open()?;
        // println!("Db: Genesis Block mined to be saved: {:?}", &genesis_block);
        // let id = db.save_block(&genesis_block)?;

        // let retrieved_block = db.get_block(id)?;
        // println!("Db: Genesis Block retrieved.: {:?}", &retrieved_block);

        // assert_eq!(&genesis_block, &retrieved_block);
        Ok(())
    });

    let th_node_webui = thread::spawn(|| -> WebUiResult<()> {
        sleep(Duration::from_millis(50));
        WebUiServer::run()
    });

    let stack_size = 20 * 1024 * 1024;

    let builder = thread::Builder::new().stack_size(stack_size);

    let th_node_client = builder.spawn(|| -> ClientResult<()> {
        sleep(Duration::from_millis(5_000));
        NodeClient::run()
    })?;

    let builder = thread::Builder::new().stack_size(stack_size);

    let th_node_server = builder.spawn(|| -> ServerResult<()> { NodeServer::run() })?;
    th_node_server
        .join()
        .map_err(|_| ServerError::custom("Error on thread join in th_node_server"))?
}
