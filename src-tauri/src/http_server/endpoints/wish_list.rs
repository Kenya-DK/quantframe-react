use entity::wish_list::CreateWishListItem;
use serde_json::json;
use std::{net::TcpStream, sync::Arc};
use utils::*;
use wf_market::enums::OrderType;

use crate::{
    add_metric,
    handlers::{handle_riven_by_entity, handle_wish_list, handle_wish_list_by_entity},
    http_server::{respond_json, respond_text},
    send_event,
    types::{OperationSet, UIEvent},
};

#[derive(Debug)]
pub struct WishListRoute {}
impl WishListRoute {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }

    pub async fn handle_request(
        &self,
        method: &str,
        path: &str,
        body: &str,
        stream: &mut TcpStream,
    ) {
        let base_path = "/wish_list";
        if method == "POST" && path == base_path {
            self.handle_post(body, stream).await;
        }
    }

    async fn handle_post(&self, body: &str, stream: &mut TcpStream) {
        match serde_json::from_str::<CreateWishListItem>(body) {
            Ok(input) => {
                match handle_wish_list_by_entity(input, "", OrderType::Sell, &OperationSet::new())
                    .await
                {
                    Ok((_, updated_item)) => {
                        info(
                            "HttpServer:WishListRoute",
                            format!("Created Wish List Item: {}", updated_item.item_name),
                            &LoggerOptions::default(),
                        );
                        add_metric!("http_server_wish_list_create", "success");
                        send_event!(
                            UIEvent::RefreshWishListItems,
                            json!({"id": updated_item.id, "source": "HttpServer"})
                        );
                        respond_json(stream, 200, &serde_json::to_string(&updated_item).unwrap());
                    }
                    Err(e) => {
                        respond_json(stream, 400, json!(e).to_string().as_str());
                    }
                }
            }
            Err(e) => respond_text(stream, 400, &format!("Invalid JSON: {}", e)),
        }
    }
}
