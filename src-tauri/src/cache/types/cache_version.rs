use std::{fs::File, io::Read, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use utils::{get_location, Error};

use crate::helper;

fn get_path() -> PathBuf {
    helper::get_app_storage_path().join("cache_version.json")
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CacheVersion {
    pub id: String,
    pub id_price: String,
}
impl Default for CacheVersion {
    fn default() -> Self {
        CacheVersion {
            id: "N/A".to_string(),
            id_price: "N/A".to_string(),
        }
    }
}
impl CacheVersion {
    pub fn load() -> Result<Self, Error> {
        let path = &get_path();
        if !path.exists() {
            let cache_version = CacheVersion::default();
            cache_version.save()?;
            return Ok(cache_version);
        }
        // Open the file and read its contents
        let mut file = File::open(path).map_err(|e| {
            Error::from_io(
                "CacheVersion",
                path,
                "Failed to open auth file",
                e,
                get_location!(),
            )
        })?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|e| {
            Error::from_io(
                "CacheVersion",
                path,
                "Failed to read auth file",
                e,
                get_location!(),
            )
        })?;

        // Parse the JSON string into a Value object
        let json_value: Value = serde_json::from_str(&content).map_err(|e| {
            Error::from_json(
                "CacheVersion",
                path,
                &content,
                "Failed to parse auth file",
                e,
                get_location!(),
            )
        })?;

        // Required properties for the settings.json file
        let required_json = serde_json::to_value(CacheVersion::default()).map_err(|e| {
            Error::from_json(
                "CacheVersion",
                path,
                "N/A",
                "Failed to serialize default user",
                e,
                get_location!(),
            )
        })?;

        // Validate the JSON object against the required properties
        let (validated_json, missing_properties) =
            helper::validate_json(&json_value, &required_json, "");

        // Check for missing properties
        if !missing_properties.is_empty() {
            for property in missing_properties.clone() {
                println!("Missing property: {}", property);
            }
        }
        match serde_json::from_value::<CacheVersion>(validated_json) {
            Ok(cache_version) => Ok(cache_version),
            Err(_) => {
                println!("Invalid JSON structure, creating default cache version.");
                let default_cache_version = CacheVersion::default();
                default_cache_version.save()?;
                return Ok(default_cache_version);
            }
        }
    }
    pub fn save(&self) -> Result<(), Error> {
        let path = &get_path();
        let content = serde_json::to_string(self).map_err(|e| {
            Error::from_json(
                "User",
                path,
                "N/A",
                "Failed to serialize user",
                e,
                get_location!(),
            )
        })?;
        std::fs::write(path, content).map_err(|e| {
            Error::from_io(
                "User",
                path,
                "Failed to write user to file",
                e,
                get_location!(),
            )
        })?;
        Ok(())
    }
}
