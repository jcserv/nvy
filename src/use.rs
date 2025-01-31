use anyhow::{anyhow, Result};
use std::fs;

use crate::config::{does_config_exist, get_profile_path, load_config};

pub fn run_use(profile: &String) -> Result<(), anyhow::Error> {
    if !does_config_exist() {
        return Err(anyhow!("nv.yml does not exist in the current directory, please run `nv init` first."));
    }
    let config = load_config()?;
    let path = get_profile_path(&config, profile)?;

    if !does_file_exist(&path) {
        return Err(anyhow!("Provided path {} under profile {} does not exist.", path, profile));
    }

    generate_shell_commands(&path);

    Ok(())
}

fn does_file_exist(path: &String) -> bool {
    fs::metadata(path).is_ok()
}

fn escape_shell_value(value: &str) -> String {
    format!("'{}'", value.replace("'", "'\\''"))
}

fn generate_shell_commands(path: &str) {
    let contents = fs::read_to_string(&path).unwrap();

    for line in contents.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            if let Some((key, value)) = trimmed.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                
                if !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                    eprintln!("Warning: Skipping invalid environment variable name: {}", key);
                    continue;
                }
                
                let escaped_value = escape_shell_value(value);
                println!("export {}={}", key, escaped_value);
            }
        }
    }
}