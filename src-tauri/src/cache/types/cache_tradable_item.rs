use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CacheTradableItem {
  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "uniqueName")]
  pub unique_name: String,

  #[serde(rename = "description")]
  pub description: String,

  #[serde(rename = "wfm_id")]
  pub wfm_id: String,

  #[serde(rename = "wfm_url_name")]
  pub wfm_url_name: String,

  #[serde(rename = "trade_tax")]
  pub trade_tax: i64,

  #[serde(rename = "mr_requirement")]
  pub mr_requirement: i64,

  #[serde(rename = "tags")]
  pub tags: Vec<String>,

  #[serde(rename = "wiki_url")]
  pub wiki_url: String,

  #[serde(rename = "image_url")]
  pub image_url: String,

  #[serde(rename = "max_rank")]
  pub max_rank: Option<i64>,

  #[serde(rename = "sub_type")]
  pub sub_type: Option<SubType>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SubType {
  #[serde(rename = "max_rank")]
  pub max_rank: Option<i64>,

  #[serde(rename = "variants")]
  pub variants: Vec<Option<String>>,

  #[serde(rename = "amber_stars")]
  pub amber_stars: Option<i64>,

  #[serde(rename = "cyan_stars")]
  pub cyan_stars: Option<i64>,

}