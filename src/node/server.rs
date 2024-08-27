mod endpoints;

use std::{
    collections::BTreeMap,
    fmt::Display,
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    os::unix::fs::MetadataExt,
    sync::{Arc, Mutex},
    thread::{self},
    time::Instant,
};

use h10::http::{
    headers::{Connection, IntoHeader},
    response::Response,
    result::{H10LibError, H10LibResult},
    status_code::StatusCode,
};
use serde::Deserialize;

use crate::{CLI_ARGS, HTTP10_STRICT_MODE, MAX_ACTIVE_SESSIONS};

use crate::node::log::LogLevel;

use self::endpoints::Endpoint;

pub(crate) use crate::node::{
    cli::{Cli, CliVerboseMode},
    result::{ServerError, ServerResult},
    traits::IntoResponse,
};

use super::constants::MAX_HTTP_MESSAGE_LENGTH;

pub(crate) struct ServerResponse(Response);
impl ServerResponse {
    pub(crate) fn new(status: StatusCode) -> Self {
        if HTTP10_STRICT_MODE.get().is_some() {
            Self(Response::new(status))
        } else {
            Self(Response::new(status).header(Connection::default()))
        }
    }
    pub(crate) fn header<H: IntoHeader>(self, header: H) -> Self {
        Self(self.0.header(header))
    }
    pub(crate) fn body<B: ToString>(self, body: B) -> Self {
        Self(self.0.body(body))
    }
}

impl IntoResponse for ServerResponse {
    fn into_response(self) -> Response {
        self.0
    }
}

pub(crate) struct NodeServer;
impl NodeServer {
    const CHUNK_SIZE: usize = MAX_HTTP_MESSAGE_LENGTH;

