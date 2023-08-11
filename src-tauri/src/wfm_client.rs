use reqwest::{Url, Client, Method, Response};
use serde_json::{Value, json};
use crate::structs::{Order, Item, Ordres};

pub static API_ENDPOINT: &str = "https://api.warframe.market/v1/";

pub async fn send_request(method: Method, url: &str, jwt_token: &str, body: Option<Value>) -> Result<Response, reqwest::Error> {
    let client = Client::new();

    let request = client.request(method, Url::parse(url).unwrap())
        .header("Authorization", format!("JWT {}", jwt_token))
        .header("Language", "en");

    let request = match body {
        Some(content) => request.json(&content),
        None => request
    };

    request.send().await
}

pub async fn get(url: &str, jwt_token: &str) -> Result<Response, reqwest::Error> {
    send_request(Method::GET, url, jwt_token, None).await
}

pub async fn post(url: &str, jwt_token: &str, body: Value) -> Result<Response, reqwest::Error> {
    send_request(Method::POST, url, jwt_token, Some(body)).await
}

pub async fn delete(url: &str, jwt_token: &str) -> Result<Response, reqwest::Error> {
    send_request(Method::DELETE, url, jwt_token, None).await
}
pub async fn put(url: &str, jwt_token: &str, body: Value) -> Result<Response, reqwest::Error> {
    send_request(Method::PUT, url, jwt_token, Some(body)).await
}
// Warframe API functions
// Get tradable items from warframe market
pub async fn get_tradable_items(jwt_token: &str, platform: &str) -> Result<Vec<Item>, reqwest::Error> {
    let response =  get(concat!(API_ENDPOINT,"items"), jwt_token).await?.json().await?
    let item_list = response.payload.items;
    Ok(item_list)
}

// Create order on warframe market
pub async fn post_ordre(jwt_token: &str, item: &str, order_type: &str, platinum: &i32, quantity: &i32, visible: &bool, rank: Option< &i32>) -> Result<Order, reqwest::Error> {
        // Construct any JSON body
        let body = json!({
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
    let response =  post(concat!(API_ENDPOINT,"profile/orders"), jwt_token, body).await?.json().await?
    let order = response.payload.order;
    Ok(order)
}
// Get orders from warframe market
pub async fn get_ordres(jwt_token: &str, ingame_name: &str) -> Result<Ordres, reqwest::Error> {
        // Construct any JSON body
        let body = json!({
            "item": item,
            "order_type": order_type,
            "platinum": platinum,
            "quantity": quantity,
            "visible": visible,
            "rank": rank
        });
    let response = get(concat!(API_ENDPOINT,"profile/",ingame_name,"/orders"), jwt_token).await?.json().await?
    let orders = response.payload;
    Ok(orders)
}

// Update order on warframe market
pub async fn update_order_listing(jwt_token: &str, order_id: &str, platinum: &i32, quantity: &i32, visible: &bool) -> Result<Ordres, reqwest::Error> {
        // Construct any JSON body
        let body = json!({
            "platinum": item,
            "quantity": order_type,
            "visible": visible
        });
    let response = put(concat!(API_ENDPOINT,"profile/orders/", order_id), jwt_token).await?.json().await?
    let orders = response.payload;
    Ok(orders)
}

// Delete order from warframe market
pub async fu delete_order(jwt_token: &str, order_id: &str) -> Result<Ordres, reqwest::Error> {
    let response = delete(concat!(API_ENDPOINT,"profile/orders/", order_id), jwt_token).await?.json().await?
    let orders = response.payload;
    Ok(orders)
}
