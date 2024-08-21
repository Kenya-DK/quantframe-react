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
pub struct StockRivenModule {
    pub client: QFClient,
    component: String,
}

impl StockRivenModule {
    pub fn new(client: QFClient) -> Self {
        StockRivenModule {
            client,
            component: "StockRiven".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }
    fn update_state(&self) {
        self.client.update_stock_riven_module(self.clone());
    }
    pub async fn create_stock_riven(
        &self,
        stock_riven: &entity::stock_riven::stock_riven::Model,
    ) -> Result<(), AppError> {
        let settings = self.client.settings.lock()?.clone();
        let analytics = settings.analytics;

        if !analytics.stock_riven {
            return Ok(());            
        }

        match self.client.post::<Value>("stats/stock_riven/add", json!(stock_riven)).await {
            Ok(ApiResult::Success(_, _)) => {
                return Ok(());
            }
            Ok(ApiResult::Error(e, _headers)) => {
                return Err(self.client.create_api_error(
                    &self.get_component("StockRivenCreate"),
                    e,
                    eyre!("There was an error creating the stock riven"),
                    LogLevel::Error,
                ));
            }
            Err(e) => return Err(e),
        }
    }
}
