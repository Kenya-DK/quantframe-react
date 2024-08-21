use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
    time::Duration,
};

use entity::transaction;
use eyre::eyre;
use serde_json::{json, Value};
use tauri::{Manager, State};
use tokio::time::Instant;

use crate::{
    app,
    qf_client::client::QFClient,
    utils::{
        enums::log_level::LogLevel,
        modules::{
            error::{self, ApiResult, AppError},
            logger,
        },
    },
    APP,
};
#[derive(Clone, Debug)]
pub struct StockItemModule {
    pub client: QFClient,
    component: String,
}

impl StockItemModule {
    pub fn new(client: QFClient) -> Self {
        StockItemModule {
            client,
            component: "StockItem".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }
    fn update_state(&self) {
        self.client.update_stock_item_module(self.clone());
    }
    pub async fn create_stock_item(
        &self,
        stock_item: &entity::stock_item::stock_item::Model,
    ) -> Result<(), AppError> {
        let settings = self.client.settings.lock()?.clone();
        let analytics = settings.analytics;

        if !analytics.stock_item {
            return Ok(());            
        }

        match self.client.post::<Value>("stats/stock_item/add", json!(stock_item)).await {
            Ok(ApiResult::Success(_, _)) => {
                return Ok(());
            }
            Ok(ApiResult::Error(e, _headers)) => {
                return Err(self.client.create_api_error(
                    &self.get_component("StockItemCreate"),
                    e,
                    eyre!("There was an error creating the stock item"),
                    LogLevel::Error,
                ));
            }
            Err(e) => return Err(e),
        }
    }
}
