use entity::trade_entry::create::CreateTradeEntry;
use serde_json::json;
use utils::{get_location, Error, LogLevel};

use crate::utils::modules::states;

/// Extension trait for creating Error instances from different error types
pub trait CreateTradeEntryExt {
    /// Validate the trade entry creation data
    fn validate(&mut self) -> Result<(), Error>;
}

impl CreateTradeEntryExt for CreateTradeEntry {
    fn validate(&mut self) -> Result<(), Error> {
        let cache = states::cache_client()?;
        match self.group.as_str() {
            "item" => {
                let item = cache.tradable_item().get_by(&self.raw).map_err(|e| {
                    e.with_location(get_location!())
                        .set_log_level(LogLevel::Warning)
                })?;
                self.wfm_id = item.wfm_id.clone();
            }
            "riven" => {
                let item = cache.riven().get_weapon_by(&self.raw).map_err(|e| {
                    e.with_location(get_location!())
                        .set_log_level(LogLevel::Warning)
                })?;
                self.wfm_id = item.wfm_id.clone();
            }
            "custom" => {
                self.wfm_id = "custom".to_string();
                self.name = self.raw.clone();
            }
            _ => {
                return Err(Error::new(
                    "CreateTradeEntryExt::Validate",
                    format!("Invalid group: {}", self.group),
                    get_location!(),
                )
                .with_context(json!({ "group": self.group.clone() }))
                .set_log_level(LogLevel::Warning));
            }
        }
        Ok(())
    }
}
