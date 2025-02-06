use anyhow::{anyhow, Result};

use crate::{nvy_config::{does_config_exist, does_file_exist, load_config, save_config, Profile, CONFIG_FILE_NAME}, success, warn};

pub fn run_profiles() -> Result<()> {
    if !does_config_exist() {
        return Err(anyhow!(
            "{} does not exist in the current directory, please run `nvy init` first.",
            CONFIG_FILE_NAME
        ));
    }

    let config = load_config()?;
    if config.profiles.is_empty() {
        println!("No profiles defined.");
        return Ok(());
    }

    println!("profiles:");
    for (name, profiles) in &config.profiles {
        println!("  - {}:", name);
        for profile in profiles {
            println!("    {}", profile);
        }
    }

    Ok(())
}

pub fn run_profiles_set(profile: &String, file: &String) -> Result<()> {
    if !does_config_exist() {
        return Err(anyhow!(
            "{} does not exist in the current directory, please run `nvy init` first.",
            CONFIG_FILE_NAME
        ));
    }

    if !does_file_exist(file) {
        return Err(anyhow!(
            "File {} does not exist in the current directory.",
            file
        ));
    }

    let mut config = load_config()?;
    config.profiles.insert(
        profile.clone(),
        vec![Profile {
            path: file.clone(),
        }],
    );

    save_config(&config)?;

    success!(
        "Set profile {} with path {}",
        profile,
        file
    );

    Ok(())
}

pub fn run_profiles_remove(profile: &String) -> Result<()> {
    if !does_config_exist() {
        return Err(anyhow!(
            "{} does not exist in the current directory, please run `nvy init` first.",
            CONFIG_FILE_NAME
        ));
    }

    let mut config = load_config()?;
    if !config.profiles.contains_key(profile) {
        warn!("Profile {} does not exist.", profile);
        return Ok(());
    }

    config.profiles.remove(profile);
    save_config(&config)?;

    success!("Removed profile {}", profile);

    Ok(())
}

