use serde::{Deserialize, Serialize};

use super::riven_attribute::RivenAttribute;

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
