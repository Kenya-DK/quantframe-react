use polars::{prelude::{DataFrame, NamedFrom}, series::Series};
use reqwest::header::HeaderMap;
use serde_json::json;

use crate::{
    error::AppError,
    logger,
    structs::{Order, Ordres},
    wfm_client2::client::ClientState,
};

pub struct OrderModule<'a> {
    pub client: &'a ClientState,
}

impl<'a> OrderModule<'a> {
    // Actions User Order
    pub async fn get_user_orders(&self, ingame_name: &str) -> Result<Ordres, AppError> {
        let url = format!("profile/{}/orders", ingame_name);
        match self.client.get(&url, None).await {
            Ok((orders, _headers)) => {
                logger::info(
                    "WarframeMarket",
                    format!("From User: {}", ingame_name).as_str(),
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
                Ok(order_id)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn update(
        &self,
        order_id: &str,
        platinum: i64,
        quantity: i64,
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
                Ok(order)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn close(&self, item: &str) -> Result<String, AppError> {
        // Get the user orders and find the order
        let mut ordres_vec = self.get_my_orders().await?;
        let mut ordres: Vec<Order> = ordres_vec.buy_orders;
        ordres.append(&mut ordres_vec.sell_orders);

        // Find Order by name.
        let order = ordres
            .iter()
            .find(|order| order.item.url_name == item)
            .clone();

        if order.is_none() {
            return Ok("No Order Found".to_string());
        }

        let url = format!("profile/orders/close/{}", order.unwrap().id);
        let result: Result<(Option<String>, HeaderMap), AppError> =
            self.client.put(&url, Some("order_id"), None).await;
        match result {
            Ok((order_data, _headers)) => {
                logger::info(
                    "WarframeMarket",
                    format!("Closed Order: {}", order.unwrap().id).as_str(),
                    true,
                    Some(self.client.log_file.as_str()),
                );
                Ok(order_data.unwrap_or("Order Successfully Closed".to_string()))
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

    // Helper
    pub async fn convert_orders_to_dataframe(
        &self,
        orders: Vec<Order>,
    ) -> Result<DataFrame, AppError> {
        let orders_df = DataFrame::new_no_checks(vec![
            // Assuming Order has fields field1, field2, ...
            Series::new(
                "id",
                orders
                    .iter()
                    .map(|order| order.id.clone())
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
                    .map(|order| order.item.url_name.clone())
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

    // End Helper
}
