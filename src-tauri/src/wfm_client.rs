use crate::{
    helper, logger,
    structs::{GlobleError, Item, Order, OrderByItem, Ordres},
};
use polars::{prelude::*, series::Series};

use reqwest::{Client, Method, Url};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

static API_ENDPOINT: &str = "https://api.warframe.market/v1/";
static LOG_FILE: &str = "wfmAPICalls.log";

async fn send_request<T: DeserializeOwned>(
    method: Method,
    url: &str,
    jwt_token: &str,
    payload_key: Option<&str>,
    body: Option<Value>,
) -> Result<T, GlobleError> {
    // Sleep for 1 seconds before sending a new request, to avoid 429 error
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    let client = Client::new();
    let new_url = format!("{}{}", API_ENDPOINT, url);

    let request = client
        .request(method, Url::parse(&new_url).unwrap())
        .header("Authorization", format!("JWT {}", jwt_token))
        .header("Language", "en");

    let request = match body {
        Some(content) => request.json(&content),
        None => request,
    };
    // let response: Value = request.send().await?.json().await;
    let response = request.send().await;
    if let Err(e) = response {
        return Err(GlobleError::ReqwestError(e));
    }
    let response_data = response.unwrap();
    let status = response_data.status();

    // println!("Url: {}, Status: {}", new_url, status);

    if status == 429 {
        // Sleep for 3 second
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        return Err(GlobleError::TooManyRequests(
            "Too Many Requests".to_string(),
        ));
    }
    if status != 200 {
        return Err(GlobleError::OtherError(response_data.text().await.unwrap()));
    }

    let response = response_data.json::<Value>().await.unwrap();

    if let Some(payload_key) = payload_key {
        let payload: T = serde_json::from_value(response["payload"][payload_key].clone()).unwrap();
        Ok(payload)
    } else {
        let payload: T = serde_json::from_value(response["payload"].clone()).unwrap();
        Ok(payload)
    }
}

async fn get<T: DeserializeOwned>(
    url: &str,
    jwt_token: &str,
    payload_key: Option<&str>,
) -> Result<T, GlobleError> {
    let payload: T = send_request(Method::GET, url, jwt_token, payload_key, None).await?;
    Ok(payload)
}

async fn post<T: DeserializeOwned>(
    url: &str,
    jwt_token: &str,
    payload_key: Option<&str>,
    body: Value,
) -> Result<T, GlobleError> {
    let payload: T = send_request(Method::POST, url, jwt_token, payload_key, Some(body)).await?;
    Ok(payload)
}

async fn delete<T: DeserializeOwned>(
    url: &str,
    jwt_token: &str,
    payload_key: Option<&str>,
) -> Result<T, GlobleError> {
    let payload: T = send_request(Method::DELETE, url, jwt_token, payload_key, None).await?;
    Ok(payload)
}

async fn put<T: DeserializeOwned>(
    url: &str,
    jwt_token: &str,
    payload_key: Option<&str>,
    body: Value,
) -> Result<T, GlobleError> {
    let payload: T = send_request(Method::PUT, url, jwt_token, payload_key, Some(body)).await?;
    Ok(payload)
}

// Warframe API functions
// Get tradable items from warframe market
pub async fn get_tradable_items(jwt_token: &str) -> Result<Vec<Item>, GlobleError> {
    let payload: Vec<Item> = get("items", jwt_token, Some("items")).await?;
    Ok(payload)
}

// Create order on warframe market
pub async fn post_ordre(
    jwt_token: &str,
    item_name: &str,
    item_id: &str,
    order_type: &str,
    platinum: i64,
    quantity: i64,
    visible: bool,
    rank: Option<f64>,
) -> Result<Order, GlobleError> {
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
    match post("profile/orders", jwt_token, Some("order"), body).await {
        Ok(order) => {
            logger::info("WarframeMarket:PostOrder", format!("Created Order: {}, Item Name: {}, Item Id: {},  Platinum: {}, Quantity: {}, Visible: {}", order_type, item_name, item_id ,platinum ,quantity ,visible).as_str(), true, Some(LOG_FILE));
            Ok(order)
        }
        Err(e) => {
            logger::error(
                "WarframeMarket:PostOrder",
                format!("{:?}", e).as_str(),
                true,
                Some(LOG_FILE),
            );
            Err(e)
        }
    }
}
// Get orders from warframe market
pub async fn get_user_ordres(jwt_token: &str, ingame_name: &str) -> Result<Ordres, GlobleError> {
    let url = format!("profile/{}/orders", ingame_name);
    let orders: Ordres = get(&url, jwt_token, None).await?;
    Ok(orders)
}

