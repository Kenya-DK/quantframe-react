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
            .weapon()
            .get_by(&self.raw)
            .map_err(|e| e.with_location(get_location!()))?;

        self.wfm_id = weapon.wfm_riven_id.clone();
        self.wfm_url = weapon.wfm_riven_url.clone();
        self.weapon_type = weapon.riven_type.clone();
        self.weapon_unique_name = weapon.unique_name.clone();
        self.weapon_name = weapon.name.clone();
        self.credits = crate::enums::TradeItemType::RivenVeiled.to_tax();
        for att in self.attributes.iter_mut() {
            match cache.attribute().get_by(&att.wfm_url) {
                Ok(attribute) => {
                    att.wfm_url = attribute.wfm_url.clone();
                    att.formatted_value = attribute.formatted_value.clone();
                }
                Err(e) => {
                    return Err(e.with_location(get_location!()));
                }
            }
        }
        self.is_validated = true;
        Ok(())
    }
}
