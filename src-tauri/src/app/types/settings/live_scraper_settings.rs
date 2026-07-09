use super::*;
use crate::enums::TradeMode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LiveScraperSettings {
    pub general: LiveScraperGeneralSettings,
    pub items: ItemSettings,
    pub rivens: RivenSettings,
}
impl LiveScraperSettings {
    pub fn has_trade_mode(&self, mode: TradeMode) -> bool {
        self.general.trade_modes.contains(&mode)
    }
}
impl Default for LiveScraperSettings {
    fn default() -> Self {
        Self {
            general: LiveScraperGeneralSettings::default(),
            items: ItemSettings::default(),
            rivens: RivenSettings::default(),
        }
    }
}