// Get orders from a specific item
pub async fn get_ordres_by_item(jwt_token: &str, item: &str) -> Result<DataFrame, GlobleError> {
    let url = format!("items/{}/orders", item);
    let orders: Result<Vec<OrderByItem>, GlobleError> = get(&url, jwt_token, Some("orders")).await;

    let orders = match orders {
        Ok(orders) => orders,
        Err(e) => {
            println!("Error: {:?}", e);
            vec![]
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

    let orders: Vec<OrderByItem> = orders
        .into_iter()
        .filter(|order| order.user.status == "ingame" && order.mod_rank == mod_rank)
        .collect();

    // Check if mod_rank is null
    let orders_df = DataFrame::new_no_checks(vec![
        Series::new(
            "username",
            orders
                .iter()
                .map(|order| order.user.ingame_name.clone())
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
            "mod_rank",
            orders
                .iter()
                .map(|order| order.mod_rank.clone())
                .collect::<Vec<_>>(),
        ),
        Series::new(
            "username",
            orders
                .iter()
                .map(|order| order.user.ingame_name.clone())
                .collect::<Vec<_>>(),
        ),
        Series::new(
            "order_type",
            orders
                .iter()
                .map(|order| order.order_type.clone())
                .collect::<Vec<_>>(),
        ),
    ]);
    Ok(orders_df)
}

/// Converts an Order object to a DataFrame object and returns it.
/// The `order` argument is an Order object containing data to be converted to a DataFrame.
/// The resulting DataFrame has columns for "id", "visible", "url_name", "platinum", "platform", "quantity", "last_update", and "creation_date".
pub fn convet_order_to_datafream(order: Order) -> Result<DataFrame, GlobleError> {
    let orders_df = DataFrame::new_no_checks(vec![
        Series::new("id", vec![order.id.clone()]),
        Series::new("visible", vec![order.visible.clone()]),
        Series::new("url_name", vec![order.item.url_name.clone()]),
        Series::new("platinum", vec![order.platinum.clone()]),
        Series::new("platform", vec![order.platform.clone()]),
        Series::new("quantity", vec![order.quantity.clone()]),
        Series::new("last_update", vec![order.last_update.clone()]),
        Series::new("creation_date", vec![order.creation_date.clone()]),
    ]);
    Ok(orders_df)
}
// Get orders from a specific user
pub async fn get_ordres_data_frames(
    jwt_token: &str,
    ingame_name: &str,
) -> Result<(DataFrame, DataFrame), GlobleError> {
    let current_orders = get_user_ordres(jwt_token, ingame_name).await?;
    let buy_orders = current_orders.buy_orders.clone();
    let my_buy_orders_df = DataFrame::new_no_checks(vec![
        // Assuming Order has fields field1, field2, ...
        Series::new(
            "id",
            buy_orders
                .iter()
                .map(|order| order.id.clone())
                .collect::<Vec<_>>(),
        ),
        Series::new(
            "visible",
            buy_orders
                .iter()
                .map(|order| order.visible.clone())
                .collect::<Vec<_>>(),
        ),
        Series::new(
            "url_name",
            buy_orders
                .iter()
                .map(|order| order.item.url_name.clone())
                .collect::<Vec<_>>(),
        ),
        Series::new(
            "platinum",
            buy_orders
                .iter()
                .map(|order| order.platinum.clone())
                .collect::<Vec<_>>(),
        ),
        Series::new(
            "platform",
            buy_orders
                .iter()
                .map(|order| order.platform.clone())
                .collect::<Vec<_>>(),
        ),
        Series::new(
            "quantity",
            buy_orders
                .iter()
                .map(|order| order.quantity.clone())
                .collect::<Vec<_>>(),
        ),
        Series::new(
            "last_update",
            buy_orders
                .iter()
                .map(|order| order.last_update.clone())
                .collect::<Vec<_>>(),
        ),
        Series::new(
            "creation_date",
            buy_orders
                .iter()
                .map(|order| order.creation_date.clone())
                .collect::<Vec<_>>(),
        ),
    ]);
    let sell_orders = current_orders.sell_orders.clone();
    let my_sell_orders_df = DataFrame::new_no_checks(vec![
        Series::new(
            "id",
            sell_orders
                .iter()
                .map(|order| order.id.clone())
                .collect::<Vec<_>>(),
        ),
        Series::new(
            "visible",
            sell_orders
                .iter()
                .map(|order| order.visible.clone())
                .collect::<Vec<_>>(),
        ),
        Series::new(
            "url_name",
            sell_orders
                .iter()
                .map(|order| order.item.url_name.clone())
                .collect::<Vec<_>>(),
        ),
        Series::new(
            "platinum",
            sell_orders
                .iter()
                .map(|order| order.platinum.clone())
                .collect::<Vec<_>>(),
        ),
        Series::new(
            "platform",
            buy_orders
                .iter()
                .map(|order| order.platform.clone())
                .collect::<Vec<_>>(),
        ),
        Series::new(
            "quantity",
            buy_orders
                .iter()
                .map(|order| order.quantity.clone())
                .collect::<Vec<_>>(),
        ),
        Series::new(
            "last_update",
            buy_orders
                .iter()
                .map(|order| order.last_update.clone())
                .collect::<Vec<_>>(),
        ),
        Series::new(
            "creation_date",
            buy_orders
                .iter()
                .map(|order| order.creation_date.clone())
                .collect::<Vec<_>>(),
        ),
    ]);
    Ok((my_buy_orders_df, my_sell_orders_df))
}

// Update order on warframe market
pub async fn update_order_listing(
    jwt_token: &str,
    order_id: &str,
    platinum: i64,
    quantity: i64,
    visible: bool,
    item_name: &str,
    item_id: &str,
    order_type: &str,
) -> Result<Order, GlobleError> {
    // Construct any JSON body
    let body = json!({
        "platinum": platinum,
        "quantity": quantity,
        "visible": visible
    });
    let url = format!("profile/orders/{}", order_id);
    match put(&url, jwt_token, Some("order"), body).await {
        Ok(order) => {
            logger::info("WarframeMarket:UpdateOrderListing", format!("Updated Order Id: {}, Item Name: {}, Item Id: {}, Platinum: {}, Quantity: {}, Visible: {}, Type: {}", order_id, item_name, item_id,platinum ,quantity ,visible, order_type).as_str(), true, Some(LOG_FILE));
            Ok(order)
        }
        Err(e) => {
            logger::error(
                "WarframeMarket:UpdateOrderListing",
                format!("{:?}", e).as_str(),
                true,
                Some(LOG_FILE),
            );
            Err(e)
        }
    }
}

// Delete order from warframe market
pub async fn delete_order(
    jwt_token: &str,
    order_id: &str,
    item_name: &str,
    item_id: &str,
    order_type: &str,
) -> Result<String, GlobleError> {
    let url = format!("profile/orders/{}", order_id);
    match delete(&url, jwt_token, Some("order_id")).await {
        Ok(order_id) => {
            logger::info(
                "WarframeMarket:DeleteOrder",
                format!(
                    "Deleted order: {}, Item Name: {}, Item Id: {}, Type: {}",
                    order_id, item_name, item_id, order_type
                )
                .as_str(),
                true,
                Some(LOG_FILE),
            );
            Ok(order_id)
        }
        Err(e) => {
            logger::error(
                "WarframeMarket:DeleteOrder",
                format!("{:?}", e).as_str(),
                true,
                None,
            );
            Err(e)
        }
    }
}
