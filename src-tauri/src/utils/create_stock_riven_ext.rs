use entity::stock_riven::CreateStockRiven;
use utils::{get_location, Error};

use crate::utils::modules::states;

/// Extension trait for creating Error instances from different error types
pub trait CreateStockRivenExt {
    /// Validate the stock item creation data
    fn validate(&mut self) -> Result<(), Error>;
}

impl CreateStockRivenExt for CreateStockRiven {
    fn validate(&mut self) -> Result<(), Error> {
        let cache = states::cache_client()?;

        let weapon = cache
            .riven()
            .get_weapon_by(&self.raw)
            .map_err(|e| e.with_location(get_location!()))?;

        self.wfm_id = weapon.wfm_id.clone();
        self.wfm_url = weapon.wfm_url_name.clone();
        self.weapon_type = weapon.wfm_group.clone();
        self.weapon_unique_name = weapon.unique_name.clone();
        self.weapon_name = weapon.name.clone();
        self.credits = crate::enums::TradeItemType::RivenVeiled.to_tax();
        for att in self.attributes.iter_mut() {
            match cache.riven().get_attribute_by(&att.url_name) {
                Ok(attribute) => {
                    att.url_name = attribute.url_name.clone();
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        self.is_validated = true;
        Ok(())
    }
}
