use serde::{Deserialize, Serialize};

use super::auction_item::AuctionItem;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auction<T> {
    #[serde(rename = "visible")]
    pub visible: bool,

    #[serde(rename = "minimal_reputation")]
    pub minimal_reputation: i64,

    #[serde(rename = "item")]
    pub item: AuctionItem,

    #[serde(rename = "buyout_price")]
    pub buyout_price: Option<i64>,

    #[serde(rename = "note")]
    pub note: String,

    #[serde(rename = "starting_price")]
    pub starting_price: i64,

    #[serde(rename = "owner")]
    pub owner: T,

    // #[serde(rename = "platform")]
    // pub platform: String,
    #[serde(rename = "closed")]
    pub closed: bool,

    // #[serde(rename = "top_bid")]
    // pub top_bid: Option<serde_json::Value>,

    // #[serde(rename = "winner")]
    // pub winner: Option<serde_json::Value>,

    // #[serde(rename = "is_marked_for")]
    // pub is_marked_for: Option<serde_json::Value>,

    // #[serde(rename = "marked_operation_at")]
    // pub marked_operation_at: Option<serde_json::Value>,

    // #[serde(rename = "created")]
    // pub created: String,

    // #[serde(rename = "updated")]
    // pub updated: String,

    // #[serde(rename = "note_raw")]
    // pub note_raw: String,
    #[serde(rename = "is_direct_sell")]
    pub is_direct_sell: bool,

    #[serde(rename = "id")]
    pub id: String,
    // #[serde(rename = "private")]
    // pub private: bool,
}
