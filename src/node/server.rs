mod api;
mod endpoints;
mod health_check;
pub mod result;

use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    time::Instant,
};

use h10::http::{
    headers::IntoHeader,
    response::Response,
    result::{H10LibError, H10LibResult},
    status_code::StatusCode,
};

use crate::node::log::LogLevel;

use crate::{node::traits::IntoResponse, NODE_CONFIG};

use super::{constants::MAX_HTTP_MESSAGE_LENGTH, NodeConfig};

use self::{endpoints::Endpoint, result::ServerResult};

pub(crate) struct ServerResponse(Response);
impl ServerResponse {
    pub(crate) fn new(status: StatusCode) -> Self {
        Self(Response::new(status))
    }
    pub(crate) fn add_header<H: IntoHeader>(self, header: H) -> Self {
        Self(self.0.add_header(header))
    }
    pub(crate) fn set_body<B: AsRef<str>>(self, body: B) -> Self {
        Self(self.0.set_body(body))
    }
}

impl IntoResponse for ServerResponse {
    fn into_response(self) -> Response {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ClientHandlingOutcome {
    Success(String),
    Failure(String),
}

pub(crate) struct NodeServer;
impl NodeServer {
    const CHUNK_SIZE: usize = MAX_HTTP_MESSAGE_LENGTH;

    fn build_listener(node_config: &NodeConfig) -> SocketAddr {
        let ip = node_config.cli().server_ip();
        let port = node_config.cli().server_port();
        SocketAddr::from((ip, port))
    }

    pub(crate) fn run() -> ServerResult<()> {
        if let Some(node_config) = NODE_CONFIG.get() {
            let socket_addr = Self::build_listener(node_config);
            let listener = TcpListener::bind(&socket_addr)?;

            println!("NodeServer: Listening for connections on {}", socket_addr);

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let outcome = Self::handle_client(stream);
                        println!("NodeServer: Outcome: {:?}", outcome);
                    }
                    Err(err) => {
                        println!("NodeServer: Error: Unable to connect: {}", err);
                    }
                }
            }
        }

        Ok(())
    }
    fn handle_client(stream: TcpStream) -> ServerResult<ClientHandlingOutcome> {
        let now = Instant::now();

        let resonse_to_send = match Self::handle_read(&stream) {
            Ok(res) => res,
            Err(err) => {
                println!(
                    "NodeServer: Internal Error(Possible Service Unavailable?). Reason: {err}."
                );
                ServerResponse::new(StatusCode::ServiceUnavailable)
            }
        };

        if let Err(err) = Self::handle_write(stream, resonse_to_send) {
            println!(
                "NodeServer: Internal Error(Unable to confirm if Response sent). Reason: {err}."
            );

            Ok(ClientHandlingOutcome::Failure(err.to_string()))
        } else {
            let elapsed = now.elapsed().as_secs_f64();
            let msg = format!("Response sent after {elapsed} secs.");
            println!("NodeServer: {msg}");
            Ok(ClientHandlingOutcome::Success(msg))
        }
    }
    fn handle_read(mut stream: &TcpStream) -> H10LibResult<ServerResponse> {
        println!(
            "NodeServer: Received connection to socket {} from {}",
            stream.local_addr()?,
            stream.peer_addr()?
        );

        let mut buf = [0u8; Self::CHUNK_SIZE];
        match stream.read(&mut buf) {
            Ok(bytes) => {
                if let Some(node_config) = NODE_CONFIG.get() {
                    let cli = node_config.cli();
                    if cli.verbose() {
                        println!("NodeServer: Request received: {bytes} Bytes.");
                    }
                }
                // TODO Blocklist peer_addr();
                Ok(Endpoint::dispatcher(&buf))
            }
            Err(err) => {
                println!("NodeServer: Error: Unable to read stream: {}", err);
                Err(H10LibError::IoError(err))
            }
        }
    }
    fn handle_write(mut stream: TcpStream, server_response: ServerResponse) -> H10LibResult<()> {
        let statuscode_str = server_response.0.status().to_string();
        let response_str = server_response.into_response().to_string();
        match stream.write(response_str.as_bytes()) {
            Ok(bytes) => {
                if let Some(node_config) = NODE_CONFIG.get() {
                    let cli = node_config.cli();
                    if cli.verbose() {
                        println!("NodeServer: Response sent: {bytes} Bytes.");
                        println!("{response_str}");
                    } else {
                        println!("NodeServer: Response sent: {bytes} Bytes. {statuscode_str}");
                    }
                }
            }
            Err(err) => println!("NodeServer: Error: Failed sending response: {}", err),
        }
        Ok(())
    }
}
