use polars::{
    prelude::{DataFrame, NamedFrom},
    series::Series,
};
use serde_json::json;

use crate::{
    enums::OrderType,
    error::{ApiResult, AppError},
    helper, logger,
    structs::{Order, Ordres},
    wfm_client::client::WFMClient,
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

    pub async fn get_user_orders(&mut self, ingame_name: &str) -> Result<Ordres, AppError> {
        let url = format!("profile/{}/orders", ingame_name);
        match self.client.get::<Ordres>(&url, None).await {
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
                    crate::enums::LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    pub async fn get_my_orders(&mut self) -> Result<Ordres, AppError> {
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
    ) -> Result<Option<Order>, AppError> {
        let auth = self.client.auth.lock()?.clone();
        let limit = auth.order_limit;

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

        if self.total_orders >= limit {
            logger::warning_con(
                &self.get_component("Create"),
                "You have reached the maximum number of orders",
            );
            return Ok(None);
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
                        "Order created type: {} item: {}, platinum: {}, quantity: {}, rank: {}, total orders: {}",
                        order_type,
                        item_id,
                        platinum,
                        quantity,
                        rank.unwrap_or(0.0),
                        self.total_orders
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
                        if message.contains("app.post_order.already_created_no_duplicates")
                            || message.contains("app.post_order.limit_exceeded") =>
                    {
                        crate::enums::LogLevel::Warning
                    }
                    _ => crate::enums::LogLevel::Error,
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
                        crate::enums::LogLevel::Warning
                    }
                    _ => crate::enums::LogLevel::Error,
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
                        crate::enums::LogLevel::Warning
                    }
                    _ => crate::enums::LogLevel::Error,
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
                            crate::enums::LogLevel::Warning
                        }
                        _ => crate::enums::LogLevel::Error,
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

    pub async fn get_orders_as_dataframe(&mut self) -> Result<(DataFrame, DataFrame), AppError> {
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
