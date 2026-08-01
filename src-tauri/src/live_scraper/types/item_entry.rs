use std::{
    fmt::Display,
    hash::{Hash, Hasher},
};

use entity::stock_item::Model as StockItemModel;
use entity::wish_list::Model as WishListModel;
use serde::{Deserialize, Serialize};
use serde_json::json;
use service::{
    sea_orm::DatabaseConnection, StockItemMutation, StockItemQuery, WishListMutation, WishListQuery,
};
use utils::{get_location, info, Error, LoggerOptions, OperationSet, Properties, SubType};
use wf_market::{
    enums::OrderType,
    types::{OrderList, OrderWithUser},
};

use crate::{cache::types::ItemPriceInfo, send_event, types::UIEvent, utils::SubTypeExt};

//
// Market Information
//

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ItemMarketInfo {
    pub lowest_price: i64,
    pub highest_price: i64,
    pub price_range: i64,
    pub volume: usize,
}

impl ItemMarketInfo {
    pub fn new(live_orders: &OrderList<OrderWithUser>, order_type: OrderType) -> Self {
        Self {
            lowest_price: live_orders.lowest_price(order_type),
            highest_price: live_orders.highest_price(order_type),
            price_range: live_orders.price_range(order_type),
            volume: if order_type == OrderType::Buy {
                live_orders.buy_orders.len()
            } else {
                live_orders.sell_orders.len()
            },
        }
    }
}
impl Display for ItemMarketInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Lowest: {} | Highest: {} | Range: {} | Volume: {}",
            self.lowest_price, self.highest_price, self.price_range, self.volume
        )
    }
}
//
// Item Entry
//

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wish_list_id: Option<i64>,

    #[serde(rename = "wfm_url")]
    pub wfm_url: String,

    #[serde(rename = "wfm_id")]
    pub wfm_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<SubType>,

    #[serde(default)]
    pub priority: i64,

    #[serde(default)]
    pub buy_quantity: i64,

    pub sell_quantity: i64,

    #[serde(default, flatten)]
    pub operations: OperationSet,

    #[serde(default)]
    pub order_type: String,

    #[serde(default)]
    pub buy_market_info: ItemMarketInfo,

    #[serde(default)]
    pub sell_market_info: ItemMarketInfo,

    #[serde(default, flatten)]
    pub properties: Properties,
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
        wfm_url: impl Into<String>,
        wfm_id: impl Into<String>,
        sub_type: Option<SubType>,
        priority: i64,
        buy_quantity: i64,
        sell_quantity: i64,
        operations: Vec<String>,
        order_type: &str,
        properties: Properties,
    ) -> Self {
        Self {
            stock_id,
            wish_list_id,
            wfm_url: wfm_url.into(),
            wfm_id: wfm_id.into(),
            sub_type,
            priority,
            buy_quantity,
            sell_quantity,
            operations: OperationSet::from(operations),
            order_type: order_type.to_owned(),
            buy_market_info: ItemMarketInfo::default(),
            sell_market_info: ItemMarketInfo::default(),
            properties,
        }
    }

    //
    // Market
    //

    pub fn apply_market_info(&mut self, live_orders: &OrderList<OrderWithUser>) {
        self.buy_market_info = ItemMarketInfo::new(live_orders, OrderType::Buy);
        self.sell_market_info = ItemMarketInfo::new(live_orders, OrderType::Sell);
    }

    //
    // Helpers
    //

    pub fn uuid(&self) -> String {
        match &self.sub_type {
            Some(sub_type) => format!("{}-{}", self.wfm_url, sub_type.shot_display()),
            None => self.wfm_url.clone(),
        }
    }

    pub fn get_quantity(&self, order_type: OrderType) -> i64 {
        match order_type {
            OrderType::Buy => self.buy_quantity,
            OrderType::Sell => self.sell_quantity,
        }
    }

    pub fn set_quantity(&mut self, order_type: OrderType, quantity: i64) -> Self {
        match order_type {
            OrderType::Buy => self.buy_quantity = quantity,
            OrderType::Sell => self.sell_quantity = quantity,
        }

        self.clone()
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    //
    // Database
    //

    pub async fn get_stock_item(&self, conn: &DatabaseConnection) -> Result<StockItemModel, Error> {
        let stock_id = self.stock_id.ok_or_else(|| {
            Error::new(
                "ItemEntry:GetStockItem",
                "Stock ID is None",
                get_location!(),
            )
        })?;

        let item = StockItemQuery::find_by_id(conn, stock_id)
            .await
            .map_err(|e| e.with_location(get_location!()))?;

        item.ok_or_else(|| {
            Error::new(
                "ItemEntry:GetStockItem",
                format!("Stock item not found for ID: {}", stock_id),
                get_location!(),
            )
            .set_log_level(utils::LogLevel::Warning)
        })
    }

    pub async fn get_wish_list_item(
        &self,
        conn: &DatabaseConnection,
    ) -> Result<WishListModel, Error> {
        let wish_list_id = self.wish_list_id.ok_or_else(|| {
            Error::new(
                "ItemEntry:GetWishListItem",
                "Wish List ID is None",
                get_location!(),
            )
        })?;

        let item = WishListQuery::get_by_id(conn, wish_list_id)
            .await
            .map_err(|e| e.with_location(get_location!()))?;

        item.ok_or_else(|| {
            Error::new(
                "ItemEntry:GetWishListItem",
                format!("Wish List item not found for ID: {}", wish_list_id),
                get_location!(),
            )
            .set_log_level(utils::LogLevel::Warning)
        })
    }

    pub async fn get_stock_item_or_error(
        &self,
        conn: &DatabaseConnection,
    ) -> Result<StockItemModel, Error> {
        self.get_stock_item(conn).await.map_err(|e| {
            e.with_location(get_location!())
                .with_context(self.to_json())
        })
    }

    pub async fn get_wishlist_item_or_error(
        &self,
        conn: &DatabaseConnection,
    ) -> Result<WishListModel, Error> {
        self.get_wish_list_item(conn).await.map_err(|e| {
            e.with_location(get_location!())
                .with_context(self.to_json())
        })
    }

    pub async fn finalize_stock_item(
        &self,
        conn: &DatabaseConnection,
        component: &str,
        stock_item: &mut StockItemModel,
        log_options: &LoggerOptions,
    ) -> Result<(), Error> {
        if stock_item.is_dirty {
            match StockItemMutation::update_by_id(conn, stock_item.to_update()).await {
                Ok(_) => {
                    info(
                        format!("{}StockItemUpdate", component),
                        &format!("Updated stock item: {:?}", self.stock_id),
                        log_options,
                    );
                    if stock_item.update_gui() {
                        send_event!(
                            UIEvent::RefreshStockItems,
                            json!({"id": self.stock_id, "source": component})
                        );
                    }
                }
                Err(e) => return Err(e.with_location(get_location!())),
            }
        }
        Ok(())
    }

    pub async fn finalize_wishlist_item(
        &self,
        conn: &DatabaseConnection,
        component: &str,
        wishlist_item: &mut WishListModel,
        log_options: &LoggerOptions,
    ) -> Result<(), Error> {
        if wishlist_item.is_dirty {
            match WishListMutation::update_by_id(conn, wishlist_item.to_update()).await {
                Ok(_) => {
                    info(
                        format!("{}WishListUpdate", component),
                        &format!("Updated wishlist item: {:?}", self.wish_list_id),
                        log_options,
                    );
                    send_event!(
                        UIEvent::RefreshWishListItems,
                        json!({"id": self.wish_list_id, "source": component})
                    );
                }
                Err(e) => return Err(e.with_location(get_location!())),
            }
        }
        Ok(())
    }
}

