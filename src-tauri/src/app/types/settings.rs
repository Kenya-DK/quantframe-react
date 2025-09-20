use std::{fs::File, io::Read, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use utils::{get_location, validate_json, Error};

use crate::{app::types::*, helper};

fn get_path() -> PathBuf {
    helper::get_app_storage_path().join("settings.json")
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub live_scraper: LiveScraperSettings,
    pub summary_settings: SummarySettings,
    pub advanced_settings: AdvancedSettings,
    pub debugging: DebuggingSettings,
    pub notifications: NotificationsSetting,
    pub tos_uuid: String,
}
impl Default for Settings {
    fn default() -> Self {
        Settings {
            live_scraper: LiveScraperSettings::default(),
            summary_settings: SummarySettings::default(),
            advanced_settings: AdvancedSettings::default(),
            notifications: NotificationsSetting::default(),
            debugging: DebuggingSettings::default(),
            tos_uuid: String::new(),
        }
    }
}

impl Settings {
    pub fn load() -> Result<Self, Error> {
        let path = &get_path();
        if !path.exists() {
            let user = Settings::default();
            user.save()?;
            return Ok(user);
        }
        // Open the file and read its contents
        let mut file = File::open(path).map_err(|e| {
            Error::from_io(
                "Settings",
                path,
                "Failed to open auth file",
                e,
                get_location!(),
            )
        })?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|e| {
            Error::from_io(
                "Settings",
                path,
                "Failed to read auth file",
                e,
                get_location!(),
            )
        })?;

        // Parse the JSON string into a Value object
        let json_value: Value = serde_json::from_str(&content).map_err(|e| {
            Error::from_json(
                "Settings",
                &path,
                &content,
                "Failed to parse settings.json",
                e,
                get_location!(),
            )
        })?;

        // Required properties for the settings.json file
        let required_json = serde_json::to_value(Settings::default()).map_err(|e| {
            Error::new(
                "Settings",
                &format!("Failed to serialize default settings: {}", e),
                get_location!(),
            )
        })?;

        // Validate the JSON object against the required properties
        let (validated_json, missing_properties) = validate_json(&json_value, &required_json, "");

        // Check for missing properties
        if !missing_properties.is_empty() {
            for property in missing_properties.clone() {
                println!("Missing property: {}", property);
            }
        }
        let data = match serde_json::from_value::<Settings>(validated_json) {
            Ok(user) => user,
            Err(_) => {
                let default_user = Settings::default();
                default_user.save()?;
                return Ok(default_user);
            }
        };
        if missing_properties.len() > 0 {
            data.save()?;
        }
        Ok(data)
    }
    pub fn save(&self) -> Result<(), Error> {
        let path = &get_path();
        let content = serde_json::to_string_pretty(self).map_err(|e| {
            Error::from_json(
                "Settings",
                path,
                "N/A",
                "Failed to serialize settings",
                e,
                get_location!(),
            )
        })?;
        std::fs::write(path, content).map_err(|e| {
            Error::from_io(
                "Settings",
                path,
                "Failed to write settings file",
                e,
                get_location!(),
            )
        })?;
        Ok(())
    }
}
