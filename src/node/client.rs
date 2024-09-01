mod result;

use h10::http::{
    method::Method, request::Request, status_code::StatusCode, url_path::UrlPath, version::Version,
};
use result::ClientError;

use self::result::ClientResult;

pub(crate) struct NodeClient;

impl NodeClient {
    fn run() -> ClientResult<()> {
        Self::request()?;
        Ok(())
    }

    // TODO: Implement Hostname/IP validation
    fn request() -> ClientResult<()> {
        use std::io::Read;
        use std::io::Write;
        use std::net::TcpStream;
        use std::time::Instant;
        let connect_str = "localhost:8080";
        let mut buf: [u8; 1024] = [0; 1024];
        let start = Instant::now();
        let mut stream = TcpStream::connect(&connect_str)?;
        let request = Request::get().path(UrlPath::root()).finish();

        stream.write_all(request.to_string().as_bytes())?;

        let num_bytes = stream.read(&mut buf)?;
        // let duration = start.elapsed();

        let status_code = Self::get_status_code(&buf[..num_bytes])?;

        println!("Status: {}", status_code);
        Ok(())
    }

    fn get_status_code(response: &[u8]) -> ClientResult<StatusCode> {
        let mut lines = response.split(|&b| b == b'\n');
        if let Some(status_line) = lines.next() {
            let response_str = String::from_utf8_lossy(status_line);
            if let Some(status_line) = response_str.split("\r\n").next() {
                if let Some(code_str) = status_line.split(" ").nth(1) {
                    let code = code_str.parse::<u16>()?;
                    return Ok(code.try_into()?);
                }
            }
        }
        Err(ClientError::custom(
            "Not found payload for StatusCode parsing",
        ))
    }
}
