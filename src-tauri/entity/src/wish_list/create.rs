use serde::{Deserialize, Serialize};

use crate::sub_type::SubType;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateWishListItem {
    #[serde(rename = "raw")]
    pub raw: String,

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

    #[serde(default)]
    #[serde(rename = "tags")]
    pub tags: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maximum_price")]
    pub maximum_price: Option<i64>,

    #[serde(rename = "quantity")]
    pub quantity: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<SubType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_sub_types: Option<SubType>,

    #[serde(rename = "is_validated")]
    #[serde(default = "bool::default")]
    pub is_validated: bool,
}

impl CreateWishListItem {
    pub fn new(
        raw: String,
        sub_type: Option<SubType>,
        maximum_price: Option<i64>,
        quantity: i64,
    ) -> Self {
        CreateWishListItem {
            raw,
            wfm_id: "".to_string(),
            wfm_url: "".to_string(),
            item_name: "".to_string(),
            item_unique_name: "".to_string(),
            tags: vec![],
            maximum_price,
            bought: None,
            quantity,
            sub_type,
            available_sub_types: None,
            is_validated: false,
        }
    }
    pub fn new_valid(
        wfm_id: String,
        wfm_url: String,
        item_name: String,
        item_unique_name: String,
        tags: Vec<String>,
        sub_type: Option<SubType>,
        maximum_price: Option<i64>,
        quantity: i64,
        bought: Option<i64>,
    ) -> Self {
        CreateWishListItem {
            raw: "".to_string(),
            wfm_id,
            bought,
            wfm_url,
            item_name,
            item_unique_name,
            tags,
            sub_type,
            quantity,
            maximum_price,
            available_sub_types: None,
            is_validated: true,
        }
    }
    pub fn to_model(&self) -> super::wish_list::Model {
        super::wish_list::Model::new(
            self.wfm_id.clone(),
            self.wfm_url.clone(),
            self.item_name.clone(),
            self.item_unique_name.clone(),
            self.sub_type.clone(),
            self.maximum_price.clone(),
            self.quantity.clone(),
        )
    }
}
