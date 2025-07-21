use std::vec;

use serde::{Deserialize, Serialize};

use crate::{app::types::*, enums::*};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LiveScraperSettings {
    // General Settings
    pub report_to_wfm: bool,
    pub auto_delete: bool,
    pub auto_trade: bool, // Will add order to you stock automatically or remove it if you have it
    // Stock Mode
    pub stock_mode: StockMode,
    // Trade Mode's
    pub trade_modes: Vec<TradeMode>,
    // Should delete other trade types, Ex: If you are selling, should you delete buy orders or wishlists etc
    pub should_delete_other_types: bool,
    // Stock Item Settings
    pub stock_item: StockItemSettings,
    // Stock Riven Settings
    pub stock_riven: StockRivenSettings,
}

impl Default for LiveScraperSettings {
    fn default() -> Self {
        LiveScraperSettings {
            stock_mode: StockMode::All,
            trade_modes: vec![TradeMode::Buy, TradeMode::Sell, TradeMode::WishList],
            should_delete_other_types: false,
            stock_item: StockItemSettings::default(),
            stock_riven: StockRivenSettings::default(),
            report_to_wfm: true,
            auto_trade: true,
            auto_delete: true,
        }
    }
}
