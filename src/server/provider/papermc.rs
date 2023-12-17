use std::fs::File;
use std::io;
use log::info;
use crate::server::provider::{BINARY_FILE_NAME, Provider, Software};

pub const PAPERMC_ARRAY: [Software; 4] = [
    Software {
        id: "paper", provider: Provider::PaperMC
    }, Software {
        id: "folia", provider: Provider::PaperMC
    }, Software {
        id: "velocity", provider: Provider::PaperMC
    }, Software {
        id: "waterfall", provider: Provider::PaperMC
    }
];

const BASE_API_URL: &str = "https://api.papermc.io/v2/projects/";

pub fn get_papermc_versions(software: &str) -> Vec<String> {
    let response = minreq::get(format!("{}{}", BASE_API_URL, software)).send().expect("Failed to get versions from papermc api");
    if response.status_code == 200 {
        let response = json::parse(response.as_str().expect("Failed to get content of request")).expect("Failed to parse downloaded json");
        let mut versions = Vec::new();
        for entry in response["versions"].members() {
            versions.push(String::from(entry.as_str().expect("Failed to get entry from versions array")));
        }
        return versions;
    } else {
        panic!("Failed to get versions from papermc api. Got status code {}", response.status_code);
    }
}

pub fn get_papermc_latest_build(software: &str, version: &str) -> u32 {
    let response = minreq::get(format!("{}{}/versions/{}", BASE_API_URL, software, version)).send().expect("Failed to get builds from papermc api");
    if response.status_code == 200 {
        let response = json::parse(response.as_str().expect("Failed to get content of request")).expect("Failed to parse downloaded json");
        let mut latest_build = 0u32;
        for entry in response["builds"].members() {
            let entry = entry.as_u32().expect("Failed to get entry from builds array");
            if entry > latest_build {
                latest_build = entry;
            }
        }
        return latest_build;
    } else {
        panic!("Failed to get builds from papermc api. Got status code {}", response.status_code);
    }
}

pub fn download_papermc_build(software: &str, version: &str, build: u32) {
    let url = format!("{}{}/versions/{}/builds/{}/downloads/{}-{}-{}.jar", BASE_API_URL, software, version, build, software, version, build);
    info!("Downloading file from {}...", url);
    let response = minreq::get(url).send().expect("Failed to download jar from papermc api");
    if response.status_code == 200 {
        let mut file = File::create(BINARY_FILE_NAME).expect("Failed to open binary file");
        io::copy(&mut response.as_bytes(), &mut file).expect("Failed to copy byte stream from memory to disk");
    } else {
        panic!("Failed to download jar from papermc api. Got status code {}", response.status_code);
    }
}