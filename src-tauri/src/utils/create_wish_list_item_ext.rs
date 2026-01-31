use entity::wish_list::create::CreateWishListItem;
use utils::{get_location, Error, LogLevel};

use crate::utils::modules::states;

/// Extension trait for creating Error instances from different error types
pub trait CreateWishListItemExt {
    /// Validate the wish list item creation data
    fn validate(&mut self) -> Result<(), Error>;
}

impl CreateWishListItemExt for CreateWishListItem {
    fn validate(&mut self) -> Result<(), Error> {
        let cache = states::cache_client()?;

        let item = cache.tradable_item().get_by(&self.raw).map_err(|e| {
            e.with_location(get_location!())
                .set_log_level(LogLevel::Warning)
        })?;
        self.wfm_id = item.wfm_id.clone();
        self.wfm_url = item.wfm_url_name.clone();
        self.item_name = item.name.clone();
        self.item_unique_name = item.unique_name.clone();
        self.tags = item.tags.clone();
        self.credits = item.trade_tax * self.quantity;
        self.is_validated = true;
        Ok(())
    }
}
