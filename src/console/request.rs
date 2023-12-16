use std::io::stdin;
use log::error;

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