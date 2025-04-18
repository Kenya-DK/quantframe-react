use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use eyre::eyre;
use serde_json::{json, Value};
use tokio::time::Instant;

use crate::{
    qf_client::client::QFClient,
    utils::{
        enums::log_level::LogLevel,
        modules::{
            error::{self, ApiResult, AppError},
            logger::{self, LoggerOptions},
            states,
        },
    },
};
#[derive(Clone, Debug)]
pub struct AnalyticsModule {
    pub client: QFClient,
    send_metrics: bool,
    is_init: bool,
    current_page: String,
    component: String,
    metricAndLabelPairsScheduledToSend: Vec<HashMap<String, String>>,
    last_user_activity: Arc<Mutex<Instant>>,
}

impl AnalyticsModule {
    pub fn new(client: QFClient) -> Self {
        AnalyticsModule {
            client,
            current_page: "home".to_string(),
            component: "Analytics".to_string(),
            is_init: false,
            send_metrics: true,
            last_user_activity: Arc::new(Mutex::new(Instant::now())),
            metricAndLabelPairsScheduledToSend: vec![],
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }
    fn update_state(&self) {
        self.client.update_analytics_module(self.clone());
    }
    pub fn is_user_active(&self) -> bool {
        let last_user_activity = self.last_user_activity.lock().unwrap();
        last_user_activity.elapsed() < Duration::from_secs(62)
    }
    pub fn set_current_page(&mut self, page: &str) {
        self.current_page = page.to_string();
        self.update_state();
    }
    pub fn set_last_user_activity(&mut self) {
        let mut last_user_activity = self.last_user_activity.lock().unwrap();
        *last_user_activity = Instant::now();
        self.update_state();
    }
    pub fn clear_metrics(&mut self) {
        self.metricAndLabelPairsScheduledToSend = vec![];
        self.update_state();
    }
    pub fn add_metric(&mut self, key: &str, value: &str) {
        let settings = states::settings().expect("Failed to get settings");
        if !settings.analytics.transaction && key.starts_with("Transaction_") {
            return;
        }
        let mut map = HashMap::new();
        map.insert(key.to_string(), value.to_string());
        if key == "Active_Page" {
            self.set_current_page(value);
            self.set_last_user_activity();
        }
        self.metricAndLabelPairsScheduledToSend.push(map);
        self.update_state();
    }
    pub fn init(&mut self) -> Result<(), AppError> {
        let app = states::app_state()?;
        if self.is_init {
            return Ok(());
        }
        self.is_init = true;
        self.update_state();

        let is_first_install = app.is_first_install.clone();
        tauri::async_runtime::spawn({
            async move {
                // Create a new instance of the QFClient and store it in the app state
                let qf = states::qf_client().expect("Failed to get qf client");

                // Create Timer for sending metrics
                let mut last_metric_time = Instant::now();

                if is_first_install {
                    logger::info(
                        &&qf.analytics().get_component("init"),
                        "Detected first install",
                        LoggerOptions::default(),
                    );
                    match qf
                        .analytics()
                        .try_send_analytics("install", 3, json!({}))
                        .await
                    {
                        Ok(_) => {}
                        Err(e) => {
                            error::create_log_file("analytics.log", &e);
                        }
                    };
                }
                loop {
                    let send_metrics = qf.analytics().send_metrics;
                    if !send_metrics {
                        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                        continue;
                    }
                    if last_metric_time.elapsed() > Duration::from_secs(15)
                        || qf.analytics().is_user_active()
                    {
                        if last_metric_time.elapsed() > Duration::from_secs(60)
                            && qf.analytics().is_user_active()
                        {
                            continue;
                        }

                        last_metric_time = Instant::now();
                        // logger::info_con(
                        //     &qf.analytics().get_component("TrySendAnalytics"),
                        //     "Sending user activity",
                        // );
                        match qf
                            .analytics()
                            .try_send_analytics(
                                "metrics/periodic",
                                3,
                                json!(qf.analytics().metricAndLabelPairsScheduledToSend),
                            )
                            .await
                        {
                            Ok(_) => {
                                qf.analytics().clear_metrics();
                            }
                            Err(e) => {
                                if e.cause().contains("Unauthorized")
                                    || e.cause().contains("Banned")
                                    || e.cause().contains("WFMBanned")
                                {
                                    error::create_log_file("analytics.log", &e);
                                    break;
                                }
                                error::create_log_file("analytics.log", &e);
                            }
                        };
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
                qf.analytics().is_init = false;
            }
        });
        Ok(())
    }
    pub fn set_send_metrics(&mut self, send_metrics: bool) {
        self.send_metrics = send_metrics;
        self.update_state();
    }
    pub async fn try_send_analytics(
        &self,
        url: &str,
        mut retry_count: i64,
        data: Value,
    ) -> Result<(), AppError> {
        let mut parameters: Vec<String> = vec![];
        if self.is_user_active() {
            parameters.push(format!("Active_Page={}", self.current_page));
        }

        while retry_count >= 0 {
            let err = match self
                .client
                .post::<Value>(
                    format!("analytics/{}?{}", url, parameters.join("&")).as_str(),
                    data.clone(),
                )
                .await
            {
                Ok(ApiResult::Success(_, _)) => {
                    return Ok(());
                }
                Ok(ApiResult::Error(e, _headers)) => AppError::new_api(
                    &self.get_component("TrySendAnalytics"),
                    e,
                    eyre!("Failed to send analytics"),
                    LogLevel::Error,
                ),
                Err(e) => e,
            };
            if retry_count == 0 {
                return Err(err);
            }
            retry_count -= 1;
            logger::warning(
                &self.get_component("TrySendAnalytics"),
                &format!(
                    "Failed to send analytics, retrying in 5 seconds, retries left: {}",
                    retry_count
                ),
                LoggerOptions::default(),
            );
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
        Ok(())
    }
}
