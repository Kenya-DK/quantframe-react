use polars::{
    prelude::{DataFrame, NamedFrom},
    series::Series,
};
use reqwest::header::HeaderMap;
use serde_json::json;

use crate::{
    error::AppError,
    helper, logger,
    structs::{Order, Ordres},
    wfm_client::client::WFMClient, enums::OrderType,
};

pub struct OrderModule<'a> {
    pub client: &'a WFMClient,
}

impl<'a> OrderModule<'a> {
    // Actions User Order
    pub async fn get_user_orders(&self, ingame_name: &str) -> Result<Ordres, AppError> {
        let url = format!("profile/{}/orders", ingame_name);
        match self.client.get(&url, None).await {
            Ok((orders, _headers)) => {
                logger::info(
                    "WarframeMarket",
                    format!("Getting Orders for {}", ingame_name).as_str(),
                    true,
                    Some(self.client.log_file.as_str()),
                );
                Ok(orders)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_my_orders(&self) -> Result<Ordres, AppError> {
        let auth = self.client.auth.lock()?.clone();
        let orders = self.get_user_orders(auth.ingame_name.as_str()).await?;
        Ok(orders)
    }

    pub async fn create(
        &self,
        item_name: &str,
        item_id: &str,
        order_type: &str,
        platinum: i64,
        quantity: i64,
        visible: bool,
        rank: Option<f64>,
    ) -> Result<Order, AppError> {
        // Construct any JSON body
        let mut body = json!({
            "item": item_id,
            "order_type": order_type,
            "platinum": platinum,
            "quantity": quantity,
            "visible": visible
        });
        // Add rank to body if it exists
        if let Some(rank) = rank {
            body["rank"] = json!(rank);
        }
        match self
            .client
            .post("profile/orders", Some("order"), body)
            .await
        {
            Ok((order, _headers)) => {
                logger::info("WarframeMarket", format!("Created Order: {}, Item Name: {}, Item Id: {},  Platinum: {}, Quantity: {}, Visible: {}", order_type, item_name, item_id ,platinum ,quantity ,visible).as_str(), true, Some(self.client.log_file.as_str()));
                self.emit("CREATE_OR_UPDATE", serde_json::to_value(&order).unwrap());
                Ok(order)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn delete(
        &self,
        order_id: &str,
        item_name: &str,
        item_id: &str,
        order_type: &str,
    ) -> Result<String, AppError> {
        let url = format!("profile/orders/{}", order_id);
        match self.client.delete(&url, Some("order_id")).await {
            Ok((order_id, _headers)) => {
                logger::info(
                    "WarframeMarket",
                    format!(
                        "Deleted order: {}, Item Name: {}, Item Id: {}, Type: {}",
                        order_id, item_name, item_id, order_type
                    )
                    .as_str(),
                    true,
                    Some(self.client.log_file.as_str()),
                );
                self.emit("DELETE", json!({ "id": &order_id }));
                Ok(order_id)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn update(
        &self,
        order_id: &str,
        platinum: i32,
        quantity: i32,
        visible: bool,
        item_name: &str,
        item_id: &str,
        order_type: &str,
    ) -> Result<Order, AppError> {
        // Construct any JSON body
        let body = json!({
            "platinum": platinum,
            "quantity": quantity,
            "visible": visible
        });
        let url = format!("profile/orders/{}", order_id);
        match self.client.put(&url, Some("order"), Some(body)).await {
            Ok((order, _headers)) => {
                logger::info("WarframeMarket", format!("Updated Order Id: {}, Item Name: {}, Item Id: {}, Platinum: {}, Quantity: {}, Visible: {}, Type: {}", order_id, item_name, item_id,platinum ,quantity ,visible, order_type).as_str(), true, Some(&self.client.log_file));
                self.emit("CREATE_OR_UPDATE", serde_json::to_value(&order).unwrap());
                Ok(order)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn close(&self, item: &str, order_type: OrderType) -> Result<String, AppError> {
        // Get the user orders and find the order
        let mut ordres_vec = self.get_my_orders().await?;
        let mut ordres: Vec<Order> = ordres_vec.buy_orders;
        ordres.append(&mut ordres_vec.sell_orders);
        // Find Order by name.
        let order = ordres
            .iter()
            .find(|order| {
                order.item.as_ref().unwrap().url_name == item && order.order_type == order_type
            })
            .clone();

        if order.is_none() {
            return Ok("No Order Found".to_string());
        }

        let mut order = order.unwrap().to_owned();

        let url = format!("profile/orders/close/{}", order.id);
        let result: Result<(Option<serde_json::Value>, HeaderMap), AppError> =
            self.client.put(&url, Some("order"), None).await;
        match result {
            Ok((order_data, _headers)) => {
                if order_data.is_none() {
                    self.emit("DELETE", json!({ "id": &order.id }));
                    return Ok("Order Successfully Closed".to_string());
                } else {
                    let order_data = order_data.unwrap();
                    order.quantity= order_data["quantity"].as_i64().unwrap();
                    self.emit("CREATE_OR_UPDATE", serde_json::to_value(&order).unwrap());
                    return Ok("Order Successfully Closed and Updated".to_string());
                }
                // logger::info(
                //     "WarframeMarket",
                //     format!("Closed Order: {}", order.unwrap().id).as_str(),
                //     true,
                //     Some(self.client.log_file.as_str()),
                // );
                // Ok(order_data.unwrap_or("Order Successfully Closed".to_string()))
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_orders_as_dataframe(&self) -> Result<(DataFrame, DataFrame), AppError> {
        let current_orders = self.get_my_orders().await?;
        let buy_orders = current_orders.buy_orders.clone();

        let sell_orders = current_orders.sell_orders.clone();

        Ok((
            self.convert_orders_to_dataframe(buy_orders).await?,
            self.convert_orders_to_dataframe(sell_orders).await?,
        ))
    }
    // End Actions User Order

    // Methods
    pub async fn get_ordres_by_item(&self, item: &str) -> Result<DataFrame, AppError> {
        let url = format!("items/{}/orders", item);

        let orders: Vec<Order> = match self.client.get(&url, Some("orders")).await {
            Ok((orders, _headers)) => orders,
            Err(e) => return Err(e),
        };

        if orders.len() == 0 {
            return Ok(DataFrame::new_no_checks(vec![]));
        }
        let mod_rank = orders
            .iter()
            .max_by(|a, b| a.mod_rank.cmp(&b.mod_rank))
            .unwrap()
            .mod_rank;

        let orders: Vec<Order> = orders
            .into_iter()
            .filter(|order| {
                if let Some(user) = &order.user {
                    user.status == "ingame" && order.mod_rank == mod_rank
                } else {
                    false
                }
            })
            .collect();
        Ok(self.convert_orders_to_dataframe(orders).await?)
    }
    // End Methods

    // Helper
    pub fn convet_order_to_datafream(&self, order: Order) -> Result<DataFrame, AppError> {
        let orders_df = DataFrame::new_no_checks(vec![
            Series::new("id", vec![order.id.clone()]),
            Series::new("visible", vec![order.visible.clone()]),
            Series::new(
                "url_name",
                vec![order
                    .item
                    .map(|item| item.url_name)
                    .unwrap_or("".to_string())],
            ),
            Series::new("platinum", vec![order.platinum.clone()]),
            Series::new("platform", vec![order.platform.clone()]),
            Series::new("order_type", vec![order.order_type.as_str().clone()]),
            Series::new("quantity", vec![order.quantity.clone()]),
            Series::new("last_update", vec![order.last_update.clone()]),
            Series::new("creation_date", vec![order.creation_date.clone()]),
        ]);
        Ok(orders_df)
    }
    pub async fn convert_orders_to_dataframe(
        &self,
        orders: Vec<Order>,
    ) -> Result<DataFrame, AppError> {
        let orders_df = DataFrame::new_no_checks(vec![
            Series::new(
                "id",
                orders
                    .iter()
                    .map(|order| order.id.clone())
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "username",
                orders
                    .iter()
                    .map(|order| {
                        if let Some(user) = &order.user {
                            user.ingame_name.clone()
                        } else {
                            "None".to_string()
                        }
                    })
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "visible",
                orders
                    .iter()
                    .map(|order| order.visible.clone())
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "url_name",
                orders
                    .iter()
                    .map(|order| {
                        order
                            .clone()
                            .item
                            .map(|item| item.url_name)
                            .unwrap_or("".to_string())
                    })
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "platinum",
                orders
                    .iter()
                    .map(|order| order.platinum.clone())
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "platform",
                orders
                    .iter()
                    .map(|order| order.platform.clone())
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "order_type",
                orders
                    .iter()
                    .map(|order| order.order_type.as_str())
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "quantity",
                orders
                    .iter()
                    .map(|order| order.quantity.clone())
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "last_update",
                orders
                    .iter()
                    .map(|order| order.last_update.clone())
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "creation_date",
                orders
                    .iter()
                    .map(|order| order.creation_date.clone())
                    .collect::<Vec<_>>(),
            ),
        ]);
        Ok(orders_df)
    }
    pub fn emit(&self, operation: &str, data: serde_json::Value) {
        helper::emit_update("orders", operation, Some(data));
    }
    // End Helper
}
