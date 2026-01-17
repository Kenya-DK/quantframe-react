use entity::stock::riven::attribute::RivenAttribute;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuctionItem {
    #[serde(rename = "type")]
    pub item_type: String,

    #[serde(rename = "weapon_url_name")]
    pub weapon_url_name: Option<String>,

    // Rivens
    #[serde(rename = "re_rolls")]
    pub re_rolls: Option<i64>,

    #[serde(rename = "attributes")]
    pub attributes: Option<Vec<RivenAttribute>>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "mod_rank")]
    pub mod_rank: Option<i64>,

    #[serde(rename = "polarity")]
    pub polarity: Option<String>,

    #[serde(rename = "mastery_level")]
    pub mastery_level: Option<i64>,

    #[serde(rename = "similarity")]
    pub similarity: Option<f64>,

    #[serde(rename = "extra_attributes")]
    pub extra_attributes: Option<Vec<RivenAttribute>>,

    #[serde(rename = "missing_attributes")]
    pub missing_attributes: Option<Vec<RivenAttribute>>,

    // Kuva Lich And Sisters of Parvos
    #[serde(rename = "element")]
    pub element: Option<String>,

    #[serde(rename = "quirk")]
    pub quirk: Option<String>,

    #[serde(rename = "having_ephemera")]
    pub having_ephemera: Option<bool>,

    #[serde(rename = "damage")]
    pub damage: Option<i64>,
}

impl Default for AuctionItem {
    fn default() -> Self {
        AuctionItem {
            item_type: "".to_string(),
            weapon_url_name: None,
            re_rolls: None,
            attributes: None,
            name: None,
            mod_rank: None,
            polarity: None,
            mastery_level: None,
            similarity: None,
            extra_attributes: None,
            missing_attributes: None,
            element: None,
            quirk: None,
            having_ephemera: None,
            damage: None,
        }
    }
}

impl AuctionItem {
    pub fn from_stock_riven(stock_riven: entity::stock::riven::stock_riven::Model) -> Self {
        AuctionItem {
            item_type: "riven".to_string(),
            weapon_url_name: Some(stock_riven.wfm_weapon_url.clone()),
            re_rolls: Some(stock_riven.re_rolls as i64),
            attributes: Some(stock_riven.attributes.0.clone()),
            name: Some(stock_riven.mod_name.clone()),
            mod_rank: Some(stock_riven.sub_type.clone().unwrap().rank.unwrap_or(0)),
            polarity: Some(stock_riven.polarity.clone()),
            mastery_level: Some(stock_riven.mastery_rank as i64),
            element: None,
            quirk: None,
            having_ephemera: None,
            damage: None,
            similarity: None,
            extra_attributes: None,
            missing_attributes: None,
        }
    }
}
