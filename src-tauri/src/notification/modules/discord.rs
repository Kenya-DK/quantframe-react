use crate::{
    database::{
        client::DBClient, enums::stock_status::StockStatus, types::price_history::PriceHistory,
    }, helper, notification::client::NotifyClient, utils::{enums::log_level::LogLevel, modules::error::AppError}, wfm_client::types::order::Order
};
use eyre::eyre;

use sea_query::{ColumnDef, Expr, Iden, InsertStatement, Query, SqliteQueryBuilder, Table, Value};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug)]
pub struct DiscordModule {
    client: NotifyClient,
    pub debug_id: String,
    component: String,
}

impl DiscordModule {
    pub fn new(client: NotifyClient) -> Self {
        DiscordModule {
            client,
            debug_id: "DiscordModule".to_string(),
            component: "GUINotification".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }
    fn update_state(&self) {
        self.client.update_discord_module(self.clone());
    }
}
