use std::io::stdin;
use log::{error, info};

pub fn request_string() -> String {
    let mut value = String::new();
    stdin().read_line(&mut value).expect("Failed to read user input");
    return String::from(value.trim());
}

pub fn request_number() -> Option<u32> {
    return match request_string().parse::<u32>() {
        Ok(value) => {
            Some(value)
        }
        Err(_) => {
            error!("Please enter a valid number");
            None
        }
    }
}

pub fn request_from_list(list: &Vec<String>) -> String {
    for i in 0..list.len() {
        info!("=> {} {}| {}", i, " ".repeat(((list.len().checked_ilog10().unwrap_or(0) + 1) - (i.checked_ilog10().unwrap_or(0) + 1)) as usize), list[i]);
    }
    loop {
        let value = request_string();
        match value.parse::<usize>() {
            Ok(value) => {
                if value >= list.len() {
                    error!("Please enter a number between 0 and {}", list.len() - 1);
                } else {
                    return list[value].clone();
                }
            }
            Err(_) => {
                for entry in list {
                    if entry.eq_ignore_ascii_case(&value) {
                        return entry.clone();
                    }
                }
                error!("The value you entered is currently not supported or available")
            }
        }
    }
}