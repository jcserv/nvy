use anyhow::{anyhow, Result};

use crate::{config::{does_config_exist, load_config, save_config,CONFIG_FILE_NAME}, success};

pub fn run_export(target: &String) -> Result<()> {
    if !does_config_exist() {
        return Err(anyhow!(
            "{} does not exist in the current directory, please run `nvy init` first.",
            CONFIG_FILE_NAME
        ));
    }

    let mut config = load_config()?;
    config.target = target.to_string();
    save_config(&config)?;

    let msg = format!("Target set to {}", target);
    success!(&msg);

    Ok(())
}