//
// Conversions
//

impl From<&ItemPriceInfo> for ItemEntry {
    fn from(item: &ItemPriceInfo) -> Self {
        Self::new(
            None,
            None,
            item.wfm_url.clone(),
            item.wfm_id.clone(),
            item.sub_type.clone(),
            0,
            1,
            0,
            vec!["Buy".into()],
            "closed",
            Properties::default(),
        )
    }
}

impl From<&StockItemModel> for ItemEntry {
    fn from(item: &StockItemModel) -> Self {
        Self::new(
            Some(item.id),
            None,
            item.wfm_url.clone(),
            item.wfm_id.clone(),
            item.sub_type.clone(),
            1,
            0,
            item.owned,
            vec!["Sell".into()],
            "closed",
            Properties::default(),
        )
    }
}

impl From<&entity::wish_list::wish_list::Model> for ItemEntry {
    fn from(item: &entity::wish_list::wish_list::Model) -> Self {
        Self::new(
            None,
            Some(item.id),
            item.wfm_url.clone(),
            item.wfm_id.clone(),
            item.sub_type.clone(),
            2,
            item.quantity,
            0,
            vec!["WishList".into()],
            "buy",
            Properties::default(),
        )
    }
}

impl From<&qf_api::types::SyndicateItemPrice> for ItemEntry {
    fn from(item: &qf_api::types::SyndicateItemPrice) -> Self {
        Self::new(
            None,
            None,
            item.wfm_id.clone(),
            item.wfm_id.clone(),
            item.sub_type.clone(),
            1,
            0,
            1,
            vec!["Syndicate".into()],
            "sell",
            Properties::from(json!({
                "syndicate": item.syndicate,
                "standingCost": item.standing_cost,
            })),
        )
    }
}
