use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use crate::error::AppError;
use crate::{helper, logger};
use eyre::eyre;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettingsState {
    pub live_scraper: LiveScraperSettings,
    pub whisper_scraper: WhisperSettings,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LiveScraperSettings {
    pub volume_threshold: i64,
    pub range_threshold: i64,
    pub avg_price_cap: i64,
    pub max_total_price_cap: i64,
    pub price_shift_threshold: i64,
    pub blacklist: Vec<String>,
    pub whitelist: Vec<String>,
    pub strict_whitelist: bool,
    pub webhook: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WhisperSettings {
    pub ping_on_notif: bool,
    pub webhook: String,
}
// Allow us to run AuthState::default()
impl Default for SettingsState {
    fn default() -> Self {
        Self {
            live_scraper: LiveScraperSettings {
                volume_threshold: 15,
                range_threshold: 10,
                avg_price_cap: 600,
                max_total_price_cap: 100000,
                price_shift_threshold: -1,
                blacklist: vec![],
                whitelist: vec![],
                strict_whitelist: false,
                webhook: "".to_string(),
            },
            whisper_scraper: WhisperSettings {
                ping_on_notif: false,
                webhook: "".to_string(),
            },
        }
    }
}
impl SettingsState {
    fn get_file_path() -> PathBuf {
        let app_path = helper::get_app_roaming_path();
        let settings_path = app_path.join("settings.json");
        settings_path
    }
    pub fn setup() -> Result<Self, AppError> {
        let path_ref = Self::get_file_path();
        if path_ref.exists() {
            let (se, vaild) = Self::read_from_file()?;
            if vaild {
                Ok(se)
            } else {
                se.save_to_file()?;
                Ok(se)
            }
        } else {
            let default_settings = SettingsState::default();
            default_settings.save_to_file()?;
            Ok(default_settings)
        }
    }

    pub fn save_to_file(&self) -> Result<(), AppError> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| AppError::new("Settings", eyre!(e.to_string())))?;
        let mut file = File::create(Self::get_file_path())
            .map_err(|e| AppError::new("Settings", eyre!(e.to_string())))?;
        file.write_all(json.as_bytes())
            .map_err(|e| AppError::new("Settings", eyre!(e.to_string())))?;
        Ok(())
    }

    pub fn read_from_file() -> Result<(Self, bool), AppError> {
        let mut file = File::open(Self::get_file_path())
            .map_err(|e| AppError::new("Settings", eyre!(e.to_string())))?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| AppError::new("Settings", eyre!(e.to_string())))?;
        Ok(Self::validate_json(&content)?)
    }

    fn validate_json(json_str: &str) -> Result<(Self, bool), AppError> {
        let mut is_valid = true;
        // Parse the JSON string into a Value object
        let mut json_value: Value = serde_json::from_str(json_str)
            .map_err(|e| AppError::new("Settings", eyre!(e.to_string())))?;

        // Create a default SettingsState object
        let default_settings = SettingsState::default();

        // Check for nested properties within 'live_scraper'
        if let Some(live_scraper) = json_value.get_mut("live_scraper") {
            if live_scraper.get("volume_threshold").is_none() {
                live_scraper["volume_threshold"] =
                    Value::from(default_settings.live_scraper.volume_threshold);
                is_valid = false;
            }
            if live_scraper.get("range_threshold").is_none() {
                live_scraper["range_threshold"] =
                    Value::from(default_settings.live_scraper.range_threshold);
                is_valid = false;
            }
            if live_scraper.get("avg_price_cap").is_none() {
                live_scraper["avg_price_cap"] =
                    Value::from(default_settings.live_scraper.avg_price_cap);
                is_valid = false;
            }
            if live_scraper.get("max_total_price_cap").is_none() {
                live_scraper["max_total_price_cap"] =
                    Value::from(default_settings.live_scraper.max_total_price_cap);
                is_valid = false;
            }
            if live_scraper.get("price_shift_threshold").is_none() {
                live_scraper["price_shift_threshold"] =
                    Value::from(default_settings.live_scraper.price_shift_threshold);
                is_valid = false;
            }
            if live_scraper.get("blacklist").is_none() {
                live_scraper["blacklist"] = Value::from(default_settings.live_scraper.blacklist);
                is_valid = false;
            }
            if live_scraper.get("whitelist").is_none() {
                live_scraper["whitelist"] = Value::from(default_settings.live_scraper.whitelist);
                is_valid = false;
            }
            if live_scraper.get("strict_whitelist").is_none() {
                live_scraper["strict_whitelist"] =
                    Value::from(default_settings.live_scraper.strict_whitelist);
                is_valid = false;
            }
            if live_scraper.get("webhook").is_none() {
                live_scraper["webhook"] = Value::from(default_settings.live_scraper.webhook);
                is_valid = false;
            }
        } else {
            // If 'live_scraper' itself doesn't exist, add it
            json_value["live_scraper"] = serde_json::to_value(default_settings.live_scraper)
                .map_err(|e| AppError::new("Settings", eyre!(e.to_string())))?;
            is_valid = false;
        }

        // Check for nested properties within 'whisper_scraper'
        if let Some(whisper_scraper) = json_value.get_mut("whisper_scraper") {
            if whisper_scraper.get("ping_on_notif").is_none() {
                whisper_scraper["ping_on_notif"] =
                    Value::from(default_settings.whisper_scraper.ping_on_notif);
                is_valid = false;
            }
            if whisper_scraper.get("webhook").is_none() {
                whisper_scraper["webhook"] = Value::from(default_settings.whisper_scraper.webhook);
                is_valid = false;
            }
        } else {
            // If 'live_scraper' itself doesn't exist, add it
            json_value["whisper_scraper"] = serde_json::to_value(default_settings.whisper_scraper)
                .map_err(|e| AppError::new("Settings", eyre!(e.to_string())))?;
            logger::info_con("Settings", "Added 'whisper_scraper' to settings.json");
            is_valid = false;
        }

        // Deserialize the updated JSON object into a SettingsState struct
        let deserialized: SettingsState = serde_json::from_value(json_value)
            .map_err(|e| AppError::new("Settings", eyre!(e.to_string())))?;

        Ok((deserialized, is_valid))
    }
}
