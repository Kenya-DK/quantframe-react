use crate::{
    live_scraper::types::order_extra_info::OrderDetails,
    logger,
    utils::{
        enums::log_level::LogLevel,
        modules::{
            error::{ApiResult, AppError},
            logger::LoggerOptions,
            states,
        },
    },
    wfm_client::{
        client::WFMClient,
        enums::order_type::OrderType,
        types::{order::Order, order_close::OrderClose, orders::Orders},
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
    pub async fn progress_order(
        &mut self,
        wfm_id: &str,
        sub_type: Option<SubType>,
        mut quantity: i64,
        order_type: OrderType,
    ) -> Result<(String, Option<Order>), AppError> {
        let wfm = self.client.clone();
        let settings = states::settings()?;
        let mut operation = "NotFound";

        // Set quantity to 1 if it's less than 1
        if quantity <= 0 {
            quantity = 1;
        }
        // Get WFM Order
        let orders = wfm.orders().get_my_orders().await?;
        let mut order = orders.find_order_by_url_sub_type(&wfm_id, order_type, sub_type.as_ref());

        // Check if order exists
        if order.is_some() {
            let mut foundOrder = order.unwrap();
            // Subtract quantity from order
            foundOrder.quantity -= quantity;

            // Set Operation
            if foundOrder.quantity <= 0 {
                operation = "Deleted";
            } else {
                operation = "Updated";
            }

            if settings.live_scraper.stock_item.report_to_wfm {
                for _ in 0..quantity {
                    self.close(&foundOrder.id).await?;
                }
            } else if foundOrder.quantity <= 0 {
                self.delete(&foundOrder.id).await?;
            } else {
                self.update(
                    &foundOrder.id,
                    foundOrder.platinum,
                    foundOrder.quantity,
                    foundOrder.visible,
                    Some(foundOrder.info.clone()),
                )
                .await?;
            }
            // Update order in orders
            order = Some(foundOrder);
        }
        return Ok((operation.to_string(), order));
    }
    pub async fn get_my_orders(&mut self) -> Result<Orders, AppError> {
        let orders = self.get_user_orders().await?;
        Ok(orders)
    }
    pub async fn get_user_orders(&mut self) -> Result<Orders, AppError> {
        match self.client.get::<Vec<Order>>("v2", "orders/my", None).await {
            Ok(ApiResult::Success(payload, _headers)) => {
                let buy_orders: Vec<Order> = payload
                    .iter()
                    .filter(|o| o.order_type == OrderType::Buy)
                    .cloned()
                    .collect();

                let sell_orders: Vec<Order> = payload
                    .iter()
                    .filter(|o| o.order_type == OrderType::Sell)
                    .cloned()
                    .collect();
                let mut orders = Orders::new(sell_orders, buy_orders);
                orders.apply_trade_info()?;
                let total_orders = orders.buy_orders.len() + orders.sell_orders.len();

                self.client.debug(
                    &self.debug_id,
                    &self.get_component("GetUserOrders"),
                    format!("{} orders were fetched.", total_orders).as_str(),
                    None,
                );
                self.set_order_count(total_orders as i64)?;
                return Ok(orders);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(self.client.create_api_error(
                    &self.get_component("GetUserOrders"),
                    error,
                    eyre!("There was an error fetching orders"),
                    LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    pub async fn create(
        &mut self,
        item_id: &str,
        order_type: &str,
        platinum: i64,
        quantity: i64,
        visible: bool,
        per_trade: Option<i64>,
        sub_type: Option<SubType>,
        info: Option<OrderDetails>,
    ) -> Result<(String, Option<Order>), AppError> {
        self.client.auth().is_logged_in()?;
        let auth = states::auth()?;
        let limit = auth.order_limit;

        if limit != -1 && self.total_orders >= limit {
            logger::warning(
                &self.get_component("Create"),
                "You have reached the maximum number of orders",
                LoggerOptions::default(),
            );
            return Ok(("order_limit_reached".to_string(), None));
        }

        // Construct any JSON body
        let mut body = json!({
            "itemId": item_id,
            "type": order_type,
            "platinum": platinum,
            "quantity": quantity,
            "visible": visible
        });

        if let Some(per_trade) = per_trade {
            if per_trade <= 0 {
                body["perTrade"] = json!(1);
            } else {
                body["perTrade"] = json!(per_trade);
            }
        }

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
            .post::<Order>("v2", "order", Some("order"), body)
            .await
        {
            Ok(ApiResult::Success(mut payload, _headers)) => {
                self.add_order_count(1)?;
                if info.is_some() {
                    payload.info = info.unwrap();
                }
                self.client.debug(
                    &self.debug_id,
                    &self.get_component("Create"),
                    format!(
                        "Creating order Type: {} Item: {}, Platinum: {}, Quantity: {}, {}, Total Orders: {}, Limit: {}",
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
                let is_duplicate_order = error
                    .messages
                    .iter()
                    .any(|message| {
                        message.contains("app.order.error.exceededOrderLimitSamePrice")
                            || message.contains("app.post_order.already_created_no_duplicates")
                    });

                if is_duplicate_order {
                    let mut existing_order: Option<Order> = None;
                    if let Ok(current_orders) = self.get_user_orders().await {
                        existing_order = current_orders.find_order_by_url_sub_type(
                            item_id,
                            OrderType::from_str(order_type),
                            sub_type.as_ref(),
                        );
                    }

                    logger::warning(
                        &self.get_component("Create"),
                        "Order already exists with the same price, returning existing order",
                        LoggerOptions::default(),
                    );

                    return Ok(("order_already_exists".to_string(), existing_order));
                }

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

    pub async fn delete(&mut self, order_id: &str) -> Result<Order, AppError> {
        let url = format!("order/{}", order_id);
        self.client.auth().is_logged_in()?;
        match self
            .client
            .delete::<Order>("v2", &url, Some("order_id"))
            .await
        {
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
                    Some(message) if message.contains("app.order.notFound") => LogLevel::Warning,
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
        &mut self,
        order_id: &str,
        platinum: i64,
        quantity: i64,
        visible: bool,
        info: Option<OrderDetails>,
    ) -> Result<Order, AppError> {
        // Construct any JSON body
        let body = json!({
            "platinum": platinum,
            "quantity": quantity,
            "visible": visible
        });
        let url = format!("order/{}", order_id);
        self.client.auth().is_logged_in()?;
        match self
            .client
            .patch::<Order>("v2", &url, Some("order"), Some(body))
            .await
        {
            Ok(ApiResult::Success(mut payload, _headers)) => {
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
                self.update_state();
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

    pub async fn close(&mut self, id: &str) -> Result<Option<OrderClose>, AppError> {
        let url = format!("order/{}/close", id);
        self.client.auth().is_logged_in()?;

        match self
            .client
            .post::<Option<OrderClose>>("v2", &url, Some("order"), json!({ "quantity": 1 }))
            .await
        {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.subtract_order_count(1)?;
                return Ok(payload);
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

    pub async fn get_orders_by_item(
        &self,
        item: &str,
        sub_type: Option<&SubType>,
        exclude: bool,
    ) -> Result<Orders, AppError> {
        // let path = logger::get_log_folder().join(format!("{}.json", item));
        // match helper::read_json_file::<Orders>(&path) {
        //     Ok(orders) => {
        //         return Ok(orders);
        //     }
        //     Err(e) => {
        //         return Err(e);
        //     }
        // }

        let url = format!("orders/item/{}", item);

        let orders = match self
            .client
            .get::<Vec<Order>>("v2", &url, Some("orders"))
            .await
        {
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
        // logger::log_json(&format!("{}.json", item), &json!(orders))?;
        Ok(orders)
    }
    // End Helper
}
