use serde::Deserialize;

pub struct Response<T> {
    pub data: T,
}
#[derive(Deserialize, Debug)]
pub struct ResponseWFMPayload {
    pub payload: Payload,
}

#[derive(Deserialize, Debug)]
pub struct Payload {
    pub items: Vec<Item>,
}

#[derive(Deserialize, Debug)]
pub struct Item {
    pub item_name: String,
    pub id: String,
    pub url_name: String,
    pub thumb: String,
}
#[derive(Deserialize, Debug)]
pub struct Order {
    #[serde(rename = "id")]
    pub id: String,
  
    #[serde(rename = "platinum")]
    pub platinum: i64,
  
    #[serde(rename = "quantity")]
    pub quantity: i64,
  
    #[serde(rename = "order_type")]
    pub order_type: String,
  
    #[serde(rename = "platform")]
    pub platform: String,
  
    #[serde(rename = "region")]
    pub region: String,
  
    #[serde(rename = "creation_date")]
    pub creation_date: String,
  
    #[serde(rename = "last_update")]
    pub last_update: String,
  
    #[serde(rename = "subtype")]
    pub  subtype: String,
  
    #[serde(rename = "visible")]
    pub  visible: bool,
  
    #[serde(rename = "item")]
    pub  item: OrderItem,
}
#[derive(Serialize, Deserialize)]
pub struct OrderItem {
  #[serde(rename = "id")]
  pub id: String,

  #[serde(rename = "url_name")]
  pub url_name: String,

  #[serde(rename = "icon")]
  pub  icon: String,

  #[serde(rename = "icon_format")]
  pub  icon_format: String,

  #[serde(rename = "thumb")]
  pub  thumb: String,

  #[serde(rename = "sub_icon")]
  pub sub_icon: String,

  #[serde(rename = "mod_max_rank")]
  pub mod_max_rank: i64,

  #[serde(rename = "subtypes")]
  pub subtypes: Vec<String>,

  #[serde(rename = "tags")]
  pub tags: Vec<String>,

  #[serde(rename = "ducats")]
  pub ducats: i64,

  #[serde(rename = "quantity_for_set")]
  pub quantity_for_set: i64,

  #[serde(rename = "en")]
  pub en: OrderItemTranslation,
}
#[derive(Serialize, Deserialize)]
pub struct OrderItemTranslation {
  #[serde(rename = "item_name")]
  item_name: String,
}

#[derive(Deserialize, Clone)]
pub struct Ordres {
    #[serde(rename = "sell_orders")]
    pub sell_orders: Vec<Order>,
    #[serde(rename = "buy_orders")]
    pub buy_orders: Vec<Order>
}
#[derive(Deserialize, Clone)]
pub struct Settings {
    pub field1: String,
    pub field2: i32,
    // more fields...
}

pub struct Invantory {
    pub id: i32,
    pub item_id: String,
    pub item_url: String,
    pub item_name: String,
    pub rank: i32,
    pub price: i32,
    pub listed_price: i32,
    pub owned: i32,
}

#[derive(Debug, Deserialize)]
pub struct PriceHistoryDto {
    pub name: String,
    pub datetime: String,
    pub order_type: String,
    pub volume: u32,
    pub min_price: u32,
    pub max_price: u32,
    pub range: u32,
    pub median: f64,
    pub avg_price: f64,
    pub mod_rank: Option<u32>,
    pub item_id: String,
}
