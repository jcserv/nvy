use anyhow::Result;
use glob::glob;
use serde_yaml::to_string;
use std::collections::BTreeMap;
use std::fs;
use std::io::{self};
use std::path::PathBuf;

use crate::config::{does_config_exist, Config, Profile, CONFIG_FILE_NAME};
use crate::log::{message, success, warn};

pub fn run_init() -> Result<()> {
    if does_config_exist() {
        if !prompt_reinit()? {
            warn("Initialization cancelled.");
            return Ok(());
        }
    }

    let env_files = discover_env_files()?;
    init_config(env_files)?;
    Ok(())
}

fn prompt_reinit() -> Result<bool> {
    message(vec![
        "An existing nv configuration file was found in the current directory.",
        "Do you want to reinitialize? [Y/n]",
    ]);

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    Ok(input != "n" && input != "no")
}

fn discover_env_files() -> Result<Vec<PathBuf>> {
    // TODO: Ignore .env.example?
    Ok(glob(".env*")?
        .filter_map(Result::ok)
        .collect())
}

/// Map an env file path to its corresponding profile name
fn get_profile_name(file_name: &str) -> Option<String> {
    if file_name == ".env" {
        Some("default".to_string())
    } else if let Some(suffix) = file_name.strip_prefix(".env.") {
        if !suffix.is_empty() {
            Some(suffix.to_string())
        } else {
            None
        }
    } else {
        None
    }
}

fn init_config(env_files: Vec<PathBuf>) -> Result<()> {
    let mut profiles = BTreeMap::new();

    profiles.insert(
        "default".to_string(),
        vec![Profile {
            path: ".env".to_string(),
        }],
    );

    for file in env_files {
        let file_name = file.to_string_lossy();
        if let Some(profile_name) = get_profile_name(&file_name) {
            if profile_name != "default" { 
                profiles.insert(
                    profile_name.to_string(),
                    vec![Profile {
                        path: file_name.into_owned(),
                    }],
                );
            }
        }
    }

    let config = Config { profiles };
    let yaml = to_string(&config)?;
    fs::write(CONFIG_FILE_NAME, yaml)?;

    success("Initialized nv.yml in the current directory.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_profile_name() {
        assert_eq!(get_profile_name(".env"), Some("default".to_string()));
        assert_eq!(get_profile_name(".env.local"), Some("local".to_string()));
        assert_eq!(get_profile_name(".env.prod"), Some("prod".to_string()));
        assert_eq!(get_profile_name(".env.staging"), Some("staging".to_string()));
        assert_eq!(get_profile_name(".env."), None);
        assert_eq!(get_profile_name("env"), None);
        assert_eq!(get_profile_name(".environment"), None);
    }

    #[test]
    fn test_init_config_empty_dir() -> Result<()> {
        let empty_files = Vec::new();
        init_config(empty_files)?;

        let content = fs::read_to_string("nv.yml")?;
        assert!(content.contains("default:"));
        assert!(content.contains("path: .env"));
        
        fs::remove_file("nv.yml")?;
        Ok(())
    }
}