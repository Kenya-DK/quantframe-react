use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PlayerTrade {
    #[serde(rename = "playerName")]
    pub player_name: String,
    #[serde(rename = "tradeTime")]
    pub trade_time: String,
    #[serde(rename = "type")]
    pub trade_type: String,
    #[serde(rename = "platinum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platinum: Option<i64>,
    #[serde(rename = "offeredItems")]
    pub offered_items: Vec<TradeItem>,
    #[serde(rename = "receivedItems")]
    pub received_items: Vec<TradeItem>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TradeItem {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "quantity")]
    pub quantity: i64,

    #[serde(rename = "unique_name")]
    #[serde(default)]
    pub unique_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rank")]
    pub rank: Option<i64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "wfm_id")]
    pub wfm_id: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "wfm_url")]
    pub wfm_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "error")]
    pub error: Option<(String, Value)>,
}
