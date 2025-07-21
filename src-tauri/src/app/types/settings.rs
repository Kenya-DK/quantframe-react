use std::{fs::File, io::Read, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{app::types::*, helper, utils::modules::error::AppError};

fn get_path() -> PathBuf {
    helper::get_app_storage_path().join("settings.json")
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    live_scraper: LiveScraperSettings,
}
impl Default for Settings {
    fn default() -> Self {
        Settings {
            live_scraper: LiveScraperSettings::default(),
        }
    }
}

impl Settings {
    pub fn load() -> Result<Self, AppError> {
        let path = &get_path();
        if !path.exists() {
            let user = Settings::default();
            user.save()?;
            return Ok(user);
        }
        // Open the file and read its contents
        let mut file = File::open(path)
            .map_err(|e| AppError::from_io("Settings", path, "Failed to open auth file", e))?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| AppError::from_io("Settings", path, "Failed to read auth file", e))?;

        // Parse the JSON string into a Value object
        let json_value: Value = serde_json::from_str(&content).map_err(|e| {
            AppError::from_json("Settings", &content, "Failed to parse settings.json", e)
        })?;

        // Required properties for the settings.json file
        let required_json = serde_json::to_value(Settings::default()).map_err(|e| {
            AppError::new(
                "Settings",
                &format!("Failed to serialize default settings: {}", e),
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
        match serde_json::from_value::<Settings>(validated_json) {
            Ok(user) => Ok(user),
            Err(_) => {
                println!("Invalid JSON structure, creating default user.");
                let default_user = Settings::default();
                default_user.save()?;
                return Ok(default_user);
            }
        }
    }
    pub fn save(&self) -> Result<(), AppError> {
        let path = &get_path();
        let content = serde_json::to_string(self).map_err(|e| {
            AppError::from_json("Settings", "N/A", "Failed to serialize settings", e)
        })?;
        std::fs::write(path, content)
            .map_err(|e| AppError::from_io("Settings", path, "Failed to write settings file", e))?;
        Ok(())
    }
}
