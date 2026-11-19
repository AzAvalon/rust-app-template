//! Application entry point: serves the probes on `APP_PORT` (default 8000).

use std::env;
use std::net::TcpListener;
use std::process::{self, ExitCode};

fn main() -> ExitCode {
    // Kubernetes stops pods with SIGTERM. The server holds no state, so it
    // exits at once; an app with work in flight would drain it here first.
    if let Err(error) = ctrlc::set_handler(|| {
        println!("stop signal received, exiting");
        process::exit(0);
    }) {
        eprintln!("cannot install the signal handler: {error}");
        return ExitCode::FAILURE;
    }

    let port = env::var("APP_PORT").unwrap_or_else(|_| "8000".to_owned());
    let listener = match TcpListener::bind(format!("0.0.0.0:{port}")) {
        Ok(listener) => listener,
        Err(error) => {
            eprintln!("cannot listen on port {port}: {error}");
            return ExitCode::FAILURE;
        }
    };
    println!("listening on port {port}");

    match rust_app_template::serve(&listener) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("server stopped: {error}");
            ExitCode::FAILURE
        }
    }
}
