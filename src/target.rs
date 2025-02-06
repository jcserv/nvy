use anyhow::{anyhow, Result};

use crate::{nvy_config::{does_config_exist, load_config, save_config,CONFIG_FILE_NAME}, success};

pub fn run_target() -> Result<()> {
    if !does_config_exist() {
        return Err(anyhow!(
            "{} does not exist in the current directory, please run `nvy init` first.",
            CONFIG_FILE_NAME
        ));
    }

    let config = load_config()?;
    println!("target: {}", config.target);

    Ok(())
}

pub fn run_target_set(file: &String) -> Result<()> {
    if !does_config_exist() {
        return Err(anyhow!(
            "{} does not exist in the current directory, please run `nvy init` first.",
            CONFIG_FILE_NAME
        ));
    }

    let mut config = load_config()?;
    config.target = file.to_string();
    save_config(&config)?;

    let msg = format!("Target set to {}", file);
    success!(&msg);

    Ok(())
}
