use entity::stock_riven::CreateStockRiven;
use qf_api::enums::app_events::ApplicationEvent as EventType;
use serde_json::json;
use std::{net::TcpStream, sync::Arc};
use utils::*;
use wf_market::enums::OrderType;

use crate::{
    handlers::handle_riven_by_entity,
    http_server::{respond_json, respond_text},
    send_event,
    track_event,
    types::UIEvent,
};

#[derive(Debug)]
pub struct StockRivenRoute {}
impl StockRivenRoute {
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
        let base_path = "/stock_riven";
        if method == "POST" && path == base_path {
            self.handle_post(body, stream).await;
        }
    }

    async fn handle_post(&self, body: &str, stream: &mut TcpStream) {
        match serde_json::from_str::<CreateStockRiven>(body) {
            Ok(input) => match handle_riven_by_entity(input, "", OrderType::Buy, &OperationSet::new())
                .await
            {
                Ok((_, updated_item)) => {
                    info(
                        "HttpServer:StockRivenRoute",
                        format!(
                            "Created Riven: {} {}",
                            updated_item.weapon_name, updated_item.mod_name
                        ),
                        &LoggerOptions::default(),
                    );
                    track_event!(EventType::StockRivenCreate, [("success", "true")]);
                    send_event!(
                        UIEvent::RefreshStockRivens,
                        json!({"id": updated_item.id, "source": "HttpServer"})
                    );
                    if updated_item.bought > 0 {
                        send_event!(
                            UIEvent::RefreshTransactions,
                            json!({"id": updated_item.id, "source": "HttpServer"})
                        );
                    }
                    respond_json(stream, 200, &serde_json::to_string(&updated_item).unwrap());
                }
                Err(e) => {
                    respond_json(stream, 400, json!(e).to_string().as_str());
                }
            },
            Err(e) => respond_text(stream, 400, &format!("Invalid JSON: {}", e)),
        }
    }
}
