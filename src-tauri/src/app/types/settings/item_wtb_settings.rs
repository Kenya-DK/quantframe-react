use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemWtbSettings {
    pub volume_threshold: i64,
    pub profit_threshold: i64,
    pub avg_price_cap: i64,
    pub trading_tax_cap: i64,
    pub max_total_price_cap: i64,
    pub price_shift_threshold: i64,
    pub buy_quantity: i64,
    pub min_wtb_profit_margin: i64,
    pub quantity_per_trade: i64,
    pub max_stock_quantity: i64,
    pub max_price_drop: i64,
    pub min_listings_below: i64,
}

impl Default for ItemWtbSettings {
    fn default() -> Self {
        Self {
            volume_threshold: 15,
            profit_threshold: 10,
            avg_price_cap: 600,
            trading_tax_cap: -1,
            buy_quantity: 1,
            max_total_price_cap: 100000,
            price_shift_threshold: -1,
            min_wtb_profit_margin: -1,
            quantity_per_trade: 1,
            max_stock_quantity: -1,
            max_price_drop: -1,
            min_listings_below: -1,
        }
    }
}
