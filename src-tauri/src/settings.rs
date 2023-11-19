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
    pub webhook: String,
    // Stock Item Settings
    pub stock_item: StockItemSettings,
    // Stock Riven Settings
    pub stock_riven: StockRivenSettings,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StockItemSettings {
    pub volume_threshold: i64,
    // pub minimum_profit: i64,
    // pub maximum_profit: i64,
    pub range_threshold: i64,
    pub avg_price_cap: i64,
    pub max_total_price_cap: i64,
    pub price_shift_threshold: i64,
    pub blacklist: Vec<String>,
    pub whitelist: Vec<String>,
    pub report_to_wfm: bool,
    pub strict_whitelist: bool,
    // What to post sell, buy, or both
    pub order_mode: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StockRivenSettings {
    pub range_threshold: i64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WhisperSettings {
    pub enable: bool,
    pub ping_on_notif: bool,
    pub webhook: String,
}
// Allow us to run AuthState::default()
impl Default for SettingsState {
    fn default() -> Self {
        Self {
            live_scraper: LiveScraperSettings {
                webhook: "".to_string(),
                stock_item: StockItemSettings {
                    volume_threshold: 15,
                    range_threshold: 10,
                    avg_price_cap: 600,
                    max_total_price_cap: 100000,
                    price_shift_threshold: -1,
                    blacklist: vec![],
                    whitelist: vec![],
                    strict_whitelist: false,
                    report_to_wfm: true,
                    order_mode: "both".to_string(),
                },
                stock_riven: StockRivenSettings {
                    range_threshold: 25,
                },
            },
            whisper_scraper: WhisperSettings {
                enable: true,
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
            if live_scraper.get("webhook").is_none() {
                live_scraper["webhook"] = Value::from(default_settings.live_scraper.webhook);
                is_valid = false;
            }

            // Check for nested properties within 'stock_item'
            if let Some(stock_item) = live_scraper.get_mut("stock_item") {
                if stock_item.get("volume_threshold").is_none() {
                    stock_item["volume_threshold"] =
                        Value::from(default_settings.live_scraper.stock_item.volume_threshold);
                    is_valid = false;
                }
                if stock_item.get("range_threshold").is_none() {
                    stock_item["range_threshold"] =
                        Value::from(default_settings.live_scraper.stock_item.range_threshold);
                    is_valid = false;
                }
                if stock_item.get("avg_price_cap").is_none() {
                    stock_item["avg_price_cap"] =
                        Value::from(default_settings.live_scraper.stock_item.avg_price_cap);
                    is_valid = false;
                }
                if stock_item.get("max_total_price_cap").is_none() {
                    stock_item["max_total_price_cap"] =
                        Value::from(default_settings.live_scraper.stock_item.max_total_price_cap);
                    is_valid = false;
                }
                if stock_item.get("price_shift_threshold").is_none() {
                    stock_item["price_shift_threshold"] = Value::from(
                        default_settings
                            .live_scraper
                            .stock_item
                            .price_shift_threshold,
                    );
                    is_valid = false;
                }
                if stock_item.get("blacklist").is_none() {
                    stock_item["blacklist"] =
                        Value::from(default_settings.live_scraper.stock_item.blacklist);
                    is_valid = false;
                }
                if stock_item.get("report_to_wfm").is_none() {
                    stock_item["report_to_wfm"] =
                        Value::from(default_settings.live_scraper.stock_item.report_to_wfm);
                    is_valid = false;
                }
                if stock_item.get("whitelist").is_none() {
                    stock_item["whitelist"] =
                        Value::from(default_settings.live_scraper.stock_item.whitelist);
                    is_valid = false;
                }
                if stock_item.get("strict_whitelist").is_none() {
                    stock_item["strict_whitelist"] =
                        Value::from(default_settings.live_scraper.stock_item.strict_whitelist);
                    is_valid = false;
                }
                if stock_item.get("order_mode").is_none() {
                    stock_item["order_mode"] =
                        Value::from(default_settings.live_scraper.stock_item.order_mode);
                    is_valid = false;
                }
            } else {
                // If 'stock_item' itself doesn't exist, add it
                live_scraper["stock_item"] =
                    serde_json::to_value(default_settings.live_scraper.stock_item)
                        .map_err(|e| AppError::new("Settings", eyre!(e.to_string())))?;
                logger::info_con(
                    "Settings",
                    "Added 'live_scraper stock_item' to settings.json",
                );
                is_valid = false;
            }

            // Check for nested properties within 'stock_riven'
            if let Some(stock_riven) = live_scraper.get_mut("stock_riven") {
                if stock_riven.get("range_threshold").is_none() {
                    stock_riven["range_threshold"] =
                        Value::from(default_settings.live_scraper.stock_riven.range_threshold);
                    is_valid = false;
                }
            } else {
                // If 'stock_riven' itself doesn't exist, add it
                live_scraper["stock_riven"] =
                    serde_json::to_value(default_settings.live_scraper.stock_riven)
                        .map_err(|e| AppError::new("Settings", eyre!(e.to_string())))?;
                logger::info_con(
                    "Settings",
                    "Added 'live_scraper stock_riven' to settings.json",
                );
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
            if whisper_scraper.get("enable").is_none() {
                whisper_scraper["enable"] = Value::from(default_settings.whisper_scraper.enable);
                is_valid = false;
            }
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
