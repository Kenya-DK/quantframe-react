use polars::{
    prelude::{DataFrame, NamedFrom},
    series::Series,
};
use serde_json::json;

use crate::{
    enums::OrderType,
    error::{ApiResult, AppError},
    helper,
    structs::{Order, Ordres},
    wfm_client::client::WFMClient,
};

use eyre::eyre;
pub struct OrderModule<'a> {
    pub client: &'a WFMClient,
    pub debug_id: String,
}

impl<'a> OrderModule<'a> {
    // Actions User Order
    pub async fn get_user_orders(&self, ingame_name: &str) -> Result<Ordres, AppError> {
        let url = format!("profile/{}/orders", ingame_name);
        match self.client.get::<Ordres>(&url, None).await {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.client.debug(
                    &self.debug_id,
                    "Order:GetUserOrders",
                    format!(
                        "{} orders were fetched.",
                        payload.buy_orders.len() + payload.sell_orders.len()
                    )
                    .as_str(),
                    None,
                );
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(self.client.create_api_error(
                    "Order:GetUserOrders",
                    error,
                    eyre!("There was an error fetching orders for {}", ingame_name),
                    crate::enums::LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    pub async fn get_my_orders(&self) -> Result<Ordres, AppError> {
        let auth = self.client.auth.lock()?.clone();
        let orders = self.get_user_orders(auth.ingame_name.as_str()).await?;
        Ok(orders)
    }

    pub async fn create(
        &self,
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
            Ok(ApiResult::Success(payload, _headers)) => {
                self.client.debug(
                    &self.debug_id,
                    "Order:Create",
                    format!(
                        "Order created type: {} item: {}, platinum: {}, quantity: {}, rank: {}",
                        order_type,
                        item_id,
                        platinum,
                        quantity,
                        rank.unwrap_or(-1.0)
                    )
                    .as_str(),
                    None,
                );
                self.emit("CREATE_OR_UPDATE", serde_json::to_value(&payload).unwrap());
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                let log_level = match error.messages.get(0) {
                    Some(message) if message.contains("app.delete_order.order_not_exist") => {
                        crate::enums::LogLevel::Warning
                    }
                    _ => crate::enums::LogLevel::Error,
                };
                return Err(self.client.create_api_error(
                    "Order:Create",
                    error,
                    eyre!("There was an error creating order"),
                    log_level,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    pub async fn delete(&self, order_id: &str) -> Result<String, AppError> {
        let url = format!("profile/orders/{}", order_id);
        match self.client.delete(&url, Some("order_id")).await {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.client.debug(
                    &self.debug_id,
                    "Order:Delete",
                    format!("Order {} was deleted.", order_id).as_str(),
                    None,
                );
                self.emit("DELETE", json!({ "id": &payload }));
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                let log_level = match error.messages.get(0) {
                    Some(message) if message.contains("app.delete_order.order_not_exist") => {
                        crate::enums::LogLevel::Warning
                    }
                    _ => crate::enums::LogLevel::Error,
                };
                return Err(self.client.create_api_error(
                    "Order:Delete",
                    error,
                    eyre!("There was an error deleting order {}", order_id),
                    log_level,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    pub async fn update(
        &self,
        order_id: &str,
        platinum: i32,
        quantity: i32,
        visible: bool,
    ) -> Result<Order, AppError> {
        // Construct any JSON body
        let body = json!({
            "platinum": platinum,
            "quantity": quantity,
            "visible": visible
        });
        let url = format!("profile/orders/{}", order_id);
        match self
            .client
            .put::<Order>(&url, Some("order"), Some(body))
            .await
        {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.client.debug(
                    &self.debug_id,
                    "Order:Update",
                    format!(
                        "Order id: {}, platinum: {}, quantity: {}",
                        order_id, platinum, quantity
                    )
                    .as_str(),
                    None,
                );
                self.emit("CREATE_OR_UPDATE", serde_json::to_value(&payload).unwrap());
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                let log_level = match error.messages.get(0) {
                    Some(message)
                        if message.contains("app.delete_order.order_not_exist")
                            || message.contains("app.form.not_exist") =>
                    {
                        crate::enums::LogLevel::Warning
                    }
                    _ => crate::enums::LogLevel::Error,
                };
                return Err(self.client.create_api_error(
                    "Order:Update",
                    error,
                    eyre!("There was an error updating order {}", order_id),
                    log_level,
                ));
            }
            Err(err) => {
                return Err(err);
            }
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

        let result: Option<serde_json::Value> =
            match self.client.put(&url, Some("order"), None).await {
                Ok(ApiResult::Success(payload, _headers)) => {
                    self.client.debug(
                        &self.debug_id,
                        "Order:Close",
                        format!(
                            "Order {} type: {} was closed.",
                            order.id,
                            order.order_type.as_str()
                        )
                        .as_str(),
                        None,
                    );
                    payload
                }
                Ok(ApiResult::Error(error, _headers)) => {
                    let log_level = match error.messages.get(0) {
                        Some(message)
                            if message.contains("app.close_order.order_not_exist")
                                || message.contains("app.form.not_exist") =>
                        {
                            crate::enums::LogLevel::Warning
                        }
                        _ => crate::enums::LogLevel::Error,
                    };
                    return Err(self.client.create_api_error(
                        "Order:Closeing",
                        error,
                        eyre!("There was an error closing order {}", order.id),
                        log_level,
                    ));
                }
                Err(err) => {
                    return Err(err);
                }
            };

        if result.is_none() {
            self.emit("DELETE", json!({ "id": &order.id }));
            return Ok("Order Successfully Closed".to_string());
        } else {
            let order_data = result.unwrap();
            order.quantity = order_data["quantity"].as_i64().unwrap();
            self.emit("CREATE_OR_UPDATE", serde_json::to_value(&order).unwrap());
            return Ok("Order Successfully Closed and Updated".to_string());
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

        let orders = match self.client.get::<Vec<Order>>(&url, Some("orders")).await {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.client.debug(
                    &self.debug_id,
                    "Order:GetOrdersByItem",
                    format!("Orders for {} were fetched. found: {}", item, payload.len()).as_str(),
                    None,
                );
                payload
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(self.client.create_api_error(
                    "Order:GetOrdersByItem",
                    error,
                    eyre!("There was an error fetching orders for {}", item),
                    crate::enums::LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
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
            Series::new("order_type", vec![order.order_type.as_str()]),
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
