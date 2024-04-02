use crate::{
    database::{
        client::DBClient, enums::stock_status::StockStatus, types::price_history::PriceHistory,
    },
    helper,
    notification::client::NotifyClient,
    utils::{enums::log_level::LogLevel, modules::error::AppError},
    wfm_client::types::order::Order,
};
use eyre::eyre;

use sea_query::{ColumnDef, Expr, Iden, InsertStatement, Query, SqliteQueryBuilder, Table, Value};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::{api::notification::Notification, AppHandle};

#[derive(Clone, Debug)]
pub struct SystemModule {
    client: NotifyClient,
    pub debug_id: String,
    component: String,
}

impl SystemModule {
    pub fn new(client: NotifyClient) -> Self {
        SystemModule {
            client,
            debug_id: "SystemModule".to_string(),
            component: "SystemNotification".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }
    fn update_state(&self) {
        self.client.update_system_module(self.clone());
    }

    pub fn send_notification(
        &self,
        title: &str,
        body: &str,
        icon: Option<&str>,
        sound: Option<&str>,
    ) {
        let sound = match sound {
            Some(s) => s,
            None => "Default",
        };
        let notification =
            Notification::new(&self.client.app_handler.config().tauri.bundle.identifier)
                .title(title)
                .body(body)
                .icon(icon.unwrap_or("assets/icons/icon.png"))
                .sound(sound);
        notification.show().unwrap();
    }
}
