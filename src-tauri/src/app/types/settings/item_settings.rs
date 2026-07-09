use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemSettings {
    pub general: ItemGeneralSettings,
    pub wtb: ItemWtbSettings,
    pub wts: ItemWtsSettings,
}
impl ItemSettings {
    pub fn get_query_id(&self) -> String {
        format!(
            "volume_threshold:{};profit_threshold:{};avg_price_cap:{};trading_tax_cap:{};max_total_price_cap:{};price_shift_threshold:{};buy_quantity:{};min_wtb_profit_margin:{};min_sma:{};min_profit:{}",
            self.wtb.volume_threshold,
            self.wtb.profit_threshold,
            self.wtb.avg_price_cap,
            self.wtb.trading_tax_cap,
            self.wtb.max_total_price_cap,
            self.wtb.price_shift_threshold,
            self.wtb.buy_quantity,
            self.wtb.min_wtb_profit_margin,
            self.wts.min_sma,
            self.wts.min_profit
        )
    }
}
impl Default for ItemSettings {
    fn default() -> Self {
        Self {
            general: ItemGeneralSettings::default(),
            wtb: ItemWtbSettings::default(),
            wts: ItemWtsSettings::default(),
        }
    }
}
