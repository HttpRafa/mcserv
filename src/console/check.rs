use std::fs;
use std::path::Path;
use log::{error, info};
use crate::console::request::{request_from_list, request_number};
use crate::server::EULA_FILE_NAME;
use crate::server::provider::{BINARY_FILE_NAME, download_build, get_latest_build, get_software_from_id, get_versions, Software, SOFTWARE_ARRAY, Version};

pub fn check_number(option: &mut Option<u32>, message: &str, example: &str) {
    if option.is_none() {
        info!("{}", message);
        info!("Example: {}", example);
        while option.is_none() {
            *option = request_number();
        }
    }
}

pub fn check_eula() {
    let file = Path::new(EULA_FILE_NAME);
    if !file.exists() || !fs::read_to_string(file).expect("Failed to read eula file").contains("eula=true") {
        info!("By accepting this EULA, you acknowledge that failure to comply with the terms may result in the termination of your access to Minecraft");
        info!("Do you accept the EULA?");
        if request_from_list(&vec!["yes".to_string(), "no".to_string()]).eq("yes") {
            fs::write(file, "eula=true").expect("Failed to write to eula file");
        }
    }
}

pub fn check_installation(version: &mut Version) {
    check_software(&mut version.software);
    let software = get_software_from_id(&version.software.as_ref().expect("Failed to continue with empty software type")).expect("The software specified in the server.toml is invalid");
    if version.version.is_none() {
        check_software_version(software, &mut version.version);
    }

    let software_version = &mut version.version.as_ref().expect("Failed to continue with empty version type");
    // Check for updates
    info!("Checking for updates...");
    match check_software_build(software, software_version, &mut version.build) {
        None => {}
        Some(latest_build) => {
            info!("Downloading build {} of {} for version {}...", latest_build, software.id, software_version);
            download_build(software, software_version, latest_build);
        }
    }
}

fn check_software(option: &mut Option<String>) {
    match option {
        Some(software) => {
            if get_software_from_id(software).is_none() {
                error!("The software specified in the server.toml is invalid");
                *option = None;
                check_software(option);
            }
        }
        None => {
            info!("Which server software do you want to use?");
            let software_ids: Vec<String> = SOFTWARE_ARRAY.into_iter().map(|software| String::from(software.id)).collect();
            *option = Some(request_from_list(&software_ids));
        }
    }
}

fn check_software_version(software: &Software, option: &mut Option<String>) {
    info!("Getting versions for {}....", software.id);
    let versions = get_versions(software);
    info!("Which version do you want to use?");
    *option = Some(request_from_list(&versions));
}

fn check_software_build(software: &Software, version: &str, option: &mut Option<u32>) -> Option<u32> {
    // Check if server binary file exists
    if !Path::new(BINARY_FILE_NAME).exists() {
        *option = None;
    }

    info!("Getting build for {}....", software.id);
    let latest_build = get_latest_build(software, version);
    info!("The latest build of {} is {}", software.id, latest_build);
    return match option {
        None => {
            *option = Some(latest_build);
            return Some(latest_build);
        }
        Some(current_build) => {
            if latest_build > *current_build {
                info!("The currently installed build {} is outdated, installing the latest build {}", current_build, latest_build);
                *option = Some(latest_build);
                return Some(latest_build)
            }
            info!("The installation is up to date");
            None
        }
    }
}