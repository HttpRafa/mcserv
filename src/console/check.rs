use log::{error, info};
use crate::console::request::{request_number, request_string};
use crate::server::provider::{SOFTWARE_IDS};

pub fn check_number(option: &mut Option<u32>, message: &str, example: &str) {
    if option.is_none() {
        info!("{}", message);
        info!("Example: {}", example);
        while option.is_none() {
            *option = request_number();
        }
    }
}

pub fn check_software(option: &mut Option<String>) {
    if option.is_none() {
        info!("Which server software do you want to use?");
        for i in 0..SOFTWARE_IDS.len() {
            info!("=> {} | {}", i, SOFTWARE_IDS[i]);
        }
        while option.is_none() {
            let value = request_string();
            match value.parse::<usize>() {
                Ok(value) => {
                    if value >= SOFTWARE_IDS.len() {
                        error!("Please enter a number between 0 and {}", SOFTWARE_IDS.len() - 1);
                    } else {
                        *option = Some(String::from(SOFTWARE_IDS[value]));
                    }
                }
                Err(_) => {
                    for software in SOFTWARE_IDS {
                        if software.eq_ignore_ascii_case(&value) {
                            *option = Some(String::from(software));
                            return;
                        }
                    }
                    error!("The software you entered is currently not supported or available")
                }
            }
        }
    }
}