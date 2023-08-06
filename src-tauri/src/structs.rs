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
