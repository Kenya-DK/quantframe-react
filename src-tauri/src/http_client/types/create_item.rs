use entity::stock::riven::attribute::RivenAttribute;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemPayload {
    #[serde(rename = "by")]
    pub by: String,

    #[serde(rename = "item_data")]
    pub item_data: CreateStockItem,
}