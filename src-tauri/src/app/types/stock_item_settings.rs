use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StockItemSettings {
    // WTB Settings
    pub volume_threshold: i64,
    pub profit_threshold: i64,
    pub avg_price_cap: i64,
    pub trading_tax_cap: i64,
    pub max_total_price_cap: i64,
    pub price_shift_threshold: i64,
    pub buy_quantity: i64,
    pub min_wtb_profit_margin: i64,
    pub blacklist: Vec<String>,

    // WTS Settings
    pub min_sma: i64,
    pub min_profit: i64,
}

impl Default for StockItemSettings {
    fn default() -> Self {
        StockItemSettings {
            min_sma: 3,
            min_profit: 10,
            volume_threshold: 15,
            profit_threshold: 10,
            avg_price_cap: 600,
            trading_tax_cap: -1,
            buy_quantity: 1,
            max_total_price_cap: 100000,
            price_shift_threshold: -1,
            blacklist: vec![],
            min_wtb_profit_margin: -1,
        }
    }
}
