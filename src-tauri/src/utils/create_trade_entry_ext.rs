use entity::trade_entry::create::CreateTradeEntry;
use serde_json::json;
use utils::{get_location, Error, LogLevel};

use crate::{
    enums::{FindBy, FindByType},
    utils::modules::states,
};

/// Extension trait for creating Error instances from different error types
pub trait CreateTradeEntryExt {
    /// Validate the trade entry creation data
    fn validate(&mut self, find_by_type: FindByType) -> Result<(), Error>;
}

impl CreateTradeEntryExt for CreateTradeEntry {
    fn validate(&mut self, find_by_type: FindByType) -> Result<(), Error> {
        let cache = states::cache_client()?;
        let find_by = FindBy::new(find_by_type, self.raw.clone());

        match self.group.as_str() {
            "item" => {
                let item = cache.tradable_item().get_by(find_by.clone())?;

                if item.is_none() {
                    return Err(Error::new(
                        "CreateTradeEntryExt::Validate",
                        format!("Tradable item {} ", find_by),
                        get_location!(),
                    )
                    .with_context(json!(find_by))
                    .set_log_level(LogLevel::Warning));
                }

                let item = item.unwrap();
                self.wfm_id = item.wfm_id.clone();
            }
            "riven" => {
                let item = cache.riven().get_riven_by(find_by.clone())?;

                if item.is_none() {
                    return Err(Error::new(
                        "CreateTradeEntryExt::Validate",
                        format!("Riven item {} ", find_by),
                        get_location!(),
                    )
                    .with_context(json!(find_by))
                    .set_log_level(LogLevel::Warning));
                }

                let item = item.unwrap();
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
