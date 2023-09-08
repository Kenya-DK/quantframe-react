use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use crate::helper;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettingsState {
    pub live_scraper: LiveScraperSettings,
    pub volume_threshold: i64,
    pub range_threshold: i64,
    pub avg_price_cap: i64,
    pub max_total_price_cap: i64,
    pub price_shift_threshold: i64,
    pub blacklist: Vec<String>,
    pub whitelist: Vec<String>,
    pub strict_whitelist: bool,
    pub ping_on_notif: bool,
    pub webhook: String,
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
    pub ping_on_notif: bool,
    pub webhook: String,
}
// Allow us to run AuthState::default()
impl Default for SettingsState {
    fn default() -> Self {
        Self {
            live_scraper: {
                volume_threshold: 15,
                range_threshold: 10,
                avg_price_cap: 600,
                max_total_price_cap: 100000,
                price_shift_threshold: -1,
                blacklist: vec![],
                whitelist: vec![],
                strict_whitelist: false,
                ping_on_notif: false,
                webhook: "".to_string(),
            },
            volume_threshold: 15,
            range_threshold: 10,
            avg_price_cap: 600,
            max_total_price_cap: 100000,
            price_shift_threshold: -1,
            blacklist: vec![],
            whitelist: vec![],
            strict_whitelist: false,
            ping_on_notif: false,
            webhook: "".to_string(),
        }
    }
}
impl SettingsState {
    fn get_file_path() -> PathBuf {
        let app_path = helper::get_app_roaming_path();
        let settings_path = app_path.join("settings.json");
        settings_path
    }
    pub fn setup() -> io::Result<Self> {
        let path_ref = Self::get_file_path();
        if path_ref.exists() {
            Self::read_from_file()
        } else {
            let default_settings = SettingsState::default();
            default_settings.save_to_file()?;
            Ok(default_settings)
        }
    }

    pub fn save_to_file(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        let mut file = File::create(Self::get_file_path())?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn read_from_file() -> io::Result<Self> {
        let mut file = File::open(Self::get_file_path())
            .map_err(|e| {AppError("Settings", eyre!(e.to_string()))} )?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| {AppError("Settings", eyre!(e.to_string()))} )?;
        
        let settings = Self::validate_json(&content)?;
        Ok(settings)
    }

    fn validate_json(json_str: &str) -> Result<Self, AppError> {
        // Parse the JSON string into a Value object
        let mut json_value: Value = serde_json::from_str(json_str)
            .map_err(|e| {AppError("Settings", eyre!(e.to_string()))} )?;

        // Create a default SettingsState object
        let mut default_settings = SettingsState::default();

        // Check for nested properties within 'live_scraper'
        if let Some(live_scraper) = json_value.get_mut("live_scraper") {
            if live_scraper.get("volume_threshold").is_none() {
                live_scraper["volume_threshold"] = Value::from(default_settings.live_scraper.volume_threshold);
            }
            if live_scraper.get("range_threshold").is_none() {
                live_scraper["range_threshold"] = Value::from(default_settings.live_scraper.range_threshold);
            }
            if live_scraper.get("avg_price_cap").is_none() {
                live_scraper["avg_price_cap"] = Value::from(default_settings.live_scraper.avg_price_cap);
            }
            if live_scraper.get("max_total_price_cap").is_none() {
                live_scraper["max_total_price_cap"] = Value::from(default_settings.live_scraper.max_total_price_cap);
            }
            if live_scraper.get("price_shift_threshold").is_none() {
                live_scraper["price_shift_threshold"] = Value::from(default_settings.live_scraper.price_shift_threshold);
            }
            if live_scraper.get("blacklist").is_none() {
                live_scraper["blacklist"] = Value::from(default_settings.live_scraper.blacklist);
            }
            if live_scraper.get("whitelist").is_none() {
                live_scraper["whitelist"] = Value::from(default_settings.live_scraper.whitelist);
            }
            if live_scraper.get("strict_whitelist").is_none() {
                live_scraper["strict_whitelist"] = Value::from(default_settings.live_scraper.strict_whitelist);
            }
            if live_scraper.get("ping_on_notif").is_none() {
                live_scraper["ping_on_notif"] = Value::from(default_settings.live_scraper.ping_on_notif);
            }
            if live_scraper.get("webhook").is_none() {
                live_scraper["webhook"] = Value::from(default_settings.live_scraper.webhook);
            }
        } else {
            // If 'live_scraper' itself doesn't exist, add it
            json_value["live_scraper"] = serde_json::to_value(default_settings.live_scraper)?;
        }

        // Deserialize the updated JSON object into a SettingsState struct
        let deserialized: SettingsState = serde_json::from_value(json_value)
            .map_err(|e| {AppError("Settings", eyre!(e.to_string()))} )?;

        Ok(deserialized)
    }
}
