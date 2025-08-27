use entity::wish_list::create::CreateWishListItem;
use qf_api::errors::ApiError as QFRequestError;
use serde_json::json;
use utils::{get_location, Error, LogLevel};
use wf_market::errors::ApiError as WFRequestError;

use crate::{
    enums::{find_by, FindBy, FindByType},
    utils::modules::states,
};

/// Extension trait for creating Error instances from different error types
pub trait CreateWishListItemExt {
    /// Validate the wish list item creation data
    fn validate(&mut self, find_by_type: FindByType) -> Result<(), Error>;
}

impl CreateWishListItemExt for CreateWishListItem {
    fn validate(&mut self, find_by_type: FindByType) -> Result<(), Error> {
        let cache = states::cache_client()?;
        let find_by = FindBy::new(find_by_type, self.raw.clone());

        let item = cache.tradable_item().get_by(find_by.clone())?;

        if item.is_none() {
            return Err(Error::new(
                "CreateWishListItem:Validate",
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
