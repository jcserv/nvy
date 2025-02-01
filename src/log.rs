use inline_colorization::*;
use std::io::{self, Write};

pub fn wrap_yellow(message: &str) -> String {
    format!("{color_yellow}{message}{color_reset}")
}

#[macro_export]
macro_rules! __log_internal {
    ($predicate:expr, $color:expr, $template:expr) => {
        println!("{}{}{}\t{}", 
            $color,
            $predicate,
            inline_colorization::color_reset,
            $template
        );
    };
    ($predicate:expr, $color:expr, $template:expr, $($arg:tt)*) => {
        println!(
            "{}{}{}\t{}", 
            $color,
            $predicate,
            inline_colorization::color_reset,
            format!($template, $($arg)*)
        );
    };
}

#[macro_export]
macro_rules! success {
    ($template:expr) => {
        crate::__log_internal!("Success", inline_colorization::color_green, $template);
    };
    ($template:expr, $($arg:tt)*) => {
        crate::__log_internal!("Success", inline_colorization::color_green, $template, $($arg)*);
    };
}

#[macro_export]
macro_rules! warn {
    ($template:expr) => {
        crate::__log_internal!("Warning", inline_colorization::color_yellow, $template);
    };
    ($template:expr, $($arg:tt)*) => {
        crate::__log_internal!("Warning", inline_colorization::color_yellow, $template, $($arg)*);
    };
}

#[macro_export]
macro_rules! error {
    ($template:expr) => {
        crate::__log_internal!("Error", inline_colorization::color_red, $template);
    };
    ($template:expr, $($arg:tt)*) => {
        crate::__log_internal!("Error", inline_colorization::color_red, $template, $($arg)*);
    };
}

pub fn message(messages: Vec<&str>) {
    for message in messages {
        print!("{message}\n");
    }
    let _ = io::stdout().flush();
}