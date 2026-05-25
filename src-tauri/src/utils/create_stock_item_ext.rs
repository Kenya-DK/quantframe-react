use entity::stock_item::CreateStockItem;
use utils::{get_location, Error};

use crate::utils::modules::states;

/// Extension trait for validating and enriching stock item creation payloads.
pub trait CreateStockItemExt {
    /// Validate the stock item creation data
    fn validate(&mut self) -> Result<(), Error>;
}

impl CreateStockItemExt for CreateStockItem {
    fn validate(&mut self) -> Result<(), Error> {
        let cache = states::cache_client()?;
        let item = cache
            .tradable_item()
            .get_by(&self.raw)
            .map_err(|e| e.with_location(get_location!()))?;

        self.wfm_id = item.wfm_id.clone();
        self.wfm_url = item.wfm_url.clone();
        self.item_name = item.name.clone();
        self.item_unique_name = item.unique_name.clone();
        if !item.variant_to_unique_name.is_empty() {
            if let Some(sub_type) = self.sub_type.as_ref() {
                let variant_key = sub_type.variant.clone().unwrap_or_default();
                if let Some(unique_name) = item.variant_to_unique_name.get(&variant_key) {
                    self.item_unique_name = unique_name.clone();
                } else {
                    return Err(Error::new(
                        "CreateStockItemExt:SubTypeVariant",
                        format!("No unique name found for sub_type: {:?}", self.sub_type),
                        get_location!(),
                    ));
                }
            }
        }

        self.tags = item.tags.clone();
        self.credits = item.trade_tax * self.quantity;
        self.is_validated = true;
        Ok(())
    }
}
