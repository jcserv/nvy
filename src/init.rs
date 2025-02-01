use anyhow::Result;
use glob::glob;
use std::collections::BTreeMap;
use std::io::{self};
use std::path::PathBuf;

use crate::config::{does_config_exist, is_target_shell, load_config, save_config, Config, Profile, DEFAULT_TARGET};
use crate::log::{message, wrap_yellow};
use crate::{success, warn};

pub fn run_init() -> Result<()> {
    let mut target = String::from(DEFAULT_TARGET);
    let mut ignore = vec![".env.example".to_string()];

    if does_config_exist() {
        if !prompt_reinit()? {
            warn!("Initialization cancelled.");
            return Ok(());
        }

        let config = load_config()?;
        if !is_target_shell(&config) {
            target = config.target.clone();
            ignore.push(target.clone().to_string());
        }
    }

    let env_files = discover_env_files(ignore)?;
    init_config(&target, env_files)?;
    Ok(())
}

fn prompt_reinit() -> Result<bool> {
    let prompt = "Do you want to reinitialize? [Y/n]";
    let formatted: String = wrap_yellow(prompt);

    message(vec![
        "An existing nv configuration file was found in the current directory.",
        &formatted,
    ]);

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    Ok(input != "n" && input != "no")
}

fn discover_env_files(ignore: Vec<String>) -> Result<Vec<PathBuf>> {
    Ok(glob(".env*")?
        .filter_map(|result| {
            result.ok().and_then(|path| {
                let path_str = path.to_string_lossy();
                if ignore.iter().any(|ignored| path_str.contains(ignored)) {
                    None
                } else {
                    Some(path)
                }
            })
        })
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

fn init_config(target: &str, env_files: Vec<PathBuf>) -> Result<()> {
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

    let config = Config { target: target.to_string(), profiles };
    let res = save_config(&config);
    match res {
        Ok(()) => {
            success!("Initialized nvy.yaml in file mode, pointing to {}; run `nvy export <file>` to change the target.", DEFAULT_TARGET);
            Ok(())
        },
        Err(e) => Err(anyhow::anyhow!(e)),
    }    
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::config::TARGET_SHELL;

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
        init_config(TARGET_SHELL,empty_files)?;

        let content = fs::read_to_string("nvy.yaml")?;
        assert!(content.contains("default:"));
        assert!(content.contains("path: .env"));
        
        fs::remove_file("nvy.yaml")?;
        Ok(())
    }
}