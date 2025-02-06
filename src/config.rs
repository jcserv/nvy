use anyhow::{anyhow, Result};

use crate::nvy_config::{does_config_exist, load_config, CONFIG_FILE_NAME};

pub fn run_config() -> Result<()> {
    if !does_config_exist() {
        return Err(anyhow!(
            "{} does not exist in the current directory, please run `nvy init` first.",
            CONFIG_FILE_NAME
        ));
    }

    let config = load_config()?;
    println!("{}", config);

    Ok(())
}