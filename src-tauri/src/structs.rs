use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RivenTypeInfo {
    #[serde(rename = "thumb")]
    pub thumb: String,

    #[serde(rename = "url_name")]
    pub url_name: String,

    #[serde(rename = "mastery_level")]
    pub mastery_level: i64,

    #[serde(rename = "group")]
    pub group: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "item_name")]
    pub item_name: String,

    #[serde(rename = "icon_format")]
    pub icon_format: Option<String>,

    #[serde(rename = "riven_type")]
    pub riven_type: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RivenAttributeInfo {
    #[serde(rename = "negative_only")]
    negative_only: bool,

    #[serde(rename = "effect")]
    effect: String,

    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "exclusive_to")]
    exclusive_to: Option<Vec<String>>,

    #[serde(rename = "group")]
    group: String,

    #[serde(rename = "units")]
    units: Option<String>,

    #[serde(rename = "search_only")]
    search_only: bool,

    #[serde(rename = "url_name")]
    pub url_name: String,

    #[serde(rename = "suffix")]
    suffix: Option<String>,

    #[serde(rename = "positive_is_negative")]
    positive_is_negative: bool,

    #[serde(rename = "prefix")]
    prefix: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Item {
    pub item_name: String,
    pub id: String,
    pub url_name: String,
    pub thumb: String,
    pub set_items: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub mod_max_rank: Option<i64>,
    pub subtypes: Option<Vec<String>>,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ItemDetails {
    pub id: String,
    pub items_in_set: Vec<ItemInfo>,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ItemInfo {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "mod_max_rank")]
    pub mod_max_rank: Option<f64>,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Order {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "platinum")]
    pub platinum: i64,

    #[serde(rename = "visible")]
    pub visible: bool,

    #[serde(rename = "order_type")]
    pub order_type: String,

    #[serde(rename = "user")]
    pub user: Option<User>,

    #[serde(rename = "last_update")]
    pub last_update: String,

    #[serde(rename = "region")]
    pub region: String,

    #[serde(rename = "platform")]
    pub platform: String,

    #[serde(rename = "creation_date")]
    pub creation_date: String,

    #[serde(rename = "quantity")]
    pub quantity: i64,

    #[serde(rename = "mod_rank")]
    pub mod_rank: Option<i64>,

    #[serde(rename = "item")]
    pub item: Option<OrderItem>,
}
#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct OrderItem {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "url_name")]
    pub url_name: String,
    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "icon_format")]
    pub icon_format: Option<String>,

    #[serde(rename = "thumb")]
    pub thumb: String,

    #[serde(rename = "sub_icon")]
    pub sub_icon: Option<String>,

    #[serde(rename = "mod_max_rank")]
    pub mod_max_rank: Option<i64>,

    #[serde(rename = "subtypes")]
    pub subtypes: Option<Vec<String>>,

    #[serde(rename = "tags")]
    pub tags: Vec<String>,

    #[serde(rename = "ducats")]
    pub ducats: Option<i64>,

    #[serde(rename = "quantity_for_set")]
    pub quantity_for_set: Option<i64>,

    #[serde(rename = "en")]
    pub en: OrderItemTranslation,
}
#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct OrderItemTranslation {
    #[serde(rename = "item_name")]
    item_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Ordres {
    #[serde(rename = "sell_orders")]
    pub sell_orders: Vec<Order>,
    #[serde(rename = "buy_orders")]
    pub buy_orders: Vec<Order>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Invantory {
    pub id: i64,
    pub item_id: String,
    pub item_url: String,
    pub item_name: String,
    pub item_type: String,
    pub rank: i64,
    pub price: f64,
    pub listed_price: Option<i64>,
    pub owned: i64,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InvantoryCreateOrUpdate {
    pub id: i64,
    pub item_id: Option<String>,
    pub item_url: Option<String>,
    pub item_name: Option<String>,
    pub item_type: Option<String>,
    pub rank: Option<i64>,
    pub price: Option<f64>,
    pub listed_price: Option<i64>,
    pub owned: Option<i64>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub id: i64,
    pub item_name: String,
    pub item_id: String,
    pub item_url: String,
    pub item_type: String,
    pub item_tags: String,
    pub rank: i64,
    pub price: i64,
    pub datetime: String,
    pub transaction_type: String,
    pub quantity: i64,
}

#[derive(sqlx::Decode, Serialize, Deserialize, Clone, Debug)]
pub struct RivenAttribute {
    #[serde(rename = "positive")]
    positive: bool,

    #[serde(rename = "value")]
    value: f64,

    #[serde(rename = "url_name")]
    url_name: String,
}

/// Generated by https://quicktype.io
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderByItem {
    #[serde(rename = "order_type")]
    pub order_type: String,

    #[serde(rename = "quantity")]
    pub quantity: i64,

    #[serde(rename = "platinum")]
    pub platinum: i64,

    #[serde(rename = "mod_rank")]
    pub mod_rank: Option<i64>,

    #[serde(rename = "user")]
    pub user: User,
    #[serde(rename = "platform")]
    pub platform: String,

    #[serde(rename = "creation_date")]
    pub creation_date: String,

    #[serde(rename = "last_update")]
    pub last_update: String,

    #[serde(rename = "visible")]
    pub visible: bool,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "region")]
    pub region: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "reputation")]
    pub reputation: i64,

    // #[serde(rename = "locale")]
    // pub locale: String,

    // #[serde(rename = "avatar")]
    // pub avatar: String,

    // #[serde(rename = "last_seen")]
    // pub last_seen: String,
    #[serde(rename = "ingame_name")]
    pub ingame_name: String,

    #[serde(rename = "id")]
    pub id: String,
    // #[serde(rename = "region")]
    // pub region: String,
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Auction<T> {
    #[serde(rename = "visible")]
    pub visible: bool,

    #[serde(rename = "minimal_reputation")]
    pub minimal_reputation: i64,

    #[serde(rename = "item")]
    pub item: AuctionItem,

    #[serde(rename = "buyout_price")]
    pub buyout_price: i64,

    #[serde(rename = "note")]
    pub note: String,

    #[serde(rename = "starting_price")]
    pub starting_price: i64,

    #[serde(rename = "owner")]
    pub owner: T,

    #[serde(rename = "platform")]
    pub platform: String,

    #[serde(rename = "closed")]
    pub closed: bool,

    #[serde(rename = "top_bid")]
    pub top_bid: Option<serde_json::Value>,

    #[serde(rename = "winner")]
    pub winner: Option<serde_json::Value>,

    #[serde(rename = "is_marked_for")]
    pub is_marked_for: Option<serde_json::Value>,

    #[serde(rename = "marked_operation_at")]
    pub marked_operation_at: Option<serde_json::Value>,

    #[serde(rename = "created")]
    pub created: String,

    #[serde(rename = "updated")]
    pub updated: String,

    #[serde(rename = "note_raw")]
    pub note_raw: String,

    #[serde(rename = "is_direct_sell")]
    pub is_direct_sell: bool,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "private")]
    pub private: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuctionItem {
    // #[serde(rename = "type")]
    // pub item_type: String,
    #[serde(rename = "weapon_url_name")]
    pub weapon_url_name: Option<String>,

    // Rivens
    #[serde(rename = "re_rolls")]
    pub re_rolls: Option<i64>,

    #[serde(rename = "attributes")]
    pub attributes: Option<Vec<RivenAttribute>>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "mod_rank")]
    pub mod_rank: Option<i64>,

    #[serde(rename = "polarity")]
    pub polarity: Option<String>,

    #[serde(rename = "mastery_level")]
    pub mastery_level: Option<i64>,

    // Kuva Lich And Sisters of Parvos
    #[serde(rename = "element")]
    pub element: Option<String>,

    #[serde(rename = "quirk")]
    pub quirk: Option<String>,

    #[serde(rename = "having_ephemera")]
    pub having_ephemera: Option<bool>,

    #[serde(rename = "damage")]
    pub damage: Option<i64>,

    #[serde(rename = "type")]
    pub top_level_type: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AuctionOwner {
    #[serde(rename = "ingame_name")]
    ingame_name: String,

    #[serde(rename = "last_seen")]
    last_seen: String,

    #[serde(rename = "reputation")]
    reputation: i64,

    #[serde(rename = "locale")]
    locale: String,

    #[serde(rename = "status")]
    status: String,

    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "region")]
    region: String,

    #[serde(rename = "avatar")]
    avatar: Option<String>,
}
