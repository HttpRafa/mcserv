use std::io;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread::{JoinHandle, spawn};
use log::{error, info, Level, log, warn};
use crate::server::configuration::Configuration;
use crate::server::provider::BINARY_FILE_NAME;

pub mod configuration;
pub mod provider;

pub const EULA_FILE_NAME: &str = "eula.txt";

pub fn start_server(configuration: &Configuration) {
    warn!("==> Server starting");
    let mut command = Command::new("java");
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    if configuration.memory.min.is_some() {
        command.arg(format!("-Xms{}m", configuration.memory.min.as_ref().unwrap()));
    }
    if configuration.memory.max.is_some() {
        command.arg(format!("-Xmx{}m", configuration.memory.max.as_ref().unwrap()));
    }
    if configuration.arguments.jvm.is_some() {
        for argument in configuration.arguments.jvm.as_ref().unwrap() {
            command.arg(argument);
        }
    }
    command.arg("-jar");
    command.arg(BINARY_FILE_NAME);
    if configuration.arguments.server.is_some() {
        for argument in configuration.arguments.server.as_ref().unwrap() {
            command.arg(argument);
        }
    }
    let mut command = command.spawn().expect("Failed to start server process");

    // Create threads to read output of server
    let stdout_handle = read_stream(command.stdout.take().expect("Failed to capture stdout"), Level::Info);
    let stderr_handle = read_stream(command.stderr.take().expect("Failed to capture stderr"), Level::Warn);

    // Wait for the server to finish
    let status = command.wait().expect("Failed to wait for program exit");

    // Wait for the reader threads to finish
    stdout_handle.join().expect("Failed to join stdout thread");
    stderr_handle.join().expect("Failed to join stderr thread");

    if status.success() {
        info!("Server stopped with exit code {}", status);
    } else {
        warn!("Server stopped with exit code {}", status);
    }
    warn!("==> Server stopped");
}

fn read_stream<T: io::Read + Send + 'static>(output: T, level: Level) -> JoinHandle<()> {
    spawn(move || {
        let reader = BufReader::new(output);
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    log!(level, "{}", line);
                }
                Err(error) => {
                    error!("Failed to read line: {}", error);
                }
            }
        }
    })
}
