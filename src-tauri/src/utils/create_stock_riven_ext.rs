use entity::stock_riven::CreateStockRiven;
use serde_json::json;
use utils::{get_location, Error, LogLevel};

use crate::{
    enums::{FindBy, FindByType},
    utils::modules::states,
};

/// Extension trait for creating Error instances from different error types
pub trait CreateStockRivenExt {
    /// Validate the stock item creation data
    fn validate(&mut self, find_by_type: FindByType) -> Result<(), Error>;
}

impl CreateStockRivenExt for CreateStockRiven {
    fn validate(&mut self, find_by_type: FindByType) -> Result<(), Error> {
        let cache = states::cache_client()?;
        let find_by = FindBy::new(find_by_type, self.raw.clone());

        let weapon = cache.riven().get_riven_by(find_by.clone())?;

        if weapon.is_none() {
            return Err(Error::new(
                "CreateStockRiven:Validate",
                format!("Riven item {} ", find_by),
                get_location!(),
            )
            .with_context(json!(find_by))
            .set_log_level(LogLevel::Warning));
        }

        let weapon = weapon.unwrap();

        self.wfm_id = weapon.wfm_id.clone();
        self.wfm_url = weapon.wfm_url_name.clone();
        self.weapon_type = weapon.wfm_group.clone();
        self.weapon_unique_name = weapon.unique_name.clone();
        self.weapon_name = weapon.name.clone();
        for att in self.attributes.iter_mut() {
            let att_find_by = FindBy::new(FindByType::Url, att.url_name.clone());
            match cache.riven().get_riven_attribute_by(att_find_by.clone()) {
                Ok(attribute) => {
                    if attribute.is_none() {
                        return Err(Error::new(
                            "CreateStockRiven:Attribute:Validate",
                            format!("Riven attribute {} not found", att_find_by),
                            get_location!(),
                        )
                        .with_context(json!(att_find_by))
                        .set_log_level(LogLevel::Warning));
                    }
                    let attribute = attribute.unwrap();
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
