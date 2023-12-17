use log::{error, info};
use crate::console::request::{request_number, request_string};
use crate::server::provider::{get_software_from_id, SOFTWARE_ARRAY, Version};

pub fn check_number(option: &mut Option<u32>, message: &str, example: &str) {
    if option.is_none() {
        info!("{}", message);
        info!("Example: {}", example);
        while option.is_none() {
            *option = request_number();
        }
    }
}

pub fn check_version(option: &mut Option<Version>) {
    if option.is_none() {
        *option = Some(Version {
            software: None,
            version: None,
            build: None,
        });
    }

    let version = option.as_mut().expect("Expected option to be present");
    check_software(&mut version.software);
    if version.version.is_none() {
        check_software_version(&version.software.as_ref().expect("Failed to get versions for empty software type"), &mut version.version);
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
            for i in 0..SOFTWARE_ARRAY.len() {
                info!("=> {} | {}", i, SOFTWARE_ARRAY[i].id);
            }
            while option.is_none() {
                let value = request_string();
                match value.parse::<usize>() {
                    Ok(value) => {
                        if value >= SOFTWARE_ARRAY.len() {
                            error!("Please enter a number between 0 and {}", SOFTWARE_ARRAY.len() - 1);
                        } else {
                            *option = Some(String::from(SOFTWARE_ARRAY[value].id));
                        }
                    }
                    Err(_) => {
                        for software in SOFTWARE_ARRAY {
                            if software.id.eq_ignore_ascii_case(&value) {
                                *option = Some(String::from(software.id));
                                return;
                            }
                        }
                        error!("The software you entered is currently not supported or available")
                    }
                }
            }
        }
    }
}

fn check_software_version(software: &String, option: &mut Option<String>) {
    let software = get_software_from_id(software);
    match software {
        Some(_) => {}
        None => {
            panic!("The software specified in the server.toml is invalid");
        }
    }
}