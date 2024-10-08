use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread,
    time::Duration,
};

use crate::{
    constants::MAX_RESPONSE_LENGTH,
    http::{
        request::Request,
        response::Response,
        result::{H10ClientError, H10LibError, H10LibResult},
    },
};

#[derive(Debug)]
pub struct HttpClient;
impl HttpClient {
    pub fn launch<S: ToString>(
        request: Request,
        connection_string: S,
        timeout: Duration,
    ) -> H10LibResult<String> {
        let arc_req_str = Arc::new(request.to_string());
        let cloned_arc_req_str = Arc::clone(&arc_req_str);

        let arc_connect_str = Arc::new(connection_string.to_string());
        let cloned_arc_connect_str = Arc::clone(&arc_connect_str);

        let (tx, rx) = mpsc::channel();
        let should_terminate = Arc::new(AtomicBool::new(false));
        let should_terminate_clone = Arc::clone(&should_terminate);

        let handle = thread::spawn(move || -> H10LibResult<()> {
            let is_done = should_terminate_clone.load(Ordering::SeqCst);

            if !is_done {
                let request = cloned_arc_req_str;
                let connection_string = cloned_arc_connect_str;
                let res_response_str = Self::request(request, connection_string);

                let is_done_after_response = should_terminate_clone.load(Ordering::SeqCst);

                if !is_done_after_response {
                    tx.send(res_response_str?)?;
                }
            }
            Ok(())
        });

        let mut maybe_response: Option<String> = None;

        match rx.recv_timeout(timeout) {
            Ok(response_str) => {
                should_terminate.store(true, Ordering::SeqCst);
                maybe_response = Some(response_str);
            }
            Err(_) => {
                should_terminate.store(true, Ordering::SeqCst);
            }
        };

        match handle.join() {
            Ok(inner) => {
                inner?;
                maybe_response.ok_or(H10LibError::ClientError(H10ClientError::Timeout))
            }
            Err(err) => {
                dbg!(&err);
                Err(H10LibError::ClientError(H10ClientError::InternalError(
                    format!("CLientError: Internal Error: {err:?}"),
                )))
            }
        }
    }

    fn request(request_str: Arc<String>, connect_str: Arc<String>) -> H10LibResult<String> {
        use std::io::{Read, Write};
        use std::net::TcpStream;

        let mut response_buffer: [u8; MAX_RESPONSE_LENGTH] = [0; MAX_RESPONSE_LENGTH];

        let mut stream = TcpStream::connect(connect_str.as_str())?;

        stream.write_all(request_str.as_bytes())?;

        stream.read(&mut response_buffer)?;

        let response = Response::parse(&response_buffer)?;

        Ok(response.to_string())
    }
}
