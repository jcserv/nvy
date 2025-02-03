use anyhow::{anyhow, Result};
use std::{collections::{HashMap, HashSet}, fs};

use crate::{config::{does_config_exist, get_profile_path, is_target_shell, load_config, Config, CONFIG_FILE_NAME}, success};

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

    fn to_env_file_line(&self) -> Option<String> {
        self.value.as_ref().map(|val| format!("{}={}", self.key, val))
    }
}

struct ExportResult {
    unset_vars: HashMap<String, EnvVar>,
    new_vars: HashMap<String, EnvVar>,
}

pub fn run_use(profiles: &Vec<String>) -> Result<()> {
    if !does_config_exist() {
        return Err(anyhow!(
            "{} does not exist in the current directory, please run `nvy init` first.",
            CONFIG_FILE_NAME
        ));
    }

    let mut result = ExportResult {
        unset_vars: HashMap::new(),
        new_vars: HashMap::new(),
    };

    let config = load_config()?;

    for profile in profiles {
        let profile_vars = export_profile(&config, profile)?;
        result.unset_vars.extend(profile_vars.unset_vars);
        result.new_vars.extend(profile_vars.new_vars);
    }

    if is_target_shell(&config) {
        for (_, var) in result.unset_vars {
            println!("{}", var.to_shell_command());
        }
        for (_, var) in result.new_vars {
            println!("{}", var.to_shell_command());
        }
        println!(
            "export {}={}",
            PROFILE_ENV_VAR,
            escape_shell_value(&profiles.join(","))
        );
    } else {
        let mut content = String::new();
        for (_, var) in result.new_vars {
            if let Some(line) = var.to_env_file_line() {
                content.push_str(&line);
                content.push('\n');
            }
        }
        let target = config.target.clone();
        fs::write(config.target, content)?;
        success!("Exported profile(s) {} to file {}", profiles.join(", "), target);
    }

    Ok(())
}

fn export_profile(config: &Config, profile: &String) -> Result<ExportResult> {
    let new_path = get_profile_path(config, profile)?;

    if !does_file_exist(&new_path) {
        return Err(anyhow!(
            "Provided path {} under profile {} does not exist.",
            new_path,
            profile
        ));
    }

    let unset_vars = get_current_profile_vars()?
        .into_iter()
        .map(|key| (key.clone(), EnvVar::new(key, None)))
        .collect();

    let new_vars = parse_env_file(&new_path)?
        .into_iter()
        .filter(|var| var.is_valid())
        .map(|var| (var.key.clone(), var))
        .collect();

    Ok(ExportResult {
        unset_vars,
        new_vars,
    })
}

fn escape_shell_value(value: &str) -> String {
    // trim any surrounding quotes if they exist
    let clean_value = value.trim_matches(|c| c == '"' || c == '\'');
    format!("'{}'", clean_value.replace('\'', "'\\''"))
}

fn does_file_exist(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn get_current_profile_vars() -> Result<HashSet<String>> {
    let mut vars = HashSet::new();
    
    let current_profiles = match std::env::var(PROFILE_ENV_VAR) {
        Ok(profiles) => profiles,
        Err(_) => return Ok(vars),
    };
    
    let config = match load_config() {
        Ok(cfg) => cfg,
        Err(_) => return Ok(vars),
    };
    
    for profile in current_profiles.split(',') {
        let path = match get_profile_path(&config, &profile.to_string()) {
            Ok(p) => p,
            Err(_) => continue,
        };
        
        let contents = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        
        for line in parse_env_line(&contents) {
            if line.is_valid() {
                vars.insert(line.key);
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_shell_value_should_wrap_basic_string_in_quotes() {
        assert_eq!(escape_shell_value("hello"), "'hello'");
    }

    #[test]
    fn test_escape_shell_value_should_remove_existing_double_quotes() {
        assert_eq!(escape_shell_value("\"hello\""), "'hello'");
    }

    #[test]
    fn test_escape_shell_value_should_remove_existing_single_quotes() {
        assert_eq!(escape_shell_value("'hello'"), "'hello'");
    }

    #[test]
    fn test_escape_shell_value_should_escape_internal_single_quotes() {
        assert_eq!(escape_shell_value("he'llo"), "'he'\\''llo'");
    }

    #[test]
    fn test_escape_shell_value_should_preserve_internal_double_quotes() {
        assert_eq!(escape_shell_value("he\"llo"), "'he\"llo'");
    }

    #[test]
    fn test_escape_shell_value_should_handle_empty_string() {
        assert_eq!(escape_shell_value(""), "''");
    }

    #[test]
    fn test_escape_shell_value_should_handle_mixed_quotes() {
        assert_eq!(escape_shell_value("\"hello'world\""), "'hello'\\''world'");
    }

    #[test]
    fn test_escape_shell_value_should_handle_multiple_internal_single_quotes() {
        assert_eq!(escape_shell_value("it's O'clock"), "'it'\\''s O'\\''clock'");
    }
}
