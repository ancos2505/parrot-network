pub(crate) mod result;

use std::{thread, time::Duration};

use h10::{
    client::HttpClient,
    http::{request::Request, response::parser::ResponseParser, url_path::UrlPath},
};

use crate::NODE_CONFIG;

use self::result::ClientResult;

use super::PeerConfig;

pub(crate) struct NodeClient;

impl NodeClient {
    pub(crate) fn run() -> ClientResult<()> {
        if let Some(node_config) = NODE_CONFIG.get() {
            for peer in node_config.toml().peers() {
                let th_result =
                    thread::spawn(|| -> ClientResult<()> { Self::request(peer) }).join();
                match th_result {
                    Ok(inner_res) => {
                        if let Err(client_error) = inner_res {
                            println!("NodeClient: Client error: {client_error:?}");
                        }
                    }
                    Err(err) => {
                        println!("NodeClient: Internal error: {err:?}");
                    }
                }
            }
        }

        Ok(())
    }

    // TODO: Implement Hostname/IP validation
    fn request(peer: &PeerConfig) -> ClientResult<()> {
        use std::time::Instant;

        let start = Instant::now();

        let request = Request::get().path(UrlPath::root()).finish();

        println!("NodeClient: Sending Request:\n{request}");
        let timeout = Duration::from_secs(5);

        let response_str = HttpClient::launch(request, peer.to_string(), timeout)?;
        
        let response = ResponseParser::parse(response_str.as_bytes())?;
        
        let elapsed = start.elapsed().as_secs_f32();
        println!("NodeClient: Response received:\n{response}");
        println!("NodeClient: StatusCode: {}", response.status());
        println!("NodeClient: Response received in {} secs", elapsed);

        Ok(())
    }
}
