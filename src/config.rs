use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;

pub const CONFIG_FILE_NAME: &str = "nv.yaml";

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(serialize_with = "ordered_map")]
    pub profiles: BTreeMap<String, Vec<Profile>>,
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

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub path: String,
}

pub fn does_config_exist() -> bool {
    fs::metadata(CONFIG_FILE_NAME).is_ok()
}

pub fn load_config() -> Result<Config, anyhow::Error> {
    let res = fs::read_to_string(CONFIG_FILE_NAME);
    match res {
        Ok(content) => Ok(serde_yaml::from_str(&content)?),
        Err(e) => Err(anyhow::anyhow!(e)),
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