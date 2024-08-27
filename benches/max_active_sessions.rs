use std::sync::{Arc, Condvar, Mutex};
use std::thread;

// TODO

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    let num_of_requests = 10;
    let num_of_rounds = 10;
    let mut cur_round = 0;
    while cur_round < num_of_rounds {
        next_wave(&pair, num_of_requests);

        dbg!("");
        // Wait for the thread to start up.
        let (lock, cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        // As long as the value inside the `Mutex<bool>` is `false`, we wait.
        dbg!("");
        while !*started {
            dbg!("");

            started = cvar.wait(started).unwrap();
        }
        cur_round += 1;
        // dbg!(cur_round);
    }
}

fn next_wave(pair: &Arc<(Mutex<bool>, Condvar)>, num_of_requests: usize) {
    for i in 0..num_of_requests {
        let pair2 = Arc::clone(pair);
        thread::spawn(move || {
            let (lock, cvar) = &*pair2;
            println!("Inside Thread");
            let mut started = lock.lock().unwrap();
            request();
            *started = true;
            // We notify the condvar that the value has changed.
            cvar.notify_one();
        });
    }
}
fn request() {
    use std::io::Read;
    use std::io::Write;
    use std::net::TcpStream;
    use std::time::Instant;
    let connect_str = "localhost:8080";
    let mut buf: [u8; 1024] = [0; 1024];
    let start = Instant::now();
    let mut stream = TcpStream::connect(&connect_str).unwrap();
    stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
    let num_bytes = stream.read(&mut buf).unwrap();
    // let duration = start.elapsed();

    let status_code = get_status_code(&buf[..num_bytes]);

    println!("Status: {}", status_code);
}

fn get_status_code(response: &[u8]) -> u16 {
    let mut lines = response.split(|&b| b == b'\n');
    if let Some(status_line) = lines.next() {
        let response_str = String::from_utf8_lossy(status_line);
        let status_line = response_str.split("\r\n").next().unwrap();
        if let Some(code) = status_line.split(" ").nth(1) {
            return code.parse().unwrap_or(0);
        }
    }
    0
}
