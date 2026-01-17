use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Item {
    pub item_name: String,
    pub id: String,
    pub url_name: String,
    pub thumb: String,
    pub wiki_url: Option<String>,
    pub trade_tax: Option<i64>,
    pub mr_requirement: Option<i64>,
    pub set_items: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub mod_max_rank: Option<i64>,
    pub subtypes: Option<Vec<String>>,
}
