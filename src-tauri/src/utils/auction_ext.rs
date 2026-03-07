use entity::stock_riven::{CreateStockRiven, RivenAttribute};
use qf_api::endpoints::riven;
use serde_json::json;
use utils::{get_location, Error};
use wf_market::{
    enums::AuctionType,
    types::{Auction, ItemAttribute},
};

use crate::{cache::client::CacheState, types::ItemRivenBase, utils::modules::states};
// Extension trait for auction
pub trait AuctionExt {
    fn apply_item_info(&mut self, cache: &CacheState) -> Result<(), Error>;
    fn to_create(&self) -> Result<CreateStockRiven, Error>;
}

impl AuctionExt for Auction {
    fn apply_item_info(&mut self, cache: &CacheState) -> Result<(), Error> {
        match self.item.item_type {
            AuctionType::Riven => {
                self.apply_uuid();
                let riven = ItemRivenBase::try_from_auction(&self, cache)?;
                self.properties
                    .set_property_value("name", json!(riven.name));
                self.properties
                    .set_property_value("mod_name", json!(riven.mod_name));
                self.item.attributes = Some(vec![]);
                for attr in riven.attributes {
                    let mut wf_attr =
                        ItemAttribute::new(attr.url_name.clone(), attr.positive, attr.value);
                    wf_attr
                        .properties
                        .set_property_value("localized_text", json!(attr.localized_text));
                    self.item.attributes.as_mut().unwrap().push(wf_attr);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn to_create(&self) -> Result<CreateStockRiven, Error> {
        if self.item.item_type != AuctionType::Riven {
            return Err(Error::new(
                "ToCreateStockRiven",
                "Cant only create stock riven from riven auction",
                get_location!(),
            ));
        }
        let cache = states::cache_client()?;

        let mut attributes: Vec<RivenAttribute> = vec![];
        for attr in self.item.attributes.as_deref().unwrap_or(&[]) {
            let c_attr = cache.riven().get_attribute_by(&attr.url_name)?;
            attributes.push(RivenAttribute::new(
                attr.positive,
                attr.value,
                attr.url_name.clone(),
                c_attr.full.clone(),
            ));
        }

        let item = CreateStockRiven::new(
            self.item.weapon_url_name.clone(),
            self.item.mod_name.clone().unwrap_or(String::new()),
            self.item.mastery_level.unwrap_or(0).into(),
            self.item.re_rolls.unwrap_or(0).into(),
            self.item.polarity.clone().unwrap_or(String::new()),
            attributes,
            self.item.mod_rank.unwrap_or(0).into(),
        );
        Ok(item)
    }
}
