use std::{
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};

use eyre::eyre;
use reqwest::{Client, Method, Url};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use crate::{
    app::client::AppState,
    auth::AuthState,
    logger,
    utils::{
        enums::log_level::LogLevel,
        modules::{
            error::{ApiResult, AppError, ErrorApiResponse},
            rate_limiter::RateLimiter,
        },
    },
};

use super::modules::{
    analytics::AnalyticsModule, auth::AuthModule, cache::CacheModule, price_scraper::PriceScraperModule, transaction::TransactionModule
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
    price_module: Arc<RwLock<Option<PriceScraperModule>>>,
    analytics_module: Arc<RwLock<Option<AnalyticsModule>>>,
    transaction_module: Arc<RwLock<Option<TransactionModule>>>,
    pub component: String,
    pub log_file: String,
    pub auth: Arc<Mutex<AuthState>>,
    pub settings: Arc<Mutex<crate::settings::SettingsState>>,
    pub app: Arc<Mutex<AppState>>,
}

impl QFClient {
    pub fn new(
        auth: Arc<Mutex<AuthState>>,
        settings: Arc<Mutex<crate::settings::SettingsState>>,
        app: Arc<Mutex<AppState>>,
    ) -> Self {
        QFClient {
            endpoint: "https://api.quantframe.app/".to_string(),
            endpoint_dev: "http://localhost:6969/".to_string(),
            is_dev: true,
            limiter: Arc::new(tokio::sync::Mutex::new(RateLimiter::new(
                3.0,
                Duration::new(1, 0),
            ))),
            auth_module: Arc::new(RwLock::new(None)),
            cache_module: Arc::new(RwLock::new(None)),
            price_module: Arc::new(RwLock::new(None)),
            analytics_module: Arc::new(RwLock::new(None)),
            transaction_module: Arc::new(RwLock::new(None)),
            log_file: "qfAPIaCalls.log".to_string(),
            component: "QuantframeApi".to_string(),
            auth,
            settings,
            app,
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
        let settings = self.settings.lock().unwrap().clone();
        if !settings.debug.contains(&"*".to_owned()) && !settings.debug.contains(&id.to_owned()) {
            return;
        }

        if file.is_none() {
            logger::debug(
                format!("{}:{}", self.component, component).as_str(),
                msg,
                true,
                None,
            );
            return;
        }
        logger::debug(
            format!("{}:{}", self.component, component).as_str(),
            msg,
            true,
            Some(&self.log_file),
        );
    }
    async fn send_request<T: DeserializeOwned>(
        &self,
        method: Method,
        url: &str,
        body: Option<Value>,
        is_bytes: bool,
    ) -> Result<ApiResult<T>, AppError> {
        let app = self.app.lock()?.clone();
        let auth = self.auth.lock()?.clone();

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
            .header("App", packageinfo.name.to_string())
            .header("Device", auth.get_device_id())
            .header("Version", packageinfo.version.to_string())
            .header("UserName", auth.ingame_name)
            .header("UserId", auth.id);

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

        let content = if is_bytes {
            let content_bytes = response_data.bytes().await.unwrap_or_default();
            format!("{{\"data\": {:?}}}", content_bytes.to_vec())
        } else {
            response_data.text().await.unwrap_or_default()
        };
        error_def.raw_response = Some(content.clone());

        let response: Value = serde_json::from_str(content.as_str()).map_err(|e| {
            error_def.messages.push(e.to_string());
            AppError::new_api(
                self.component.as_str(),
                error_def.clone(),
                eyre!(format!("Could not parse response: {}, {:?}", content, e)),
                LogLevel::Critical,
            )
        })?;
        // If the status code is not between 200 and 204, it's an error
        if error_def.status_code < 200 || error_def.status_code > 299 {
            if response.get("message").is_some() && response.get("message").unwrap().is_string() {
                let msg = response.get("message").unwrap().as_str();
                error_def.messages.push(msg.unwrap_or_default().to_string());
            }
            if response.get("message").is_some() && response.get("message").unwrap().is_array() {
                let msg = response.get("message").unwrap().as_array();
                for m in msg.unwrap() {
                    error_def.messages.push(m.to_string());
                }
            }
            if response.get("error").is_some() && response.get("error").unwrap().is_string() {
                let msg = response.get("error").unwrap().as_str();
                error_def.error = msg.unwrap_or_default().to_string();
            }
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
            .send_request::<ByteResponse>(Method::GET, url, None, true)
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

    pub async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<ApiResult<T>, AppError> {
        Ok(self.send_request(Method::GET, url, None, false).await?)
    }

    pub async fn post<T: DeserializeOwned>(
        &self,
        url: &str,
        body: Value,
    ) -> Result<ApiResult<T>, AppError> {
        Ok(self
            .send_request(Method::POST, url, Some(body), false)
            .await?)
    }

    pub async fn delete<T: DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<ApiResult<T>, AppError> {
        Ok(self.send_request(Method::DELETE, url, None, false).await?)
    }
    pub async fn put<T: DeserializeOwned>(
        &self,
        url: &str,
        body: Option<Value>,
    ) -> Result<ApiResult<T>, AppError> {
        Ok(self.send_request(Method::PUT, url, body, false).await?)
    }
    pub fn cache(&self) -> CacheModule {
        // Lazily initialize ItemModule if not already initialized
        if self.cache_module.read().unwrap().is_none() {
            *self.cache_module.write().unwrap() = Some(CacheModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the cache_module is initialized
        self.cache_module.read().unwrap().as_ref().unwrap().clone()
    }

    pub fn price(&self) -> PriceScraperModule {
        // Lazily initialize PriceScraperModule if not already initialized
        if self.price_module.read().unwrap().is_none() {
            *self.price_module.write().unwrap() =
                Some(PriceScraperModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the price_module is initialized
        self.price_module.read().unwrap().as_ref().unwrap().clone()
    }
    pub fn auth(&self) -> AuthModule {
        // Lazily initialize PriceScraperModule if not already initialized
        if self.auth_module.read().unwrap().is_none() {
            *self.auth_module.write().unwrap() = Some(AuthModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the price_module is initialized
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

    pub fn update_transaction_module(&self, module: TransactionModule) {
        // Update the stored TransactionModule
        *self.transaction_module.write().unwrap() = Some(module);
    }
}
