use std::{
    collections::HashMap,
    sync::{Arc, Mutex, Weak},
    time::{Duration, Instant},
};

use reqwest::Method;
use serde_json::{Value, json};

use crate::{client::Client, enums::ResponseFormat, errors::ApiError};

#[derive(Debug)]
pub struct AnalyticsRoute {
    _metrics: Mutex<Vec<HashMap<String, String>>>,
    _last_user_activity: Mutex<Instant>,
    _stop: Arc<Mutex<bool>>,
    client: Weak<Client>,
}

impl AnalyticsRoute {
    /**
     * Creates a new `AnalyticsRoute` with an empty Authentication list.
     * The `client` parameter is an `Arc<Client<State>>` that allows the route
     */
    pub fn new(client: Arc<Client>) -> Arc<Self> {
        Arc::new(Self {
            _last_user_activity: Mutex::new(Instant::now()),
            _metrics: Mutex::new(Vec::new()),
            _stop: Arc::new(Mutex::new(false)),
            client: Arc::downgrade(&client),
        })
    }

    pub fn set_last_user_activity(&self) {
        let mut last_activity = self._last_user_activity.lock().unwrap();
        *last_activity = Instant::now();
    }

    pub fn is_active(&self) -> bool {
        let last_activity = self._last_user_activity.lock().unwrap();
        last_activity.elapsed().as_secs() < 60 // Active if last activity was within the last 60 seconds
    }

    pub async fn send_current_metrics(&self) -> Result<(), ApiError> {
        let metrics = self._metrics.lock().unwrap().clone();
        if metrics.is_empty() {
            return Ok(()); // No metrics to send
        }
        self.try_send_analytics("/users/metrics/periodic", 3, json!(metrics))
            .await
    }

    pub fn start(self: Arc<Self>) -> Result<(), ApiError> {
        let mut stop = self._stop.lock().unwrap();
        *stop = false;
        tokio::spawn({
            let this = Arc::clone(&self);
            async move {
                // Create Timer for sending metrics
                let mut last_metric_time = Instant::now();
                loop {
                    if *this._stop.lock().unwrap() {
                        break;
                    }
                    if last_metric_time.elapsed() < Duration::from_secs(30) || !this.is_active() {
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                        continue;
                    }
                    last_metric_time = Instant::now();

                    match this.send_current_metrics().await {
                        Ok(_) => {
                            this._metrics.lock().unwrap().clear();
                        }
                        Err(e) => {
                            eprintln!("Failed to send metrics: {}", e);
                        }
                    };
                    // Sleep for a while before checking again
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        });
        Ok(())
    }

    pub fn stop(&self) {
        let mut stop = self._stop.lock().unwrap();
        *stop = true;
    }

    pub fn add_metric(&self, key: impl Into<String>, value: impl Into<String>) {
        let mut metrics = self._metrics.lock().unwrap();
        metrics.push(HashMap::from([(key.into(), value.into())]));
    }

    pub async fn try_send_analytics(
        &self,
        url: &str,
        mut retry_count: i64,
        data: Value,
    ) -> Result<(), ApiError> {
        let client = self.client.upgrade().expect("Client should not be dropped");

        while retry_count >= 0 {
            let err = match client
                .call_api::<Value>(
                    Method::POST,
                    format!("{}", url).as_str(),
                    Some(data.clone()),
                    None,
                    ResponseFormat::Json,
                )
                .await
            {
                Ok(_) => {
                    return Ok(());
                }
                Err(e) => e,
            };
            if let ApiError::UserBanned(_) = &err {
                let mut stop = self._stop.lock().unwrap();
                *stop = true;
                return Err(err);
            }

            if retry_count == 0 {
                return Err(err);
            }
            retry_count -= 1;
            println!(
                "Retrying to send analytics data, attempts left: {}",
                retry_count
            );
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
        Ok(())
    }

    /**
     * Creates a new `AnalyticsRoute` from an existing one, sharing the client.
     * This is useful for cloning routes when the client state changes.
     */
    pub fn from_existing(old: &AnalyticsRoute, client: Arc<Client>) -> Arc<Self> {
        Arc::new(Self {
            _last_user_activity: Mutex::new(*old._last_user_activity.lock().unwrap()),
            _stop: old._stop.clone(),
            _metrics: Mutex::new(old._metrics.lock().unwrap().clone()),
            client: Arc::downgrade(&client),
        })
    }
}
