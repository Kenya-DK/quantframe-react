use entity::{dto::*, stock_item::*};
use std::{io::Write, net::TcpStream, sync::Arc};
use utils::get_location;
use wf_market::enums::OrderType;

use crate::{
    enums::FindByType,
    handlers::handle_item_by_entity,
    http_server::{respond_json, respond_text},
};
#[derive(Debug)]
pub struct StockItemRoute {}
impl StockItemRoute {
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
        let base_path = "/stock_item";
        if method == "POST" && path == base_path {
            self.handle_post(body, stream).await;
        }
    }

    async fn handle_post(&self, body: &str, stream: &mut TcpStream) {
        match serde_json::from_str::<CreateStockItem>(body) {
            Ok(input) => {
                match handle_item_by_entity(input, "", OrderType::Buy, FindByType::Url, &[]).await {
                    Ok((_, updated_item)) => {
                        respond_json(stream, 200, &serde_json::to_string(&updated_item).unwrap());
                    }
                    Err(e) => {
                        respond_text(
                            stream,
                            400,
                            &format!(
                                "Error processing item: {}",
                                e.with_location(get_location!())
                            ),
                        );
                    }
                }
            }
            Err(e) => respond_text(stream, 400, &format!("Invalid JSON: {}", e)),
        }
    }
}
