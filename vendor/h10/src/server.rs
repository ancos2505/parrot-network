mod cli;
mod log;
mod result;
mod traits;

use std::{
    collections::BTreeMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread::{self},
    time::Instant,
};

pub(crate) use self::cli::{CliHttp10StrictMode, CliVerboseMode};

use h10::http::{
    headers::{Connection, IntoHeader},
    response::Response,
    result::{H10LibError, H10LibResult},
    status_code::StatusCode,
};

// use self::log::LogLevel;

pub(crate) use self::{
    cli::Cli,
    result::{ServerError, ServerResult},
    traits::IntoResponse,
};

use crate::{pages::Endpoint, CLI_ARGS, FOUR_KBYTES, HTTP10_STRICT_MODE, MAX_ACTIVE_SESSIONS};

pub(crate) struct ServerResponse(Response);
impl ServerResponse {
    pub fn new(status: StatusCode) -> Self {
        if HTTP10_STRICT_MODE.get().is_some() {
            Self(Response::new(status))
        } else {
            Self(Response::new(status).add_header(Connection::default()))
        }
    }
    pub fn add_header<H: IntoHeader>(self, header: H) -> Self {
        Self(self.0.add_header(header))
    }
    pub fn set_body<B: AsRef<str>>(self, body: B) -> Self {
        Self(self.0.set_body(body))
    }
}

impl IntoResponse for ServerResponse {
    fn into_response(self) -> Response {
        self.0
    }
}

pub(crate) struct HttpServer;
impl HttpServer {
    const CHUNK_SIZE: usize = FOUR_KBYTES;

    fn listener(cli_data: &Cli) -> String {
        format!("{}:{}", cli_data.ip_address, cli_data.port)
    }
    pub fn run() -> ServerResult<()> {
        if let Some(cli) = CLI_ARGS.get() {
            if cli.is_help {
                Cli::usage();
                return Ok(());
            }

            let mut active_sessions = Arc::new(Mutex::new(0));

            let list_str = Self::listener(cli);
            let listener = TcpListener::bind(&list_str)?;
            // let listener = TcpListener::bind(&list_str)?;

            println!("Listening for connections on {}", list_str);
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
            Err(_) => {
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
                    Err(_) => {
                        // dbg!(error);
                    }
                };
                match Self::handle_read(&stream) {
                    Ok(res) => res,
                    Err(_) => {
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

        match Self::handle_write(arc_prev_stats, stream, response_str, arc_act_session) {
            Ok(_) => {
                println!(
                    "Response generated in {} secs. ",
                    now.elapsed().as_secs_f64(),
                );
            }
            Err(err) => {
                println!("Error on sending Reponse. Reason: {err}.",);
            }
        }

        // println!(
        //     "Active sessions: {:?}. Response generated in {} secs. ",
        //     opened_sessions,
        //     now.elapsed().as_secs_f64(),
        // );
    }
    fn handle_read(mut stream: &TcpStream) -> H10LibResult<ServerResponse> {
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
            Err(_) => {
                // dbg!(error);
            }
        };
        let statuscode_str = server_response.0.status().to_string();
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
