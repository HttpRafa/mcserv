use std::fs;
use std::path::Path;
use log::error;
use serde::{Deserialize, Serialize};

const SERVER_FILENAME: &str = "server.toml";

#[derive(Deserialize, Serialize)]
pub struct Configuration {
    pub min_memory: Option<u32>,
    pub max_memory: Option<u32>,
    pub software: Option<String>,
    pub version: Option<String>,

}

impl Configuration {

    pub fn write(self) {
        fs::write(Path::new(SERVER_FILENAME), toml::to_string(&self).expect("Failed to convert configuration to toml")).expect("Failed to write configuration to file");
    }

}

fn create_empty_server_configuration() -> Configuration {
    return Configuration {
        min_memory: None,
        max_memory: None,
        software: None,
        version: None,
    }
}

pub fn load_server() -> Configuration {
    let server_file_path = Path::new(SERVER_FILENAME);
    if !server_file_path.exists() {
        return create_empty_server_configuration();
    }
    let result = fs::read_to_string(server_file_path);
    return match result {
        Ok(data) => {
            toml::from_str(data.as_str()).unwrap()
        }
        Err(error) => {
            error!("Failed to read server configuration: {}", error.to_string());
            create_empty_server_configuration()
        }
    }
}