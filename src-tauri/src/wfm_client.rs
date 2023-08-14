use std::{error::Error, io::Cursor};

use crate::structs::{GlobleError, Item, Order, OrderByItem, Ordres};
use polars::{prelude::*, series::Series};

use reqwest::{Client, Method, Url};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

static API_ENDPOINT: &str = "https://api.warframe.market/v1/";

async fn send_request<T: DeserializeOwned>(
    method: Method,
    url: &str,
    jwt_token: &str,
    payload_key: Option<&str>,
    body: Option<Value>,
) -> Result<T, GlobleError> {
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
    item: &str,
    order_type: &str,
    platinum: i64,
    quantity: i64,
    visible: bool,
    rank: Option<f64>,
) -> Result<Order, GlobleError> {
    // Construct any JSON body
    let mut body = json!({
        "item": item,
        "order_type": order_type,
        "platinum": platinum,
        "quantity": quantity,
        "visible": visible
    });
    // Add rank to body if it exists
    if let Some(rank) = rank {
        body["rank"] = json!(rank);
    }
    let order: Order = post("profile/orders", jwt_token, Some("order"), body).await?;
    Ok(order)
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
) -> Result<Order, GlobleError> {
    // Construct any JSON body
    let body = json!({
        "platinum": platinum,
        "quantity": quantity,
        "visible": visible
    });
    let url = format!("profile/orders/{}", order_id);
    let order: Order = put(&url, jwt_token, Some("order"), body).await?;

    Ok(order)
}

// Delete order from warframe market
pub async fn delete_order(jwt_token: &str, order_id: &str) -> Result<String, GlobleError> {
    let url = format!("profile/orders/{}", order_id);
    let order_id: String = delete(&url, jwt_token, Some("order_id")).await?;
    Ok(order_id)
}
