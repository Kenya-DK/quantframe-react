use serde::{Deserialize, Serialize};

use crate::{
    enums::OrderType,
    error::AppError,
};

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum WarframeLanguage {
    English, // Add other language variants as needed
    French,
    Spanish,
    German,
    Russian,
    Unknown,
}

impl Default for WarframeLanguage {
    fn default() -> Self {
        WarframeLanguage::English
    }
}
impl WarframeLanguage {
    pub fn from_str(s: &str) -> Self {
        match s {
            "en" => WarframeLanguage::English,
            "fr" => WarframeLanguage::French,
            "es" => WarframeLanguage::Spanish,
            "de" => WarframeLanguage::German,
            "ru" => WarframeLanguage::Russian,
            _ => WarframeLanguage::Unknown,
        }
    }
}
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
    pub riven_type: Option<String>,
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
    pub wiki_url: Option<String>,
    pub trade_tax: Option<i64>,
    pub mr_requirement: Option<i64>,
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
    pub order_type: OrderType,

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

    #[serde(rename = "subtype")]
    pub subtype: Option<String>,

    #[serde(rename = "quantity")]
    pub quantity: i64,

    #[serde(rename = "mod_rank")]
    pub mod_rank: Option<i64>,

    #[serde(rename = "item")]
    pub item: Option<OrderItem>,

    #[serde(rename = "profit")]
    pub profit: Option<f64>,

    #[serde(rename = "closed_avg")]
    pub closed_avg: Option<f64>,
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

    #[serde(rename = "vaulted")]
    pub vaulted: Option<bool>,

    #[serde(rename = "en")]
    pub en: OrderItemTranslation,
}
#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct OrderItemTranslation {
    #[serde(rename = "item_name")]
    item_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Orders {
    #[serde(rename = "sell_orders")]
    pub sell_orders: Vec<Order>,
    #[serde(rename = "buy_orders")]
    pub buy_orders: Vec<Order>,
}
impl Orders {
    pub fn sort_by_platinum(&mut self) {
        self.sell_orders.sort_by(|a, b| a.platinum.cmp(&b.platinum));
        self.buy_orders.sort_by(|a, b| b.platinum.cmp(&a.platinum));
    }

    pub fn filter_by_username(&mut self, username: &str, exclude: bool)
    where
        Self: Sized,
    {
        self.sell_orders = self
            .sell_orders
            .iter()
            .filter(|order| {
                if exclude {
                    // And User is ingame_name
                    order.user.clone().map(|user| user.ingame_name.clone())
                        != Some(username.to_owned())
                } else {
                    order.user.clone().map(|user| user.ingame_name.clone())
                        == Some(username.to_owned())
                }
            })
            .cloned()
            .collect();
        self.buy_orders = self
            .buy_orders
            .iter()
            .filter(|order| {
                if exclude {
                    // And User is ingame_name
                    order.user.clone().map(|user| user.ingame_name.clone())
                        != Some(username.to_owned())
                } else {
                    order.user.clone().map(|user| user.ingame_name.clone())
                        == Some(username.to_owned())
                }
            })
            .cloned()
            .collect();
    }

    pub fn lowest_order(&self, order_type: OrderType) -> Option<Order> {
        let orders = match order_type {
            OrderType::Sell => &self.sell_orders,
            OrderType::Buy => &self.buy_orders,
            _ => return None,
        };

        if orders.is_empty() {
            return None;
        }
        orders
            .iter()
            .min_by(|a, b| a.platinum.cmp(&b.platinum))
            .cloned()
    }

    pub fn lowest_price(&self, order_type: OrderType) -> i64 {
        let order = self.lowest_order(order_type);
        if order.is_none() {
            return 0;
        }
        order.unwrap().platinum
    }

    pub fn highest_order(&self, order_type: OrderType) -> Option<Order> {
        let orders = match order_type {
            OrderType::Sell => &self.sell_orders,
            OrderType::Buy => &self.buy_orders,
            _ => return None,
        };

        if orders.is_empty() {
            return None;
        }
        orders
            .iter()
            .max_by(|a, b| a.platinum.cmp(&b.platinum))
            .cloned()
    }

    pub fn highest_price(&self, order_type: OrderType) -> i64 {
        let order = self.highest_order(order_type);
        if order.is_none() {
            return 0;
        }
        order.unwrap().platinum
    }

    pub fn get_price_range(&self) -> i64 {
        let lowest_price = self.lowest_price(OrderType::Sell);
        let highest_price = self.highest_price(OrderType::Buy);
        return lowest_price - highest_price;
    }

    pub fn total_count(&self) -> i64 {
        self.sell_orders.len() as i64 + self.buy_orders.len() as i64
    }
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
    pub positive: bool,

    #[serde(rename = "value")]
    pub value: f64,

    #[serde(rename = "url_name")]
    pub url_name: String,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuctionItem {
    #[serde(rename = "type")]
    pub item_type: String,

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

    #[serde(rename = "similarity")]
    pub similarity: Option<f64>,

    #[serde(rename = "extra_attributes")]
    pub extra_attributes: Option<Vec<RivenAttribute>>,

    #[serde(rename = "missing_attributes")]
    pub missing_attributes: Option<Vec<RivenAttribute>>,

    // Kuva Lich And Sisters of Parvos
    #[serde(rename = "element")]
    pub element: Option<String>,

    #[serde(rename = "quirk")]
    pub quirk: Option<String>,

    #[serde(rename = "having_ephemera")]
    pub having_ephemera: Option<bool>,

    #[serde(rename = "damage")]
    pub damage: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuctionOwner {
    #[serde(rename = "ingame_name")]
    pub ingame_name: String,

    #[serde(rename = "last_seen")]
    pub last_seen: String,

    #[serde(rename = "reputation")]
    pub reputation: i64,

    #[serde(rename = "locale")]
    pub locale: String,

    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "region")]
    pub region: String,

    #[serde(rename = "avatar")]
    pub avatar: Option<String>,
}

#[derive(sqlx::Decode, Serialize, Deserialize, Debug, Clone)]
pub struct PriceHistory {
    #[serde(rename = "user_id")]
    pub user_id: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "price")]
    pub price: i64,
}