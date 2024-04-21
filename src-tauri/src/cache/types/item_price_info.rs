use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ItemPriceInfo {
    #[serde(rename = "url_name")]
    pub url_name: String,

    #[serde(rename = "item_id")]
    pub item_id: String,

    #[serde(rename = "order_type")]
    pub order_type: String,

    #[serde(rename = "volume")]
    pub volume: f64,

    #[serde(rename = "max_price")]
    pub max_price: f64,

    #[serde(rename = "min_price")]
    pub min_price: f64,

    #[serde(rename = "avg_price")]
    pub avg_price: f64,

    #[serde(rename = "moving_avg")]
    pub moving_avg: Option<f64>,

    #[serde(rename = "mod_rank")]
    pub mod_rank: Option<i64>,

    #[serde(rename = "median")]
    pub median: f64,

    #[serde(rename = "range")]
    pub range: f64,

    #[serde(rename = "week_price_shift")]
    pub week_price_shift: f64,
}


#[derive(Serialize, Deserialize)]
pub struct Settings {
    #[serde(rename = "debug")]
    pub debug: Vec<Option<serde_json::Value>>,

    #[serde(rename = "dev_mode")]
    pub dev_mode: bool,

    #[serde(rename = "live_scraper")]
    pub live_scraper: LiveScraper,

    #[serde(rename = "notifications")]
    pub notifications: Notifications,
}

#[derive(Serialize, Deserialize)]
pub struct LiveScraper {
    #[serde(rename = "stock_mode")]
    pub stock_mode: String,

    #[serde(rename = "webhook")]
    pub webhook: String,

    #[serde(rename = "stock_item")]
    pub stock_item: StockItem,

    #[serde(rename = "stock_riven")]
    pub stock_riven: StockRiven,
}

#[derive(Serialize, Deserialize)]
pub struct StockItem {
    #[serde(rename = "volume_threshold")]
    pub volume_threshold: i64,

    #[serde(rename = "range_threshold")]
    pub range_threshold: i64,

    #[serde(rename = "avg_price_cap")]
    pub avg_price_cap: i64,

    #[serde(rename = "max_total_price_cap")]
    pub max_total_price_cap: i64,

    #[serde(rename = "price_shift_threshold")]
    pub price_shift_threshold: i64,

    #[serde(rename = "blacklist")]
    pub blacklist: Vec<String>,

    #[serde(rename = "whitelist")]
    pub whitelist: Vec<String>,

    #[serde(rename = "report_to_wfm")]
    pub report_to_wfm: bool,

    #[serde(rename = "auto_trade")]
    pub auto_trade: bool,

    #[serde(rename = "strict_whitelist")]
    pub strict_whitelist: bool,

    #[serde(rename = "min_sma")]
    pub min_sma: i64,

    #[serde(rename = "auto_delete")]
    pub auto_delete: bool,

    #[serde(rename = "order_mode")]
    pub order_mode: String,
}

#[derive(Serialize, Deserialize)]
pub struct StockRiven {
    #[serde(rename = "range_threshold")]
    pub range_threshold: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Notifications {
    #[serde(rename = "on_new_conversation")]
    pub on_new_conversation: On,

    #[serde(rename = "on_wfm_chat_message")]
    pub on_wfm_chat_message: On,
}

#[derive(Serialize, Deserialize)]
pub struct On {
    #[serde(rename = "discord_notify")]
    pub discord_notify: bool,

    #[serde(rename = "system_notify")]
    pub system_notify: bool,

    #[serde(rename = "content")]
    pub content: String,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "webhook")]
    pub webhook: String,

    #[serde(rename = "user_ids")]
    pub user_ids: Vec<String>,
}