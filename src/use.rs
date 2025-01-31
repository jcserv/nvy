use anyhow::{anyhow, Result};
use std::{collections::HashSet, fs};

use crate::config::{does_config_exist, get_profile_path, load_config, CONFIG_FILE_NAME};

const PROFILE_ENV_VAR: &str = "NV_CURRENT_PROFILE";

#[derive(Debug)]
struct EnvVar {
    key: String,
    value: Option<String>,
}

impl EnvVar {
    fn new(key: String, value: Option<String>) -> Self {
        Self { key, value }
    }

    fn is_valid(&self) -> bool {
        self.key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
    }

    fn to_shell_command(&self) -> String {
        match &self.value {
            Some(val) => format!("export {}={}", self.key, escape_shell_value(val)),
            None => format!("unset {}", self.key),
        }
    }
}

pub fn run_use(profile: &String) -> Result<()> {
    if !does_config_exist() {
        return Err(anyhow!(
            "{} does not exist in the current directory, please run `nv init` first.",
            CONFIG_FILE_NAME
        ));
    }

    let config = load_config()?;
    let new_path = get_profile_path(&config, profile)?;

    if !does_file_exist(&new_path) {
        return Err(anyhow!(
            "Provided path {} under profile {} does not exist.",
            new_path,
            profile
        ));
    }

    let unset_vars = get_current_profile_vars()?
        .into_iter()
        .map(|key| EnvVar::new(key, None));

    let new_vars = parse_env_file(&new_path)?
        .into_iter()
        .filter(EnvVar::is_valid)
        .chain(std::iter::once(EnvVar::new(
            PROFILE_ENV_VAR.to_string(),
            Some(profile.clone()),
        )));

    for var in unset_vars.chain(new_vars) {
        println!("{}", var.to_shell_command());
    }

    Ok(())
}

fn does_file_exist(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn escape_shell_value(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn get_current_profile_vars() -> Result<HashSet<String>> {
    let mut vars = HashSet::new();
    
    let current_profile = match std::env::var(PROFILE_ENV_VAR) {
        Ok(profile) => profile,
        Err(_) => return Ok(vars),
    };
    
    let config = match load_config() {
        Ok(cfg) => cfg,
        Err(_) => return Ok(vars),
    };
    
    let path = match get_profile_path(&config, &current_profile) {
        Ok(p) => p,
        Err(_) => return Ok(vars),
    };
    
    let contents = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return Ok(vars),
    };
    
    for line in parse_env_line(&contents) {
        if line.is_valid() {
            vars.insert(line.key);
        }
    }
    
    Ok(vars)
}

fn parse_env_file(path: &str) -> Result<Vec<EnvVar>> {
    let contents = fs::read_to_string(path)?;
    Ok(parse_env_line(&contents).collect())
}

fn parse_env_line(contents: &str) -> impl Iterator<Item = EnvVar> + '_ {
    contents.lines().filter_map(|line| {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return None;
        }

        let (key, value) = line.split_once('=')?;
        Some(EnvVar::new(key.trim().to_string(), Some(value.trim().to_string())))
    })
}