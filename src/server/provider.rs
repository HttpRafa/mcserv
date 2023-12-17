use serde::{Deserialize, Serialize};
use crate::server::provider::papermc::PAPERMC_ARRAY;

pub mod papermc;

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

pub fn get_versions(software: &String, provider: Provider) -> Vec<String> {
    return vec![];
}