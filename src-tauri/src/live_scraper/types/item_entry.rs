use std::hash::{Hash, Hasher};

use entity::dto::SubType;
use entity::stock_item::Model as StockItemModel;
use serde::{Deserialize, Serialize};
use service::{sea_orm::DatabaseConnection, StockItemQuery, WishListQuery};
use utils::{get_location, Error};

use crate::{cache::types::ItemPriceInfo, utils::ErrorFromExt};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stock_id")]
    pub stock_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "wish_list_id")]
    pub wish_list_id: Option<i64>,

    #[serde(rename = "wfm_url")]
    pub wfm_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sub_type")]
    pub sub_type: Option<SubType>,

    // Trading Stats.
    #[serde(default)]
    #[serde(rename = "priority")]
    pub priority: i64,

    #[serde(default)]
    #[serde(rename = "buy_quantity")]
    pub buy_quantity: i64,

    #[serde(default)]
    #[serde(rename = "sell_quantity")]
    pub sell_quantity: i64,

    #[serde(rename = "operation")]
    #[serde(default)]
    pub operation: Vec<String>,

    #[serde(rename = "order_type")]
    #[serde(default)]
    pub order_type: String,
}

impl Hash for ItemEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.wfm_url.hash(state);
        self.sub_type.hash(state);
    }
}

impl ItemEntry {
    pub fn new(
        stock_id: Option<i64>,
        wish_list_id: Option<i64>,
        wfm_url: String,
        sub_type: Option<SubType>,
        priority: i64,
        buy_quantity: i64,
        sell_quantity: i64,
        operation: Vec<String>,
        order_type: &str,
    ) -> ItemEntry {
        ItemEntry {
            stock_id,
            wish_list_id,
            wfm_url,
            sub_type,
            priority,
            buy_quantity,
            sell_quantity,
            operation,
            order_type: order_type.to_string(),
        }
    }
    pub fn uuid(&self) -> String {
        let mut uuid = self.wfm_url.clone();
        if let Some(sub_type) = self.sub_type.clone() {
            uuid.push_str(&format!("-{}", sub_type.shot_display()));
        }
        uuid
    }
    pub fn set_buy_quantity(mut self, quantity: i64) -> Self {
        self.buy_quantity = quantity;
        self
    }
    pub fn set_sell_quantity(mut self, quantity: i64) -> Self {
        self.sell_quantity = quantity;
        self
    }
    pub async fn get_stock_item(&self, conn: &DatabaseConnection) -> Result<StockItemModel, Error> {
        if let Some(stock_id) = self.stock_id {
            match StockItemQuery::find_by_id(conn, stock_id).await {
                Ok(stock_item) => {
                    if let Some(item) = stock_item {
                        Ok(item)
                    } else {
                        Err(Error::new(
                            "ItemEntry:GetStockItem",
                            "Stock item not found",
                            get_location!(),
                        )
                        .set_log_level(utils::LogLevel::Warning))
                    }
                }
                Err(e) => {
                    return Err(Error::from_db(
                        "ItemEntry:GetStockItem",
                        "Failed to get stock item by ID: {}",
                        e,
                        get_location!(),
                    ))
                }
            }
        } else {
            Err(Error::new(
                "ItemEntry:GetStockItem",
                "Stock ID is None",
                get_location!(),
            ))
        }
    }
    pub async fn get_wish_list_item(
        &self,
        conn: &DatabaseConnection,
    ) -> Result<entity::wish_list::wish_list::Model, Error> {
        if let Some(wish_list_id) = self.wish_list_id {
            match WishListQuery::get_by_id(conn, wish_list_id).await {
                Ok(item) => {
                    if let Some(item) = item {
                        Ok(item)
                    } else {
                        Err(Error::new(
                            "ItemEntry:GetWishListItem",
                            "Wish list item not found",
                            get_location!(),
                        )
                        .set_log_level(utils::LogLevel::Warning))
                    }
                }
                Err(e) => {
                    return Err(Error::from_db(
                        "ItemEntry:GetWishListItem",
                        "Failed to get wish list item by ID: {}",
                        e,
                        get_location!(),
                    ))
                }
            }
        } else {
            Err(Error::new(
                "ItemEntry:GetWishListItem",
                "Wish List ID is None",
                get_location!(),
            ))
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}
impl From<&ItemPriceInfo> for ItemEntry {
    fn from(item: &ItemPriceInfo) -> Self {
        ItemEntry::new(
            None,
            None,
            item.wfm_url.clone(),
            item.sub_type.clone(),
            0,
            1,
            0,
            vec!["Buy".to_string()],
            "closed",
        )
    }
}
impl From<&StockItemModel> for ItemEntry {
    fn from(item: &StockItemModel) -> Self {
        ItemEntry::new(
            Some(item.id),
            None,
            item.wfm_url.clone(),
            item.sub_type.clone(),
            1,
            0,
            item.owned,
            vec!["Sell".to_string()],
            "closed",
        )
    }
}
impl From<&entity::wish_list::wish_list::Model> for ItemEntry {
    fn from(item: &entity::wish_list::wish_list::Model) -> Self {
        ItemEntry::new(
            None,
            Some(item.id),
            item.wfm_url.clone(),
            item.sub_type.clone(),
            2,
            item.quantity,
            0,
            vec!["WishList".to_string()],
            "buy",
        )
    }
}
