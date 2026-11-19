//! Integration tests: they start the real `app` binary, as the container does,
//! and talk to it over TCP. Each file in `tests/` is its own test crate; cargo
//! and nextest run them with the unit tests.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Child, Command};
use std::thread;
use std::time::{Duration, Instant};

/// The app binary, killed when the test ends (even on a failed assertion).
struct App(Child);

impl Drop for App {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

/// Starts the binary on a free port and waits until it accepts connections.
fn start_app() -> (App, u16) {
    // Ask the OS for a free port, then hand it to the app.
    let port = TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let child = Command::new(env!("CARGO_BIN_EXE_app"))
        .env("APP_PORT", port.to_string())
        .spawn()
        .expect("start the app binary");
    let app = App(child);

    let deadline = Instant::now() + Duration::from_secs(10);
    while TcpStream::connect(("127.0.0.1", port)).is_err() {
        assert!(
            Instant::now() < deadline,
            "the app did not listen on {port}"
        );
        thread::sleep(Duration::from_millis(20));
    }
    (app, port)
}

fn get(port: u16, path: &str) -> String {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    write!(stream, "GET {path} HTTP/1.1\r\nhost: test\r\n\r\n").unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    response
}

#[test]
fn healthz_answers_200() {
    let (_app, port) = start_app();

    let response = get(port, "/healthz");

    assert!(response.starts_with("HTTP/1.1 200 OK"), "{response}");
    assert!(response.ends_with("\r\n\r\nok"), "{response}");
}

#[test]
fn unknown_path_answers_404() {
    let (_app, port) = start_app();

    let response = get(port, "/nope");

    assert!(response.starts_with("HTTP/1.1 404 Not Found"), "{response}");
}
