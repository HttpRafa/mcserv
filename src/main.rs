use simplelog::{ColorChoice, Config, LevelFilter, TerminalMode, TermLogger};
use crate::console::check::{check_number, check_version};
use crate::server::configuration::load_server;

mod server;
mod console;

fn main() {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).expect("Failed to setup logging");

    let mut server = load_server();
    check_number(&mut server.min_memory, "What is the minimum amount of RAM in KiB that the server should be allocated?", "1024");
    check_number(&mut server.max_memory, "What is the maximum amount of RAM in KiB that the server should have available?", "2048");
    check_version(&mut server.version);

    server.write()
}