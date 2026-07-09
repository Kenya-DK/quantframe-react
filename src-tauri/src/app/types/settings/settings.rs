use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use utils::{
    extract_json_values, get_location, log_json_formatted, merge_json, validate_json, Error,
};

use super::*;
use crate::helper;

fn get_path() -> PathBuf {
    helper::get_app_storage_path().join("settings.json")
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    // General Settings
    pub lang: String,

    pub live_scraper: LiveScraperSettings,
    pub summary_settings: SummarySettings,
    pub advanced_settings: AdvancedSettings,
    pub log_settings: LogSettings,

    pub notifications: NotificationsSetting,
    pub generate_trade_message: GenerateTradeMessageSetting,
    pub tos_uuid: String,
    pub wf_inventory: WFInventorySettings,

    pub debugging: DebuggingSettings,
}
impl Default for Settings {
    fn default() -> Self {
        Settings {
            lang: "en".to_string(),
            live_scraper: LiveScraperSettings::default(),
            summary_settings: SummarySettings::default(),
            log_settings: LogSettings::default(),
            advanced_settings: AdvancedSettings::default(),
            notifications: NotificationsSetting::default(),
            debugging: DebuggingSettings::default(),
            generate_trade_message: GenerateTradeMessageSetting::default(),
            tos_uuid: String::new(),
            wf_inventory: WFInventorySettings::default(),
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

        let mut json_value: Value = serde_json::from_str(&content).map_err(|e| {
            Error::from_json(
                "Settings",
                &path,
                &content,
                "Failed to parse settings.json",
                e,
                get_location!(),
            )
        })?;
        let required_json = serde_json::to_value(Settings::default()).map_err(|e| {
            Error::new(
                "Settings",
                &format!("Failed to serialize default settings: {}", e),
                get_location!(),
            )
        })?;
        let mut mapping = HashMap::new();
        mapping.insert(
            "live_scraper.report_to_wfm",
            "live_scraper.general.report_to_wfm",
        );
        mapping.insert(
            "live_scraper.auto_delete",
            "live_scraper.general.auto_delete",
        );
        mapping.insert("live_scraper.auto_trade", "live_scraper.general.auto_trade");
        mapping.insert("live_scraper.stock_mode", "live_scraper.general.stock_mode");
        mapping.insert(
            "live_scraper.trade_modes",
            "live_scraper.general.trade_modes",
        );
        mapping.insert(
            "live_scraper.should_delete_other_types",
            "live_scraper.general.delete_conflicting_orders",
        );
        mapping.insert(
            "live_scraper.stock_item.blacklist",
            "live_scraper.items.general.blacklist",
        );
        mapping.insert(
            "live_scraper.stock_item.buy_list",
            "live_scraper.items.general.buy_list",
        );
        mapping.insert(
            "live_scraper.stock_item.volume_threshold",
            "live_scraper.items.wtb.volume_threshold",
        );
        mapping.insert(
            "live_scraper.stock_item.profit_threshold",
            "live_scraper.items.wtb.profit_threshold",
        );
        mapping.insert(
            "live_scraper.stock_item.avg_price_cap",
            "live_scraper.items.wtb.avg_price_cap",
        );
        mapping.insert(
            "live_scraper.stock_item.trading_tax_cap",
            "live_scraper.items.wtb.trading_tax_cap",
        );
        mapping.insert(
            "live_scraper.stock_item.max_total_price_cap",
            "live_scraper.items.wtb.max_total_price_cap",
        );
        mapping.insert(
            "live_scraper.stock_item.price_shift_threshold",
            "live_scraper.items.wtb.price_shift_threshold",
        );
        mapping.insert(
            "live_scraper.stock_item.buy_quantity",
            "live_scraper.items.wtb.buy_quantity",
        );
        mapping.insert(
            "live_scraper.stock_item.min_wtb_profit_margin",
            "live_scraper.items.wtb.min_wtb_profit_margin",
        );
        mapping.insert(
            "live_scraper.stock_item.quantity_per_trade",
            "live_scraper.items.wtb.quantity_per_trade",
        );
        mapping.insert(
            "live_scraper.stock_item.max_stock_quantity",
            "live_scraper.items.wtb.max_stock_quantity",
        );
        mapping.insert(
            "live_scraper.stock_item.min_sma",
            "live_scraper.items.wts.min_sma",
        );
        mapping.insert(
            "live_scraper.stock_item.min_profit",
            "live_scraper.items.wts.min_profit",
        );
        mapping.insert(
            "live_scraper.stock_riven.update_interval",
            "live_scraper.rivens.general.update_interval",
        );
        mapping.insert(
            "live_scraper.stock_riven.min_profit",
            "live_scraper.rivens.wts.min_profit",
        );
        mapping.insert(
            "live_scraper.stock_riven.threshold_percentage",
            "live_scraper.rivens.wts.threshold_percentage",
        );
        mapping.insert(
            "live_scraper.stock_riven.limit_to",
            "live_scraper.rivens.wts.max_results",
        );
        let result = extract_json_values(&json_value, &mapping);
        merge_json(&mut json_value, &result);

        let (validated_json, missing_properties) = validate_json(&json_value, &required_json, "");

        if !missing_properties.is_empty() {
            for property in missing_properties.clone() {
                println!("Missing property: {}", property);
            }
        }
        let mut data = match serde_json::from_value::<Settings>(validated_json) {
            Ok(user) => user,
            Err(_) => {
                let default_user = Settings::default();
                default_user.save()?;
                return Ok(default_user);
            }
        };

        if data.live_scraper.items.wtb.max_total_price_cap > 150_000 {
            data.live_scraper.items.wtb.max_total_price_cap = 150_000;
        }

        let default_template = SaveTemplateSetting::default();
        if data
            .generate_trade_message
            .templates
            .iter()
            .all(|t| t.template != default_template.template)
        {
            data.generate_trade_message.templates.push(default_template);
        }

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
