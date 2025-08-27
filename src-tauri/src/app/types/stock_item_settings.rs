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
    pub quantity_per_trade: i64,

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
            min_wtb_profit_margin: -1,
            quantity_per_trade: 1,
        }
    }
}

impl StockItemSettings {
    pub fn get_query_id(&self) -> String {
        format!(
            "volume_threshold:{};profit_threshold:{};avg_price_cap:{};trading_tax_cap:{};max_total_price_cap:{};price_shift_threshold:{};buy_quantity:{};min_wtb_profit_margin:{};min_sma:{};min_profit:{}",
            self.volume_threshold,
            self.profit_threshold,
            self.avg_price_cap,
            self.trading_tax_cap,
            self.max_total_price_cap,
            self.price_shift_threshold,
            self.buy_quantity,
            self.min_wtb_profit_margin,
            self.min_sma,
            self.min_profit
        )
    }
}
