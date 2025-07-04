use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::enums::stock_mode::StockMode;
use crate::enums::trade_mode::TradeMode;
use crate::utils::modules::error::AppError;
use crate::utils::modules::logger::LoggerOptions;
use crate::{helper, logger};
use eyre::eyre;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettingsState {
    // Debug Mode
    pub debug: Vec<String>,
    pub tos_uuid: String,
    pub cross_play: bool,
    pub notification_ids: Vec<String>,
    // Warframe Log Path
    pub wf_log_path: String,
    pub http: HttpConfig,
    pub live_scraper: LiveScraperSettings,
    pub notifications: Notifications,
    // Analytics Settings
    pub analytics: AnalyticsSettings,
    // Generate Trade Message Settings
    pub generate_trade_msg: GenerateTradeMsgSettings,
    pub summary_settings: SummarySettings,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SummarySettings {
    pub resent_days: i64, // How many days to keep the summary
    pub resent_transactions: i64, // How many transactions to keep in the summary
    pub categories: Vec<SummaryCategorySetting>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SummaryCategorySetting {
    pub icon: String,
    pub name: String,
    pub types: Vec<String>,
    pub tags: Vec<String>,
}
impl SummaryCategorySetting {
    pub fn new(icon: &str, name: &str, types: Vec<&str>, tags: Vec<&str>) -> Self {
        Self {
            icon: icon.to_string(),
            name: name.to_string(),
            types: types.iter().map(|s| s.to_string()).collect(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
        }
    }
    
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenerateTradeMsgSettings {
    pub wts_items: TradeMsgSettings,
    pub wtb_items: TradeMsgSettings,
    pub wts_riven: TradeMsgSettings,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeMsgSettings {
    pub prefix: String,
    pub suffix: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LiveScraperSettings {
    // Stock Mode
    pub stock_mode: StockMode,
    // Trade Mode's
    pub trade_modes: Vec<TradeMode>,
    // Should delete other trade types, Ex: If you are selling, should you delete buy orders or wishlists etc
    pub should_delete_other_types: bool,
    // Discord Webhook
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
    pub trading_tax_cap: i64,
    pub max_total_price_cap: i64,
    pub price_shift_threshold: i64,
    pub blacklist: Vec<String>,
    pub report_to_wfm: bool,
    pub auto_trade: bool, // Will add order to you stock automatically or remove it if you have it
    pub min_sma: i64,
    pub min_profit: i64,
    pub min_wtb_profit_margin: i64,
    pub auto_delete: bool,
    pub buy_quantity: i64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnalyticsSettings {
    pub stock_item: bool,
    pub stock_riven: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StockRivenSettings {
    pub min_profit: i64,
    pub threshold_percentage: f64,
    pub limit_to: i64,
    pub update_interval: i64, // in seconds
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Notification {
    pub discord_notify: bool,
    pub system_notify: bool,
    pub content: String,
    pub title: String,
    // Use For Discord
    pub webhook: Option<String>,
    pub user_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Notifications {
    pub on_new_conversation: Notification,
    pub on_wfm_chat_message: Notification,
    pub on_new_trade: Notification,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HttpConfig {
    pub host: String,
    pub port: i64,
}
// Allow us to run AuthState::default()
impl Default for SettingsState {
    fn default() -> Self {
        Self {
            tos_uuid: "".to_string(),
            cross_play: false,
            debug: vec!["*".to_string()],
            notification_ids: vec!["".to_string()],
            wf_log_path: "".to_string(),
            http: HttpConfig {
                host: "localhost".to_string(),
                port: 8080,
            },
            generate_trade_msg: GenerateTradeMsgSettings {
                wts_items: TradeMsgSettings {
                    prefix: "WTS ".to_string(),
                    suffix: None,
                },
                wtb_items: TradeMsgSettings {
                    prefix: "WTB ".to_string(),
                    suffix: None,
                },
                wts_riven: TradeMsgSettings {
                    prefix: "WTS Rivens ".to_string(),
                    suffix: None,
                },
            },
            live_scraper: LiveScraperSettings {
                stock_mode: StockMode::All,
                trade_modes: vec![TradeMode::Buy,TradeMode::Sell,TradeMode::WishList],
                should_delete_other_types: true,              
                webhook: "".to_string(),
                stock_item: StockItemSettings {
                    min_sma: 3,
                    min_profit: 10,
                    volume_threshold: 15,
                    range_threshold: 10,
                    avg_price_cap: 600,
                    trading_tax_cap: -1,
                    buy_quantity: 1,
                    max_total_price_cap: 100000,
                    price_shift_threshold: -1,
                    blacklist: vec![],
                    report_to_wfm: true,
                    min_wtb_profit_margin: -1,
                    auto_trade: true,
                    auto_delete: true,
                },
                stock_riven: StockRivenSettings {
                    min_profit: 25,
                    threshold_percentage: 15.0,
                    limit_to: 5,
                    update_interval: 120, // in seconds
                },
            },
            notifications: Notifications {
                on_new_conversation: Notification {
                    discord_notify: false,
                    system_notify: true,
                    content: "From: <PLAYER_NAME>".to_string(),
                    title: "New Conversation".to_string(),
                    webhook: Some("".to_string()),
                    user_ids: Some(vec![]),
                },
                on_wfm_chat_message: Notification {
                    discord_notify: false,
                    system_notify: true,
                    content: "From: <WFM_MESSAGE>".to_string(),
                    title: "New WFM Message".to_string(),
                    webhook: Some("".to_string()),
                    user_ids: Some(vec![]),
                },
                on_new_trade: Notification {
                    discord_notify: false,
                    system_notify: true,
                    content: "From: <PLAYER_NAME>\nOffered: <OF_COUNT> Received: <RE_COUNT> Plat: <TOTAL_PLAT>"
                        .to_string(),
                    title: "Item <TR_TYPE>".to_string(),
                    webhook: Some("".to_string()),
                    user_ids: Some(vec![]),
                },
            },
            analytics: AnalyticsSettings { 
                stock_item: true,
                stock_riven: true,
            },
            summary_settings: SummarySettings {
                resent_days: 7,
                resent_transactions: 10,
                categories: vec![
                    SummaryCategorySetting::new("imgs/categories/mods.png","Mod",vec![],vec!["mod"]),
                    SummaryCategorySetting::new("imgs/categories/arcane.png","Arcane",vec![],vec!["arcane_enhancement"]),
                    SummaryCategorySetting::new("imgs/categories/set.png","Set",vec![],vec!["set"]),
                    SummaryCategorySetting::new("imgs/categories/prime.png","Prime",vec![],vec!["prime"]),
                    SummaryCategorySetting::new("imgs/categories/axi-intact.png","Relic",vec![],vec!["relic"]),
                    SummaryCategorySetting::new("imgs/categories/rivenIcon2.png","Riven",vec!["riven"], vec![]),
                ],
            },
        }
    }
}
impl SettingsState {
    fn get_file_path() -> PathBuf {
        let app_path = helper::get_app_storage_path();
        let settings_path = app_path.join("settings.json");
        settings_path
    }
    pub fn setup() -> Result<Self, AppError> {
        let path_ref = Self::get_file_path();
        if path_ref.exists() {
            let (se, valid) = Self::read_from_file()?;
            if valid {
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

    pub fn is_wf_log_valid(&self) -> Result<bool, AppError> {
        if !self.wf_log_path.is_empty() && !PathBuf::from(&self.wf_log_path).exists() {
            return Err(AppError::new(
                "Settings",
                eyre::eyre!(format!(
                    "Warframe EE.log path does not exist [J]{}[J]",
                    json!({
                        "i18n_key": "wf_log_path_not_exist",
                        "path": self.wf_log_path
                    })
                )),
            ));
        }
        Ok(true)
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
        // Parse the JSON string into a Value object
        let json_value: Value = serde_json::from_str(json_str)
            .map_err(|e| AppError::new("Settings", eyre!(e.to_string())))?;

        // Required properties for the settings.json file
        let required_json = serde_json::to_value(SettingsState::default())
            .map_err(|e| AppError::new("Settings", eyre!(e.to_string())))?;

        // Validate the JSON object against the required properties
        let (validated_json, missing_properties) =
            helper::validate_json(&json_value, &required_json, "");

        // Check for missing properties
        if !missing_properties.is_empty() {
            for property in missing_properties.clone() {
                logger::warning(
                    "Settings",
                    &format!("Missing property: {}", property),
                    LoggerOptions::default(),
                );
            }
        }

        // Deserialize the updated JSON object into a SettingsState struct
        let deserialized: SettingsState = serde_json::from_value(validated_json)
            .map_err(|e| AppError::new("Settings", eyre!(e.to_string())))?;

        Ok((deserialized, missing_properties.is_empty()))
    }

    pub fn has_trade_mode(&self, mode: TradeMode) -> bool {
        self.live_scraper.trade_modes.contains(&mode)
    }
}
