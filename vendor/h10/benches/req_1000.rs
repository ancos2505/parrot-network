use std::thread;

use h10::http::{
    request::Request,
    response::Response,
    result::{H10LibError, H10LibResult},
    status_code::StatusCode,
    url_path::UrlPath,
};

fn main() {
    let num_of_requests = 10;
    let num_of_rounds = 100;
    let mut cur_round = 0;
    while cur_round < num_of_rounds {
        println!("Round: [{}]", cur_round + 1);
        next_wave(num_of_requests);
        cur_round += 1;
    }
}

fn next_wave(num_of_requests: usize) {
    for i in 0..num_of_requests {
        thread::spawn(move || {
            println!("Req #{}", i + 1);
            request().unwrap();
        });
    }
}
fn request() -> H10LibResult<()> {
    use std::io::Read;
    use std::io::Write;
    use std::net::TcpStream;
    use std::time::Instant;
    let connect_str = "localhost:8080";
    let mut buf: [u8; 1024] = [0; 1024];
    let start = Instant::now();
    let mut stream = TcpStream::connect(&connect_str).unwrap();
    let request = Request::get().path(UrlPath::root()).finish();
    stream.write_all(request.to_string().as_bytes()).unwrap();
    let num_bytes = stream.read(&mut buf).unwrap();
    // let duration = start.elapsed();

    let status_code = get_status_code(&buf[..num_bytes])?;

    println!(
        "Status: {} in {} secs",
        status_code,
        start.elapsed().as_secs_f32()
    );
    Ok(())
}

fn get_status_code(response_bytes: &[u8]) -> H10LibResult<StatusCode> {
    let (_, status_code, ..) = Response::parse(response_bytes)?.into_inner();
    Ok(status_code)
}
