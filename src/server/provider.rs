use serde::{Deserialize, Serialize};
use crate::server::provider::papermc::{download_papermc_build, get_papermc_latest_build, get_papermc_versions, PAPERMC_ARRAY};

mod papermc;

pub const BINARY_FILE_NAME: &str = "server.jar";
pub const SOFTWARE_ARRAY: [Software; 4] = PAPERMC_ARRAY;

pub struct Software {
    pub id: &'static str,
    pub provider: Provider
}

pub enum Provider {
    PaperMC
}

#[derive(Deserialize, Serialize)]
pub struct Version {
    pub software: Option<String>,
    pub version: Option<String>,
    pub build: Option<u32>
}

pub fn get_software_from_id(id: &String) -> Option<&Software> {
    return SOFTWARE_ARRAY.iter().filter(|software| software.id.eq(id)).next();
}

pub fn get_versions(software: &Software) -> Vec<String> {
    return match software.provider {
        Provider::PaperMC => {
            get_papermc_versions(software.id)
        }
    }
}

pub fn get_latest_build(software: &Software, version: &str) -> u32 {
    return match software.provider {
        Provider::PaperMC => {
            get_papermc_latest_build(software.id, version)
        }
    }
}

pub fn download_build(software: &Software, version: &str, build: u32) {
    match software.provider {
        Provider::PaperMC => {
            download_papermc_build(software.id, version, build)
        }
    }
}