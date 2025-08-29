use entity::stock_item::CreateStockItem;
use serde_json::json;
use utils::{get_location, Error, LogLevel};

use crate::{
    enums::{FindBy, FindByType},
    utils::modules::states,
};

/// Extension trait for creating Error instances from different error types
pub trait CreateStockItemExt {
    /// Validate the stock item creation data
    fn validate(&mut self, find_by: FindBy) -> Result<(), Error>;
}

impl CreateStockItemExt for CreateStockItem {
    fn validate(&mut self, find_by: FindBy) -> Result<(), Error> {
        let cache = states::cache_client()?;
        let item = cache.tradable_item().get_by(find_by.clone())?;

        if item.is_none() {
            return Err(Error::new(
                "CreateStockItem:Validate",
                format!("Tradable item {} ", find_by),
                get_location!(),
            )
            .with_context(json!(find_by))
            .set_log_level(LogLevel::Warning));
        }

        let item = item.unwrap();
        self.wfm_id = item.wfm_id.clone();
        self.wfm_url = item.wfm_url_name.clone();
        self.item_name = item.name.clone();
        self.item_unique_name = item.unique_name.clone();
        self.tags = item.tags.clone();
        self.is_validated = true;
        Ok(())
    }
}
