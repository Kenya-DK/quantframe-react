use serde::{Deserialize, Serialize};

use crate::live_scraper::ItemEntry;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DebuggingSettings {
    pub live_scraper: DebuggingLiveScraperSettings,
}

impl Default for DebuggingSettings {
    fn default() -> Self {
        DebuggingSettings {
            live_scraper: DebuggingLiveScraperSettings::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DebuggingLiveScraperSettings {
    pub entries: Vec<ItemEntry>,
    pub fake_orders: bool,
}

impl Default for DebuggingLiveScraperSettings {
    fn default() -> Self {
        DebuggingLiveScraperSettings {
            entries: Vec::new(),
            fake_orders: false,
        }
    }
}
