use std::thread::sleep;
use std::time::Duration;
use log::info;
use simplelog::{ColorChoice, Config, LevelFilter, TerminalMode, TermLogger};
use crate::console::check::{check_number, check_installation, check_eula};
use crate::server::configuration::load_server;
use crate::server::start_server;

mod server;
mod console;

fn main() {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).expect("Failed to setup logging");

    loop {
        // Load config from file
        let mut server = load_server();

        // Validate that the information is correct
        check_number(&mut server.memory.min, "What is the minimum amount of RAM in MiB that the server should be allocated?", "1024");
        check_number(&mut server.memory.max, "What is the maximum amount of RAM in MiB that the server should have available?", "2048");
        check_installation(&mut server.version);

        // Write changes to disk
        server.write();

        // Check eula
        check_eula();

        // Start server
        start_server(&server);

        // Wait 10 seconds
        for i in (1..=10).rev() {
            info!("Restarting server in {} seconds. Press Ctl+C to exit the program", i);
            sleep(Duration::from_secs(1));
        }
    }
}