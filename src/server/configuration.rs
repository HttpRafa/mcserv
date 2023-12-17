use std::{fs, vec};
use std::path::Path;
use log::error;
use serde::{Deserialize, Serialize};
use crate::server::provider::Version;

const SERVER_FILENAME: &str = "server.toml";

#[derive(Deserialize, Serialize)]
pub struct Configuration {
    pub arguments: Arguments,
    pub memory: Memory,
    pub version: Version,
}

#[derive(Deserialize, Serialize)]
pub struct Arguments {
    pub jvm: Option<Vec<String>>,
    pub server: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
pub struct Memory {
    pub min: Option<u32>,
    pub max: Option<u32>,
}

impl Configuration {

    pub fn write(&self) {
        fs::write(Path::new(SERVER_FILENAME), toml::to_string(self).expect("Failed to convert configuration to toml")).expect("Failed to write configuration to file");
    }

}

fn create_empty_server_configuration() -> Configuration {
    return Configuration {
        arguments: Arguments { jvm: Some(vec![]), server: Some(vec![String::from("nogui")]) },
        version: Version {
            software: None,
            version: None,
            build: None,
        },
        memory: Memory { min: None, max: None },
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