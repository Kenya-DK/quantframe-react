use crate::{
    logger,
    utils::{
        enums::log_level::LogLevel,
        modules::error::{ApiResult, AppError},
    },
    wfm_client::{
        client::WFMClient,
        enums::order_type::OrderType,
        types::{order::Order, orders::Orders},
    },
};
use entity::sub_type::SubType;
use serde_json::json;

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

    pub async fn progress_order(
        &mut self,
        url: &str,
        sub_type: Option<SubType>,
        mut quantity: i64,
        order_type: OrderType,
        need_update: bool,
    ) -> Result<(String, Option<Order>), AppError> {
        let wfm = self.client.clone();
        let settings = self.client.settings.lock()?.clone();

        // Set quantity to 1 if it's less than 1
        if quantity <= 0 {
            quantity = 1;
        }
        // Check if the order is a Buy order and report_to_wfm is true, or if the order is a Sale order
        if order_type == OrderType::Buy && settings.live_scraper.stock_item.report_to_wfm
            || order_type == OrderType::Sell
        {
            // Get WFM Order
            let orders = wfm.orders().get_my_orders().await?;
            let order = orders.find_order_by_url_sub_type(&url, order_type, sub_type.as_ref());

            // Check if order exists
            if order.is_some() {
                let mut order = order.unwrap();
                // Subtract quantity from order
                order.quantity -= quantity;

                // If report_to_wfm is true, close the order
                if settings.live_scraper.stock_item.report_to_wfm {
                    self.close(&order.id).await?;
                } else {
                    // Delete order if quantity is less than or equal to 0 and update if not
                    if order.quantity <= 0 {
                        self.delete(&order.id).await?;
                    } else if need_update {
                        self.update(&order.id, order.platinum, order.quantity, order.visible)
                            .await?;
                    }
                }
                // Return order_deleted if quantity is less than or equal to 0, else return order_updated
                if order.quantity <= 0 {
                    return Ok(("Deleted".to_string(), Some(order)));
                } else {
                    return Ok(("Updated".to_string(), Some(order)));
                }
            }
            // Return order_not_found if order does not exist
            return Ok(("NotFound".to_string(), None));
        }
        // Return order_not_reported if the order is not a Buy order or a Sale order
        return Ok(("NotReported".to_string(), None));
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

    pub async fn find_order_by_url_sub_type(
        &mut self,
        url: &str,
        sub_type: Option<&SubType>,
        order_type: OrderType,
    ) -> Result<Option<Order>, AppError> {
        match self.get_my_orders().await {
            Ok(orders) => {
                let order = orders.find_order_by_url_sub_type(url, order_type, sub_type);
                if order.is_some() {
                    return Ok(order);
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
        Ok(None)
    }

    pub async fn create(
        &mut self,
        item_id: &str,
        order_type: &str,
        platinum: i64,
        quantity: i64,
        visible: bool,
        sub_type: Option<SubType>,
    ) -> Result<(String, Option<Order>), AppError> {
        self.client.auth().is_logged_in()?;
        let auth = self.client.auth.lock()?.clone();
        let limit = auth.order_limit;

        if limit != -1 && self.total_orders >= limit {
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

        // Add SubType data
        if let Some(item_sub) = sub_type.clone() {
            if let Some(mod_rank) = item_sub.rank {
                body["rank"] = json!(mod_rank);
            }
            if let Some(subtype) = item_sub.variant {
                body["subtype"] = json!(subtype);
            }
            if let Some(amber_stars) = item_sub.amber_stars {
                body["amber_stars"] = json!(amber_stars);
            }
            if let Some(cyan_stars) = item_sub.cyan_stars {
                body["cyan_stars"] = json!(cyan_stars);
            }
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
                        sub_type.unwrap_or(SubType::new_empty()).display(),
                        self.total_orders,
                        limit.clone()
                    )
                    .as_str(),
                    None,
                );
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
        self.client.auth().is_logged_in()?;
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
        platinum: i64,
        quantity: i64,
        visible: bool,
    ) -> Result<Order, AppError> {
        // Construct any JSON body
        let body = json!({
            "platinum": platinum,
            "quantity": quantity,
            "visible": visible
        });
        let url = format!("profile/orders/{}", order_id);
        self.client.auth().is_logged_in()?;
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

    pub async fn close(&mut self, id: &str) -> Result<bool, AppError> {
        let url = format!("profile/orders/close/{}", id);
        self.client.auth().is_logged_in()?;

        match self
            .client
            .put::<serde_json::Value>(&url, Some("order"), None)
            .await
        {
            Ok(ApiResult::Success(_payload, _headers)) => {
                self.subtract_order_count(1)?;
                return Ok(true);
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
                    eyre!("There was an error closing order {}", id),
                    log_level,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        };
    }
    // End Actions User Order

    pub async fn get_orders_by_item(
        &self,
        item: &str,
        sub_type: Option<&SubType>,
        exclude: bool,
    ) -> Result<Orders, AppError> {
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

        let orders: Vec<Order> = orders
            .into_iter()
            .filter(|order| {
                if let Some(user) = &order.user {
                    user.status == "ingame"
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

        let mut orders = Orders {
            buy_orders,
            sell_orders,
        };
        orders = orders.filter_by_sub_type(sub_type, exclude);
        Ok(orders)
    }
    // End Helper
}
