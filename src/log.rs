use inline_colorization::*;
use std::io::{self, Write};

pub fn wrap_yellow(message: &str) -> String {
    format!("{color_yellow}{message}{color_reset}")
}

pub fn success(message: &str) {
    println!("{color_green}Success{color_reset}\t{message}");
}

pub fn warn(message: &str) {
    println!("{color_green}Warning{color_reset}\t{message}");
}

pub fn error(message: &str) {
    println!("{color_red}Error{color_reset}\t{message}");
}

pub fn message(messages: Vec<&str>) {
    for message in messages {
        print!("{message}\n");
    }
    let _ = io::stdout().flush();
}