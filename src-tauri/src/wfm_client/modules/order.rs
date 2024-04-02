use serde_json::json;
use crate::{
    helper, logger, utils::{enums::log_level::LogLevel, modules::error::{ApiResult, AppError}}, wfm_client::{client::WFMClient, enums::order_type::OrderType, types::{order::Order, orders::Orders}}
};

use eyre::eyre;
#[derive(Clone, Debug)]
pub struct OrderModule {
    pub client: WFMClient,
    pub debug_id: String,
    pub total_orders: i64,
    component: String,
}

impl OrderModule {
    pub fn new(client: WFMClient) -> Self {
        OrderModule {
            client,
            debug_id: "wfm_client_order".to_string(),
            total_orders: 0,
            component: "Orders".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}", self.component, component)
    }
    fn update_state(&self) {
        self.client.update_order_module(self.clone());
    }
    pub fn set_order_count(&mut self, increment: i64) -> Result<(), AppError> {
        let ref mut count = self.total_orders;
        *count = increment;
        self.update_state();
        Ok(())
    }

    pub fn subtract_order_count(&mut self, increment: i64) -> Result<(), AppError> {
        let ref mut count = self.total_orders;
        *count -= increment;
        if *count < 0 {
            *count = 0;
        }
        self.update_state();
        Ok(())
    }

    pub fn add_order_count(&mut self, increment: i64) -> Result<(), AppError> {
        let ref mut count = self.total_orders;
        *count += increment;
        self.update_state();
        Ok(())
    }

    pub async fn get_user_orders(&mut self, ingame_name: &str) -> Result<Orders, AppError> {
        let url = format!("profile/{}/orders", ingame_name);
        match self.client.get::<Orders>(&url, None).await {
            Ok(ApiResult::Success(payload, _headers)) => {
                let total_orders = payload.buy_orders.len() + payload.sell_orders.len();

                self.client.debug(
                    &self.debug_id,
                    &self.get_component("GetUserOrders"),
                    format!("{} orders were fetched.", total_orders).as_str(),
                    None,
                );
                self.set_order_count(total_orders as i64)?;
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(self.client.create_api_error(
                    &self.get_component("GetUserOrders"),
                    error,
                    eyre!("There was an error fetching orders for {}", ingame_name),
                    LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    pub async fn get_my_orders(&mut self) -> Result<Orders, AppError> {
        let auth = self.client.auth.lock()?.clone();
        let orders = self.get_user_orders(auth.ingame_name.as_str()).await?;
        Ok(orders)
    }

    pub async fn create(
        &mut self,
        item_id: &str,
        order_type: &str,
        platinum: i64,
        quantity: i64,
        visible: bool,
        rank: Option<f64>,
    ) -> Result<(String, Option<Order>), AppError> {
        let auth = self.client.auth.lock()?.clone();
        let limit = auth.order_limit;

        if self.total_orders >= limit {
            logger::warning_con(
                &self.get_component("Create"),
                "You have reached the maximum number of orders",
            );
            return Ok(("order_limit_reached".to_string(), None));
        }

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
                self.add_order_count(1)?;
                self.client.debug(
                    &self.debug_id,
                    &self.get_component("Create"),
                    format!(
                        "Creating order Type: {} Item: {}, Platinum: {}, Quantity: {}, Rank: {}, Total Orders: {}, Limit: {}",
                        order_type,
                        item_id,
                        platinum,
                        quantity,
                        rank.unwrap_or(0.0),
                        self.total_orders,
                        limit.clone()
                    )
                    .as_str(),
                    None,
                );
                self.emit("CREATE_OR_UPDATE", serde_json::to_value(&payload).unwrap());
                return Ok(("order_created".to_string(), Some(payload)));
            }
            Ok(ApiResult::Error(error, _headers)) => {
                let log_level = match error.messages.get(0) {
                    Some(message)
                        if message.contains("app.post_order.already_created_no_duplicates")
                            || message.contains("app.post_order.limit_exceeded") =>
                    {
                        LogLevel::Warning
                    }
                    _ => LogLevel::Error,
                };
                return Err(self.client.create_api_error(
                    &self.get_component("Create"),
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

    pub async fn delete(&mut self, order_id: &str) -> Result<String, AppError> {
        let url = format!("profile/orders/{}", order_id);
        match self.client.delete(&url, Some("order_id")).await {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.subtract_order_count(1)?;
                self.client.debug(
                    &self.debug_id,
                    &self.get_component("Delete"),
                    format!(
                        "Order {} was deleted. Total orders: {}",
                        order_id, self.total_orders
                    )
                    .as_str(),
                    None,
                );
                self.emit("DELETE", json!({ "id": &payload }));
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                let log_level = match error.messages.get(0) {
                    Some(message) if message.contains("app.delete_order.order_not_exist") => {
                        LogLevel::Warning
                    }
                    _ => LogLevel::Error,
                };
                return Err(self.client.create_api_error(
                    &self.get_component("Delete"),
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
                    &self.get_component("Update"),
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
                        if message.contains("app.form.not_exist")
                            || message.contains("app.form.invalid") =>
                    {
                        LogLevel::Warning
                    }
                    _ => LogLevel::Error,
                };
                return Err(self.client.create_api_error(
                    &self.get_component("Update"),
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

    pub async fn close(&mut self, item: &str, order_type: OrderType) -> Result<String, AppError> {
        // Get the user orders and find the order
        let mut orders_vec = self.get_my_orders().await?;
        let mut orders: Vec<Order> = orders_vec.buy_orders;
        orders.append(&mut orders_vec.sell_orders);
        // Find Order by name.
        let order = orders
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
                    self.subtract_order_count(1)?;
                    self.client.debug(
                        &self.debug_id,
                        &self.get_component("Close"),
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
                        Some(message) if message.contains("app.close_order.order_not_exist") => {
                            LogLevel::Warning
                        }
                        _ => LogLevel::Error,
                    };
                    return Err(self.client.create_api_error(
                        &self.get_component("Close"),
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
        }
        let order_data = result.unwrap();
        order.quantity = order_data["quantity"].as_i64().unwrap();
        self.emit("CREATE_OR_UPDATE", serde_json::to_value(&order).unwrap());
        return Ok("Order Successfully Closed and Updated".to_string());
    }
    // End Actions User Order

    pub async fn get_orders_by_item(&self, item: &str) -> Result<Orders, AppError> {
        let url = format!("items/{}/orders", item);

        let orders = match self.client.get::<Vec<Order>>(&url, Some("orders")).await {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.client.debug(
                    &self.debug_id,
                    &self.get_component("GetOrdersByItem"),
                    format!("Orders for {} were fetched. found: {}", item, payload.len()).as_str(),
                    None,
                );
                payload
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(self.client.create_api_error(
                    &self.get_component("GetOrdersByItem"),
                    error,
                    eyre!("There was an error fetching orders for {}", item),
                    LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        };

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

        let mut buy_orders: Vec<Order> = orders
            .iter()
            .filter(|order| order.order_type == OrderType::Buy)
            .cloned()
            .collect();
        buy_orders.sort_by(|a, b| b.platinum.cmp(&a.platinum));

        let mut sell_orders: Vec<Order> = orders
            .iter()
            .filter(|order| order.order_type == OrderType::Sell)
            .cloned()
            .collect();
        sell_orders.sort_by(|a, b| a.platinum.cmp(&b.platinum));

        Ok(Orders {
            buy_orders,
            sell_orders,
        })
    }

    pub fn emit(&self, operation: &str, data: serde_json::Value) {
        helper::emit_update("orders", operation, Some(data));
    }
    // End Helper
}
