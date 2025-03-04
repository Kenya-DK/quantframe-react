use std::{
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};

use eyre::eyre;
use reqwest::{Client, Method, Url};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    app::client::AppState,
    auth::AuthState,
    logger,
    notification::client::NotifyClient,
    utils::{
        enums::{
            log_level::LogLevel,
            ui_events::{UIEvent, UIOperationEvent},
        },
        modules::{
            error::{ApiResult, AppError, ErrorApiResponse},
            logger::LoggerOptions,
            rate_limiter::RateLimiter,
            states,
        },
    },
};

use super::modules::{
    alert::AlertModule, analytics::AnalyticsModule, auth::AuthModule, cache::CacheModule,
    item::ItemModule, riven::RivenModule, transaction::TransactionModule,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ByteResponse {
    #[serde(rename = "data")]
    pub data: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct QFClient {
    endpoint: String,
    endpoint_dev: String,
    is_dev: bool,
    limiter: Arc<tokio::sync::Mutex<RateLimiter>>,
    auth_module: Arc<RwLock<Option<AuthModule>>>,
    cache_module: Arc<RwLock<Option<CacheModule>>>,
    item_module: Arc<RwLock<Option<ItemModule>>>,
    riven_module: Arc<RwLock<Option<RivenModule>>>,
    analytics_module: Arc<RwLock<Option<AnalyticsModule>>>,
    alert_module: Arc<RwLock<Option<AlertModule>>>,
    transaction_module: Arc<RwLock<Option<TransactionModule>>>,
    pub component: String,
    pub log_file: &'static str,
}

impl QFClient {
    pub fn new() -> Self {
        QFClient {
            endpoint: "https://api.quantframe.app/".to_string(),
            endpoint_dev: "http://localhost:6969/".to_string(),
            is_dev: if cfg!(debug_assertions) { true } else { false },
            limiter: Arc::new(tokio::sync::Mutex::new(RateLimiter::new(
                3.0,
                Duration::new(1, 0),
            ))),
            auth_module: Arc::new(RwLock::new(None)),
            cache_module: Arc::new(RwLock::new(None)),
            item_module: Arc::new(RwLock::new(None)),
            riven_module: Arc::new(RwLock::new(None)),
            alert_module: Arc::new(RwLock::new(None)),
            analytics_module: Arc::new(RwLock::new(None)),
            transaction_module: Arc::new(RwLock::new(None)),
            log_file: "qfAPIaCalls.log",
            component: "QuantframeApi".to_string(),
        }
    }
    pub fn create_api_error(
        &self,
        component: &str,
        err: ErrorApiResponse,
        eyre_report: eyre::ErrReport,
        level: LogLevel,
    ) -> AppError {
        return AppError::new_api(
            format!("{}:{}", self.component, component).as_str(),
            err,
            eyre_report,
            level,
        );
    }
    pub fn debug(&self, id: &str, component: &str, msg: &str, file: Option<bool>) {
        let settings = states::settings().expect("Settings not initialized");
        if !settings.debug.contains(&"*".to_owned()) && !settings.debug.contains(&id.to_owned()) {
            return;
        }

        if file.is_none() {
            logger::debug(
                format!("{}:{}", self.component, component).as_str(),
                msg,
                LoggerOptions::default(),
            );
            return;
        }
        logger::debug(
            format!("{}:{}", self.component, component).as_str(),
            msg,
            LoggerOptions::default().set_file(self.log_file),
        );
    }
    fn handle_error(&self, errors: Vec<String>, data: Value) {
        let mut auth = states::auth().expect("Auth not initialized");
        let notify = states::notify_client().expect("NotifyClient not initialized");
        if errors.contains(&"Unauthorized".to_string()) {
            auth.reset();
            notify.gui().send_event_update(
                UIEvent::UpdateUser,
                UIOperationEvent::Set,
                Some(json!(&auth.clone())),
            );
        } else if errors.contains(&"Banned".to_string()) {
            let reason = data
                .get("banned_reason")
                .unwrap()
                .as_str()
                .unwrap_or_default();
            let until = data
                .get("banned_until")
                .unwrap()
                .as_str()
                .unwrap_or_default();
            auth.ban_user_qf(reason, until);
            notify.gui().send_event_update(
                UIEvent::UpdateUser,
                UIOperationEvent::Set,
                Some(json!(&auth.clone())),
            );
        }
    }
    async fn send_request<T: DeserializeOwned>(
        &self,
        method: Method,
        url: &str,
        body: Option<Value>,
        is_bytes: bool,
        is_string: bool,
    ) -> Result<ApiResult<T>, AppError> {
        let app = states::app_state()?;
        let auth = states::auth()?;

        let mut rate_limiter = self.limiter.lock().await;
        rate_limiter.wait_for_token().await;

        let packageinfo = app.get_app_info();

        let client = Client::new();
        let base_url = if self.is_dev {
            self.endpoint_dev.clone()
        } else {
            self.endpoint.clone()
        };
        let new_url = format!("{}{}", base_url, url);
        let request = client
            .request(method.clone(), Url::parse(&new_url).unwrap())
            .header(
                "Authorization",
                format!(
                    "JWT {}",
                    auth.qf_access_token.clone().unwrap_or("".to_string())
                ),
            )
            .header("AppId", app.app_id.to_string())
            .header("IsDevelopment", app.is_development.to_string())
            .header("App", packageinfo.name.to_string())
            .header("Device", auth.get_device_id())
            .header("Version", packageinfo.version.to_string())
            .header("wfm_user_name", auth.ingame_name)
            .header("Platform", "PC".to_string())
            .header("wfm_id", auth.id);

        let request = match body.clone() {
            Some(content) => request.json(&content),
            None => request,
        };

        // let response: Value = request.send().await?.json().await;
        let response = request.send().await;

        // Create default error response
        let mut error_def = ErrorApiResponse {
            status_code: 500,
            error: "UnknownError".to_string(),
            messages: vec![],
            raw_response: None,
            body: body.clone(),
            url: Some(new_url.clone()),
            method: Some(method.to_string()),
        };

        if let Err(e) = response {
            error_def.messages.push(e.to_string());
            return Err(AppError::new_api(
                self.component.as_str(),
                error_def,
                eyre!(format!("There was an error sending the request: {}", e)),
                LogLevel::Critical,
            ));
        }
        let response_data = response.unwrap();
        error_def.status_code = response_data.status().as_u16() as i64;
        let headers = response_data.headers().clone();

        let mut content = if is_bytes {
            let content_bytes = response_data.bytes().await.unwrap_or_default();
            format!("{{\"data\": {:?}}}", content_bytes.to_vec())
        } else {
            response_data.text().await.unwrap_or_default()
        };

        // If T is a string, return the content as a string
        if is_string {
            content = Value::String(content).to_string();
        }
        error_def.raw_response = Some(content.clone());

        // Parse the response into a Value object
        let response: Value = match serde_json::from_str(content.as_str()) {
            Ok(val) => val,
            Err(e) => {
                error_def.messages.push(e.to_string());

                return Err(AppError::new_api(
                    self.component.as_str(),
                    error_def,
                    eyre!(format!("Could not parse response: {}, {:?}", content, e)),
                    LogLevel::Critical,
                ));
            }
        };

        // Check if response is an error
        if auth.wfm_banned {
            error_def.error = "WFMBanned".to_string();
            error_def.messages.push("WFMBanned".to_string());
            self.handle_error(error_def.messages.clone(), response);
            return Ok(ApiResult::Error(error_def, headers));
        }
        if response.get("error").is_some() || error_def.status_code == 401 {
            error_def.error = "ApiError".to_string();

            let error = if response.get("error").is_some() {
                response.get("error").unwrap().as_str().unwrap_or_default()
            } else if error_def.status_code == 401 {
                "Unauthorized"
            } else {
                "UnknownError"
            };

            let messages = response.get("message");
            if error == "banned" {
                error_def.messages.push("Banned".to_string());
            } else if error == "Unauthorized" {
                error_def.messages.push("Unauthorized".to_string());
            }
            if messages.is_some() {
                let msg = messages.unwrap();
                if msg.is_string() {
                    error_def.messages.push(msg.as_str().unwrap().to_string());
                } else if msg.is_array() {
                    let msgs = msg.as_array().unwrap();
                    for m in msgs {
                        error_def.messages.push(m.to_string());
                    }
                }
            }
            self.handle_error(error_def.messages.clone(), response);
            return Ok(ApiResult::Error(error_def, headers));
        }

        // Convert the response to a T object
        match serde_json::from_value(response.clone()) {
            Ok(payload) => Ok(ApiResult::Success(payload, headers)),
            Err(e) => {
                error_def.messages.push(e.to_string());
                return Err(AppError::new_api(
                    "QuantframeApi",
                    error_def,
                    eyre!(format!("Could not parse response: {}, {:?}", content, e)),
                    LogLevel::Critical,
                ));
            }
        }
    }

    pub async fn get_bytes(&self, url: &str) -> Result<ApiResult<Vec<u8>>, AppError> {
        match self
            .send_request::<ByteResponse>(Method::GET, url, None, true, false)
            .await
        {
            Ok(ApiResult::Success(payload, _headers)) => {
                return Ok(ApiResult::Success(payload.data, _headers));
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(AppError::new_api(
                    self.component.as_str(),
                    error,
                    eyre!("There was an error fetching the bytes"),
                    LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        };
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        url: &str,
        is_string: bool,
    ) -> Result<ApiResult<T>, AppError> {
        Ok(self
            .send_request(Method::GET, url, None, false, is_string)
            .await?)
    }

    pub async fn post<T: DeserializeOwned>(
        &self,
        url: &str,
        body: Value,
    ) -> Result<ApiResult<T>, AppError> {
        Ok(self
            .send_request(Method::POST, url, Some(body), false, false)
            .await?)
    }

    pub async fn delete<T: DeserializeOwned>(&self, url: &str) -> Result<ApiResult<T>, AppError> {
        Ok(self
            .send_request(Method::DELETE, url, None, false, false)
            .await?)
    }
    pub async fn put<T: DeserializeOwned>(
        &self,
        url: &str,
        body: Option<Value>,
    ) -> Result<ApiResult<T>, AppError> {
        Ok(self
            .send_request(Method::PUT, url, body, false, false)
            .await?)
    }
    pub fn cache(&self) -> CacheModule {
        // Lazily initialize ItemModule if not already initialized
        if self.cache_module.read().unwrap().is_none() {
            *self.cache_module.write().unwrap() = Some(CacheModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the cache_module is initialized
        self.cache_module.read().unwrap().as_ref().unwrap().clone()
    }

    pub fn item(&self) -> ItemModule {
        // Lazily initialize ItemModule if not already initialized
        if self.item_module.read().unwrap().is_none() {
            *self.item_module.write().unwrap() = Some(ItemModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the item_module is initialized
        self.item_module.read().unwrap().as_ref().unwrap().clone()
    }
    pub fn riven(&self) -> RivenModule {
        // Lazily initialize RivenModule if not already initialized
        if self.riven_module.read().unwrap().is_none() {
            *self.item_module.write().unwrap() = Some(ItemModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the item_module is initialized
        self.riven_module.read().unwrap().as_ref().unwrap().clone()
    }
    pub fn auth(&self) -> AuthModule {
        // Lazily initialize AuthModule if not already initialized
        if self.auth_module.read().unwrap().is_none() {
            *self.auth_module.write().unwrap() = Some(AuthModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the item_module is initialized
        self.auth_module.read().unwrap().as_ref().unwrap().clone()
    }

    pub fn analytics(&self) -> AnalyticsModule {
        // Lazily initialize AnalyticsModule if not already initialized
        if self.analytics_module.read().unwrap().is_none() {
            *self.analytics_module.write().unwrap() =
                Some(AnalyticsModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the analytics_module is initialized
        self.analytics_module
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone()
    }

    pub fn update_analytics_module(&self, module: AnalyticsModule) {
        // Update the stored AnalyticsModule
        *self.analytics_module.write().unwrap() = Some(module);
    }
    pub fn alert(&self) -> AlertModule {
        // Lazily initialize AlertModule if not already initialized
        if self.alert_module.read().unwrap().is_none() {
            *self.alert_module.write().unwrap() = Some(AlertModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the analytics_module is initialized
        self.alert_module.read().unwrap().as_ref().unwrap().clone()
    }
    pub fn update_alert_module(&self, module: AlertModule) {
        // Update the stored AnalyticsModule
        *self.alert_module.write().unwrap() = Some(module);
    }
    pub fn transaction(&self) -> TransactionModule {
        // Lazily initialize TransactionModule if not already initialized
        if self.transaction_module.read().unwrap().is_none() {
            *self.transaction_module.write().unwrap() =
                Some(TransactionModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the transaction_module is initialized
        self.transaction_module
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone()
    }
}
