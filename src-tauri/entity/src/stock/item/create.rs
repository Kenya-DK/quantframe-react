use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::attribute::RivenAttribute;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateStockItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bought")]
    pub bought: Option<i64>,

    #[serde(default = "String::default")]
    #[serde(rename = "wfm_id")]
    pub wfm_id: String,

    #[serde(rename = "wfm_url")]
    pub wfm_url: String,
    
    #[serde(rename = "item_name")]
    pub item_name: String,

    #[serde(rename = "item_unique_name")]
    pub item_unique_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minimum_price")]
    pub minimum_price: Option<i64>,
    
    #[serde(rename = "is_hidden")]
    pub is_hidden: bool
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<SubType>,
}

impl CreateStockItem {
    pub fn to_stock(&self) -> super::stock_item::Model {
        super::stock_item::Model::new(
            self.wfm_id.clone(), 
            self.wfm_url.clone(),
            self.item_name.clone(),
            self.item_unique_name.clone(),
            self.sub_type.clone(),
            self.bought.unwrap_or(0),
            self.minimum_price,
            self.quantity.clone())
    }
}