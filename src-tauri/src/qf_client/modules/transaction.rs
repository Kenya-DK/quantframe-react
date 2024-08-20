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
    is_init: bool,
    current_page: String,
    component: String,
    metricAndLabelPairsScheduledToSend: Vec<HashMap<String, String>>,
    last_user_activity: Arc<Mutex<Instant>>,
}

impl TransactionModule {
    pub fn new(client: QFClient) -> Self {
        TransactionModule {
            client,
            current_page: "home".to_string(),
            component: "Transaction".to_string(),
            is_init: false,
            last_user_activity: Arc::new(Mutex::new(Instant::now())),
            metricAndLabelPairsScheduledToSend: vec![],
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
        transaction: entity::transaction::transaction::Model,
    ) -> Result<(), AppError> {
        let settings = self.client.settings.lock()?.clone();
        
        match self.client.analytics().try_send_analytics("transaction/add", 3, json!(transaction)).await {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {                
                return Err(e);
            }
        };
    }
}
