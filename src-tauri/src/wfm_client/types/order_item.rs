use serde::{Deserialize, Serialize};

use super::order_item_translation::OrderItemTranslation;

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct OrderItem {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "url_name")]
    pub url_name: String,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "icon_format")]
    pub icon_format: Option<String>,

    #[serde(rename = "thumb")]
    pub thumb: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sub_icon")]
    pub sub_icon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mod_max_rank")]
    pub mod_max_rank: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subtypes")]
    pub subtypes: Option<Vec<String>>,

    #[serde(rename = "tags")]
    pub tags: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ducats")]
    pub ducats: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "quantity_for_set")]
    pub quantity_for_set: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vaulted")]
    pub vaulted: Option<bool>,

    #[serde(rename = "en")]
    pub en: OrderItemTranslation,
}
