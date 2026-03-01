use entity::{dto::SubType, stock_riven::RivenAttribute};
use serde::{Deserialize, Serialize};
use utils::{generate_uuid_from_list, get_location, Error, Properties};
use wf_market::{enums::AuctionType, types::Auction};

use crate::cache::CacheState;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemRivenBase {
    #[serde(rename = "name", default)]
    pub name: String,

    #[serde(rename = "unique_name", default)]
    pub unique_name: String,

    #[serde(rename = "wfm_url", default)]
    pub wfm_url: String,

    #[serde(rename = "sub_type", skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<entity::dto::SubType>,

    #[serde(rename = "mod_name", default)]
    pub mod_name: String,

    #[serde(rename = "mastery_rank", default)]
    pub mastery_rank: i64,

    #[serde(rename = "re_rolls", default)]
    pub re_rolls: i64,

    #[serde(rename = "polarity", default)]
    pub polarity: String,

    #[serde(rename = "attributes", default)]
    pub attributes: Vec<RivenAttribute>,

    #[serde(rename = "uuid", default)]
    pub uuid: String,

    // Extra properties that may be needed for specific item types, stored as a JSON object
    #[serde(flatten)]
    pub properties: Properties,
}
impl ItemRivenBase {
    pub fn update_uuid(&mut self) -> (String, String) {
        let mut input = String::new();

        input.push_str(&format!("type:{};", "0"));
        input.push_str(&format!("weapon:{};", self.wfm_url));

        input.push_str(&format!("mod_name:{};", self.mod_name.to_lowercase()));
        input.push_str(&format!("re_rolls:{};", self.re_rolls));
        input.push_str(&format!("mastery:{};", self.mastery_rank));
        input.push_str(&format!(
            "mod_rank:{};",
            self.sub_type
                .clone()
                .unwrap_or_default()
                .rank
                .unwrap_or_default()
        ));
        input.push_str(&format!("polarity:{};", self.polarity.to_lowercase()));

        let mut sorted_attrs = self.attributes.clone();
        sorted_attrs.sort_by_key(|a| a.url_name.clone());
        for a in sorted_attrs {
            input.push_str(&format!("attr:{}:{}:{};", a.url_name, a.positive, a.value));
        }
        let uuid = generate_uuid_from_list(&[input]);
        self.uuid = uuid.0.clone();
        (uuid.0, uuid.1)
    }
    pub fn matches_query(&self, query: &str) -> bool {
        let query = query.to_lowercase();
        self.name.to_lowercase().contains(&query) || self.mod_name.to_lowercase().contains(&query)
    }
    pub fn try_from_auction(auction: &Auction, cache: &CacheState) -> Result<Self, Error> {
        if auction.item.item_type != AuctionType::Riven {
            return Err(Error::new(
                "TryFromAuction",
                "Can only create ItemRivenBase from Riven auction",
                get_location!(),
            ));
        }
        let mut riven = Self::default();

        if let Ok(weapon) = cache.riven().get_weapon_by(&auction.item.weapon_url_name) {
            riven.name = weapon.name.clone();
            riven.unique_name = weapon.unique_name.clone();
            riven.wfm_url = weapon.wfm_url_name.clone();
        }
        riven.sub_type = Some(SubType::rank(auction.item.mod_rank.unwrap_or(0) as i64));
        riven.mod_name = auction.item.mod_name.clone().unwrap_or(String::new());
        riven.mastery_rank = auction.item.mastery_level.unwrap_or(0) as i64;
        riven.re_rolls = auction.item.re_rolls.unwrap_or(0) as i64;
        riven.polarity = auction.item.polarity.clone().unwrap_or(String::new());
        riven.uuid = auction.uuid.clone();
        for attr in auction.item.attributes.as_deref().unwrap_or(&[]) {
            let localized_text = match cache.riven().get_attribute_by(&attr.url_name) {
                Ok(c_attr) => c_attr.full.clone(),
                Err(_) => attr.url_name.clone(), // Fallback to URL name if not found in cache
            };
            riven.attributes.push(RivenAttribute::new(
                attr.positive,
                attr.value,
                attr.url_name.clone(),
                localized_text,
            ));
        }
        Ok(riven)
    }
    pub fn try_from_stock_riven(item: &entity::stock_riven::Model) -> Result<Self, Error> {
        let mut riven = Self::default();
        riven.name = item.weapon_name.clone();
        riven.unique_name = item.weapon_unique_name.clone();
        riven.wfm_url = item.wfm_weapon_url.clone();
        riven.sub_type = item.sub_type.clone();
        riven.mod_name = item.mod_name.clone();
        riven.mastery_rank = item.mastery_rank;
        riven.re_rolls = item.re_rolls;
        riven.polarity = item.polarity.clone();
        riven.uuid = item.uuid.clone();
        riven.attributes = item.attributes.0.clone();
        Ok(riven)
    }
}

impl Default for ItemRivenBase {
    fn default() -> Self {
        Self {
            name: String::new(),
            unique_name: String::new(),
            wfm_url: String::new(),
            sub_type: None,
            mod_name: String::new(),
            mastery_rank: 0,
            re_rolls: 0,
            polarity: String::new(),
            attributes: Vec::new(),
            properties: Properties::default(),
            uuid: String::new(),
        }
    }
}
