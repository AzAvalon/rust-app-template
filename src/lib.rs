//! Put the application here. Until then it only answers the Kubernetes
//! probes: `GET /healthz` returns 200.
//!
//! The server uses the standard library only and handles one connection at a
//! time, with read and write timeouts so a slow client cannot hold it. Replace
//! it with a real HTTP stack (axum, actix-web...) when the app needs one.

use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

/// How long a client may take to send its request line or read the answer.
const IO_TIMEOUT: Duration = Duration::from_secs(5);
/// Longest request line read; longer ones are cut and get a 404.
const MAX_REQUEST_LINE: u64 = 8 * 1024;

/// Accepts connections forever. Errors on one connection are logged and do
/// not stop the server.
///
/// # Errors
///
/// Never returns `Ok`; returns the error of `accept` only if the listener
/// itself fails.
pub fn serve(listener: &TcpListener) -> io::Result<()> {
    for stream in listener.incoming() {
        if let Err(error) = handle(&stream?) {
            eprintln!("connection error: {error}");
        }
    }
    Ok(())
}

fn handle(stream: &TcpStream) -> io::Result<()> {
    stream.set_read_timeout(Some(IO_TIMEOUT))?;
    stream.set_write_timeout(Some(IO_TIMEOUT))?;

    let mut request_line = String::new();
    BufReader::new(stream)
        .take(MAX_REQUEST_LINE)
        .read_line(&mut request_line)?;

    let response: &[u8] = if request_line.starts_with("GET /healthz ") {
        b"HTTP/1.1 200 OK\r\ncontent-length: 2\r\nconnection: close\r\n\r\nok"
    } else {
        b"HTTP/1.1 404 Not Found\r\ncontent-length: 0\r\nconnection: close\r\n\r\n"
    };
    let mut stream = stream;
    stream.write_all(response)
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    use super::serve;

    #[test]
    fn healthz_answers_200() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        thread::spawn(move || serve(&listener));

        let mut stream = TcpStream::connect(address).unwrap();
        stream
            .write_all(b"GET /healthz HTTP/1.1\r\nhost: test\r\n\r\n")
            .unwrap();
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();

        assert!(response.starts_with("HTTP/1.1 200 OK"), "{response}");
    }
}
