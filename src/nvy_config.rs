use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::to_string;
use std::collections::BTreeMap;
use std::{fmt, fs};

pub const TARGET_SHELL: &str = "sh";

/// Set it to this so that there's no chance of accidentally overwriting a user's config
pub const DEFAULT_TARGET: &str = ".env.nvy";

pub const CONFIG_FILE_NAME: &str = "nvy.yaml";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub target: String,

    #[serde(default)]
    pub current_profiles: Vec<String>,

    #[serde(serialize_with = "ordered_map")]
    pub profiles: BTreeMap<String, Vec<Profile>>,
}

pub fn is_target_shell(cfg: &Config) -> bool {
    cfg.target == TARGET_SHELL
}

pub fn does_file_exist(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn does_config_exist() -> bool {
    does_file_exist(CONFIG_FILE_NAME)
}

pub fn load_config() -> Result<Config, anyhow::Error> {
    let res = fs::read_to_string(CONFIG_FILE_NAME);
    match res {
        Ok(content) => Ok(serde_yaml::from_str(&content)?),
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}

pub fn save_config(config: &Config) -> Result<()> {
    let yaml = to_string(&config)?;
    fs::write(CONFIG_FILE_NAME, yaml)?;
    Ok(())
}

pub fn get_profile_path(config: &Config, profile: &String) -> Result<String, anyhow::Error> {
    let path = config.profiles.get(profile);
    match path {
        Some(path) => {
            if path.len() == 0 {
                return Err(anyhow!("Profile {} does not have any paths defined.", profile));
            }
            if path.len() > 1 {
                return Err(anyhow!("Profile {} has more than one path defined.", profile));
            }
            let path_str = path[0].path.to_string();
            if path_str.len() == 0 {
                return Err(anyhow!("Profile {} has an empty path defined.", profile));
            }
            return Ok(path_str);
        },
        None => Err(anyhow!("Profile {} does not exist in the {} file.", profile, CONFIG_FILE_NAME)),
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "target: {}", self.target)?;
        let mut curr_profiles_str = self.current_profiles.join(", ");
        if curr_profiles_str.is_empty() {
            curr_profiles_str = "none".to_string();
        }
        writeln!(f, "current: {}", curr_profiles_str)?;
        writeln!(f, "profiles:")?;
        
        for (name, profiles) in &self.profiles {
            if let Some(profile) = profiles.first() {
                writeln!(f, "  - {}: \"{}\"", name, profile.path)?;
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub path: String,
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "- {}", self.path)
    }
}

fn ordered_map<S>(value: &BTreeMap<String, Vec<Profile>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeMap;
    let mut map = serializer.serialize_map(Some(value.len()))?;
    
    if let Some(default) = value.get("default") {
        map.serialize_entry("default", default)?;
    }
    
    for (k, v) in value.iter() {
        if k != "default" {
            map.serialize_entry(k, v)?;
        }
    }
    
    map.end()
}