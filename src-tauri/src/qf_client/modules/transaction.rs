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
pub struct TransactionModule {
    pub client: QFClient,
    component: String,
}

impl TransactionModule {
    pub fn new(client: QFClient) -> Self {
        TransactionModule {
            client,
            component: "Transaction".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }
    fn update_state(&self) {
        self.client.update_transaction_module(self.clone());
    }
    pub async fn create_transaction(
        &self,
        transaction: &entity::transaction::transaction::Model,
    ) -> Result<(), AppError> {
        let settings = self.client.settings.lock()?.clone();
        let analytics = settings.analytics;

        if !analytics.transaction {
            return Ok(());            
        }

        match self.client.post::<Value>("stats/transaction/add", json!(transaction)).await {
            Ok(ApiResult::Success(_, _)) => {
                return Ok(());
            }
            Ok(ApiResult::Error(e, _headers)) => {
                return Err(self.client.create_api_error(
                    &self.get_component("Login"),
                    e,
                    eyre!("There was an error logging in"),
                    LogLevel::Error,
                ));
            }
            Err(e) => return Err(e),
        }
    }
}
