use std::hash::{Hash, Hasher};

use entity::{stock::item::stock_item, sub_type::SubType};
use serde::{Deserialize, Serialize};

use crate::cache::types::item_price_info::ItemPriceInfo;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stock_id")]
    pub stock_id: Option<i64>,

    #[serde(rename = "wfm_url")]
    pub wfm_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sub_type")]
    pub sub_type: Option<SubType>,

    #[serde(default)]
    #[serde(rename = "priority")]
    pub priority: i64,
    
    #[serde(default)]
    #[serde(rename = "quantity")]
    pub quantity: i64,

}

impl Hash for ItemEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.wfm_url.hash(state);
        self.sub_type.hash(state);
    }
}

impl ItemEntry {
    pub fn display(&self) -> String {
        format!(
            "Stock ID: {:?}, WFM URL: {}, Sub Type: {:?}, Priority: {}, Quantity: {}",
            self.stock_id,
            self.wfm_url,
            self.sub_type
                .clone()
                .unwrap_or(SubType::default())
                .shot_display(),
            self.priority,
            self.quantity
        )
    }
    pub fn from_stock_item(stock_item: &stock_item::Model) -> ItemEntry {
        ItemEntry {
            stock_id: Some(stock_item.id),
            wfm_url: stock_item.wfm_url.to_owned(),
            sub_type: stock_item.sub_type.clone(),
            priority: 1,
            quantity: stock_item.owned,
        }
    }
    pub fn from_item_price(item_price: &ItemPriceInfo, quantity) -> ItemEntry {
        ItemEntry {
            stock_id: None,
            wfm_url: item_price.url_name.clone(),
            sub_type: item_price.sub_type.clone(),
            priority: 0,
            quantity
        }
    }
}
