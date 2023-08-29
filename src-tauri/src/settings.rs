use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use crate::helper;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettingsState {
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
        let mut file = File::open(Self::get_file_path())?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let settings = serde_json::from_str(&content)?;
        Ok(settings)
    }
}
