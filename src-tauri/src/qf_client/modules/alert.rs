use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use serde_json::{json, Value};
use tauri::{Manager, State};
use tokio::time::Instant;

use crate::{
    qf_client::{
        client::QFClient,
        types::{alert::Alert, paginated::Paginated},
    },
    utils::{
        enums::{
            log_level::LogLevel,
            ui_events::{UIEvent, UIOperationEvent},
        },
        modules::error::{self, ApiResult, AppError},
    },
    APP,
};
#[derive(Clone, Debug)]
pub struct AlertModule {
    pub client: QFClient,
    component: String,
    is_init: bool,
}

impl AlertModule {
    pub fn new(client: QFClient) -> Self {
        AlertModule {
            is_init: false,
            client,
            component: "Alerts".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }
    fn update_state(&self) {
        self.client.update_alert_module(self.clone());
    }
    pub fn init(&mut self) -> Result<(), AppError> {
        let notify = self.client.notify.lock().unwrap().clone();
        if self.is_init {
            return Ok(());
        }
        self.is_init = true;
        self.update_state();
        tauri::async_runtime::spawn({
            async move {
                // Create a new instance of the QFClient and store it in the app state
                let qf_handle = APP.get().expect("failed to get app handle");
                let qf_state: State<Arc<Mutex<QFClient>>> = qf_handle.state();
                let qf = qf_state.lock().expect("failed to lock app state").clone();

                // Create Timer for sending metrics
                let mut last_updated = Instant::now();

                let mut lasted_alerts: Vec<String> = Vec::new();

                let mut starting = true;

                loop {
                    if last_updated.elapsed() > Duration::from_secs(5) || starting {
                        starting = false;
                        last_updated = Instant::now();

                        match qf.alert().get_alerts().await {
                            Ok(data) => {
                                let new_uuids: Vec<String> =
                                    data.results.iter().map(|alert| alert.get_uuid()).collect();
                                let old_uuids: Vec<String> = lasted_alerts.clone();

                                let diff = new_uuids
                                    .iter()
                                    .filter(|uuid| !old_uuids.contains(uuid))
                                    .collect::<Vec<_>>();
                                if diff.len() > 0 || old_uuids.len() != new_uuids.len() {
                                    lasted_alerts = new_uuids;
                                    notify.gui().send_event_update(
                                        UIEvent::UpdateAlerts,
                                        UIOperationEvent::Set,
                                        Some(json!(data.results)),
                                    );
                                }
                            }
                            Err(e) => {
                                error::create_log_file("alerts.log".to_string(), &e);
                            }
                        };
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            }
        });
        Ok(())
    }
    pub async fn get_alerts(&self) -> Result<Paginated<Alert>, AppError> {
        match self
            .client
            .get::<Paginated<Alert>>("alert?page=0&limit=25&enabled=true", false)
            .await
        {
            Ok(ApiResult::Success(payload, _headers)) => {
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(self.client.create_api_error(
                    &self.get_component("GetAlerts"),
                    error,
                    eyre::eyre!("Failed to get alerts"),
                    LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
}
