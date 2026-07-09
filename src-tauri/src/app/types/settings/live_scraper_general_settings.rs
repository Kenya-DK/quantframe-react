use crate::enums::{StockMode, TradeMode};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LiveScraperGeneralSettings {
    pub report_to_wfm: bool,
    pub auto_delete: bool,
    pub auto_trade: bool,
    pub stock_mode: StockMode,
    pub trade_modes: Vec<TradeMode>,
    pub delete_conflicting_orders: bool,
}

impl Default for LiveScraperGeneralSettings {
    fn default() -> Self {
        Self {
            report_to_wfm: true,
            auto_trade: true,
            auto_delete: true,

            stock_mode: StockMode::All,
            trade_modes: vec![TradeMode::Buy, TradeMode::Sell, TradeMode::WishList],
            delete_conflicting_orders: false,
        }
    }
}
