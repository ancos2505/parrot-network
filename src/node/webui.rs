mod pages;
pub(crate) mod result;

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    time::Instant,
};

use h10::http::{
    headers::{Connection, IntoHeader},
    response::Response,
    result::{H10LibError, H10LibResult},
    status_code::StatusCode,
};
use result::WebUiResult;

use crate::{HTTP10_STRICT_MODE, NODE_CONFIG};

use crate::node::log::LogLevel;

use self::pages::Endpoint;

pub(crate) use crate::node::{cli::Cli, traits::IntoResponse};

use super::{constants::MAX_HTTP_MESSAGE_LENGTH, NodeConfig};

#[derive(Debug,PartialEq, Eq)]
enum ClientHandlingOutcome {
    Success(String),
    Failure(String),
}

pub(crate) struct WebUiResponse(Response);
impl WebUiResponse {
    pub(crate) fn new(status: StatusCode) -> Self {
        if HTTP10_STRICT_MODE.get().is_some() {
            Self(Response::new(status))
        } else {
            Self(Response::new(status).add_header(Connection::default()))
        }
    }
    pub(crate) fn add_header<H: IntoHeader>(self, header: H) -> Self {
        Self(self.0.add_header(header))
    }
    pub(crate) fn body<B: AsRef<str>>(self, body: B) -> Self {
        Self(self.0.set_body(body))
    }
}

impl IntoResponse for WebUiResponse {
    fn into_response(self) -> Response {
        self.0
    }
}

pub(crate) struct WebUiServer;
impl WebUiServer {
    const CHUNK_SIZE: usize = MAX_HTTP_MESSAGE_LENGTH;

    fn listener(node_config: &NodeConfig) -> String {
        let cli = node_config.cli();
        format!("{}:{}", cli.webui_ip(), cli.webui_port())
    }
    pub(crate) fn run() -> WebUiResult<()> {
        if let Some(node_config) = NODE_CONFIG.get() {
            let list_str = Self::listener(node_config);
            let listener = TcpListener::bind(&list_str)?;

            println!("WebuiServer: Listening for connections on {}", list_str);

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        Self::handle_client(stream);
                    }
                    Err(err) => {
                        println!("WebuiServer: Error: Unable to connect: {}", err);
                    }
                }
            }
        }

        Ok(())
    }
    fn handle_client(stream: TcpStream) -> ClientHandlingOutcome {
        let now = Instant::now();

        let response_to_send = match Self::handle_read(&stream) {
            Ok(res) => res,
            Err(error) => {
                dbg!(error);
                WebUiResponse::new(StatusCode::ServiceUnavailable)
            }
        };

        if let Err(error) = Self::handle_write(stream, response_to_send) {
            // dbg!(error);
            ClientHandlingOutcome::Failure(error.to_string())
        } else {
            let elapsed = now.elapsed().as_secs_f64();
            let msg = format!("Response sent after {elapsed} secs.");
            println!("WebuiServer: {msg}");
            ClientHandlingOutcome::Success(msg)
        }
    }
    fn handle_read(mut stream: &TcpStream) -> H10LibResult<WebUiResponse> {
        // TODO
        let mut buf = [0u8; Self::CHUNK_SIZE];
        match stream.read(&mut buf) {
            Ok(bytes) => {
                if let Some(node_config) = NODE_CONFIG.get() {
                    if node_config.cli().verbose() {
                        println!("WebuiServer: Request received: {bytes} Bytes.");
                    }
                }
                Ok(Endpoint::dispatcher(&buf))
            }
            Err(err) => {
                println!("WebuiServer: Error: Unable to read stream: {}", err);
                Err(H10LibError::IoError(err))
            }
        }
    }
    fn handle_write(mut stream: TcpStream, server_response: WebUiResponse) -> H10LibResult<()> {
        let statuscode_str = server_response.0.status().to_string();
        let response_str = server_response.into_response().to_string();
        match stream.write(response_str.as_bytes()) {
            Ok(bytes) => {
                if let Some(node_config) = NODE_CONFIG.get() {
                    if node_config.cli().verbose() {
                        println!("WebuiServer: Response sent: {bytes} Bytes.");
                        println!("{response_str}");
                    } else {
                        println!("WebuiServer: Response sent: {bytes} Bytes. {statuscode_str}");
                    }
                }
            }
            Err(err) => println!("WebuiServer: Error: Failed sending response: {}", err),
        }
        Ok(())
    }
}