    fn get_config(cli: &Cli) -> ServerResult<Config> {
        let path = &*cli.config_file;
        let mut toml_str = "".to_string();
        let mut file = File::open(path)?;
        let metadata = file.metadata()?;

        if metadata.size() > 1024 * 1024 {
            return Err(ServerError::Custom(
                "Server config file is larger than 1MByte.".into(),
            ));
        }

        file.read_to_string(&mut toml_str)?;
        Ok(toml::from_str(&toml_str)?)
    }
    fn listener(server_config: &Config) -> String {
        format!("{}", server_config.server)
    }
    pub(crate) fn run() -> ServerResult<()> {
        if let Some(cli) = CLI_ARGS.get() {
            if cli.is_help {
                Cli::usage();
                return Ok(());
            }
            let server_config = Self::get_config(cli)?;

            let mut active_sessions = Arc::new(Mutex::new(0));

            let list_str = Self::listener(&server_config);
            let listener = TcpListener::bind(&list_str)?;
            // let listener = TcpListener::bind(&list_str)?;

            println!("Node: Listening for connections on {}", list_str);
            let prev_stats: Arc<Mutex<BTreeMap<String, (u64, u64)>>> =
                Arc::new(Mutex::new(BTreeMap::new()));

            {
                // let stats_mutex = Arc::clone(&prev_stats);
                // let res = stats_mutex.lock();
                // if let Ok(mut data) = res {
                //     Stats::new().render(&mut data)?;
                // }
            }

            let mut incomming = 0;

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        incomming += 1;
                        // dbg!(incomming);
                        let stats_mutex = Arc::clone(&prev_stats);
                        let act_session = Arc::clone(&active_sessions);

                        thread::spawn(move || {
                            Self::handle_client(&stats_mutex, stream, &act_session);
                        });
                        // incomming += 1;
                        // dbg!(incomming);
                        // let stats_mutex = Arc::clone(&prev_stats);
                        // let act_session = Arc::clone(&active_sessions);
                        // thread::spawn(move || {
                        //     if let Ok(mut data) = stats_mutex.lock() {
                        //         let _unused = Self::handle_client(&mut data, stream, act_session);
                        //     }
                        // });
                    }
                    Err(e) => {
                        println!("Unable to connect: {}", e);
                    }
                }
            }
        }

        Ok(())
    }
    fn handle_client(
        arc_prev_stats: &Arc<Mutex<BTreeMap<String, (u64, u64)>>>,
        // prev_stats: &mut BTreeMap<String, (u64, u64)>,
        stream: TcpStream,
        arc_act_session: &Arc<Mutex<usize>>,
    ) {
        let now = Instant::now();

        let act_session = Arc::clone(arc_act_session);

        let opened_sessions = match act_session.try_lock() {
            Ok(data) => Some(*data),
            Err(error) => {
                // dbg!(error);
                None
            }
        };

        let response_str = if let Some(sessions) = opened_sessions {
            if sessions < MAX_ACTIVE_SESSIONS {
                match act_session.try_lock() {
                    Ok(mut data) => {
                        // dbg!(*data);
                        *data += 1;
                        // dbg!(*data);
                    }
                    Err(error) => {
                        // dbg!(error);
                    }
                };
                match Self::handle_read(&stream) {
                    Ok(res) => res,
                    Err(error) => {
                        // dbg!(error);
                        ServerResponse::new(StatusCode::ServiceUnavailable)
                    }
                }
            } else {
                ServerResponse::new(StatusCode::ServiceUnavailable)
            }
        } else {
            ServerResponse::new(StatusCode::ServiceUnavailable)
        };

        if let Err(error) =
            Self::handle_write(arc_prev_stats, stream, response_str, arc_act_session)
        {
            // dbg!(error);
        }

        // println!(
        //     "Active sessions: {:?}. Response generated in {} secs. ",
        //     opened_sessions,
        //     now.elapsed().as_secs_f64(),
        // );
        println!(
            "Response generated in {} secs. ",
            now.elapsed().as_secs_f64(),
        );
    }
    fn handle_read(mut stream: &TcpStream) -> H10LibResult<ServerResponse> {
        // let slow_motion = Duration::from_millis(1234);
        // dbg!(slow_motion);
        // sleep(slow_motion);
        // TODO
        let mut buf = [0u8; Self::CHUNK_SIZE];
        match stream.read(&mut buf) {
            Ok(bytes) => {
                if let Some(cli_data) = CLI_ARGS.get() {
                    if cli_data.verbose == CliVerboseMode::Enabled {
                        println!("Request received: {bytes} Bytes.");
                    }
                }
                Ok(Endpoint::dispatcher(&buf))
            }
            Err(err) => {
                println!("Unable to read stream: {}", err);
                Err(H10LibError::IoError(err))
            }
        }
    }
    fn handle_write(
        // prev_stats: &mut BTreeMap<String, (u64, u64)>,
        arc_prev_stats: &Arc<Mutex<BTreeMap<String, (u64, u64)>>>,
        mut stream: TcpStream,
        server_response: ServerResponse,
        act_session: &Arc<Mutex<usize>>,
    ) -> H10LibResult<()> {
        // let prev_stats = Arc::clone(arc_prev_stats);
        match act_session.try_lock() {
            Ok(mut data) => {
                // dbg!(*data);
                *data -= 1;
                // dbg!(*data);
            }
            Err(error) => {
                // dbg!(error);
            }
        };
        let statuscode_str = server_response.0.status.to_string();
        let response_str = server_response.into_response().to_string();
        match stream.write(response_str.as_bytes()) {
            Ok(bytes) => {
                if let Some(cli_data) = CLI_ARGS.get() {
                    if cli_data.verbose == CliVerboseMode::Enabled {
                        println!("Response sent: {bytes} Bytes.");
                        println!("{response_str}");
                    } else {
                        println!("Response sent: {bytes} Bytes. {statuscode_str}");
                    }
                }
            }
            Err(e) => println!("Failed sending response: {}", e),
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct Config {
    // global_string: Option<String>,
    // global_integer: Option<u64>,
    server: ServerConfig,
    peers: Option<Vec<PeerConfig>>,
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
