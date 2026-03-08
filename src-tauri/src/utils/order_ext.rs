use utils::{warning, Error, LoggerOptions};
use wf_market::types::Order;

use crate::cache::client::CacheState;

// Extension trait for order
pub trait OrderExt {
    fn apply_info(&mut self, cache: &CacheState) -> Result<(), Error>;
}

impl OrderExt for Order {
    fn apply_info(&mut self, cache: &CacheState) -> Result<(), Error> {
        match cache.tradable_item().get_by(&self.item_id) {
            Ok(item_info) => {
                self.properties
                    .set_property_value("wfm_id", item_info.wfm_id.clone());
                self.properties
                    .set_property_value("wfm_url", item_info.wfm_url_name.clone());
                self.properties
                    .set_property_value("name", item_info.name.clone());
                self.properties
                    .set_property_value("image", item_info.image_url.clone());
                self.properties
                    .set_property_value("t_type", item_info.sub_type.clone());
            }
            Err(_) => {
                warning(
                    "Order",
                    format!(
                        "Failed to apply item info for Order ID: {} with Item ID: {}",
                        self.id, self.item_id
                    ),
                    &LoggerOptions::default(),
                );
            }
        }
        Ok(())
    }
}
