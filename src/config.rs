use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    #[serde(serialize_with = "ordered_map")]
    pub profiles: BTreeMap<String, Vec<Profile>>,
}

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub path: String,
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