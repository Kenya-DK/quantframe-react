use std::{
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};

use eyre::eyre;
use reqwest::{header::HeaderMap, Client, Method, StatusCode, Url};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    app::client::AppState, auth::AuthState, logger, utils::{
        enums::log_level::LogLevel,
        modules::{
            error::{ApiResult, AppError, ErrorApiResponse},
            rate_limiter::RateLimiter,
        },
    }
};

use super::modules::{cache::CacheModule, price_scraper::PriceScraperModule};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ByteResponse {
    #[serde(rename = "data")]
    pub data: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct QFClient {
    endpoint: String,
    limiter: Arc<tokio::sync::Mutex<RateLimiter>>,
    cache_module: Arc<RwLock<Option<CacheModule>>>,
    price_module: Arc<RwLock<Option<PriceScraperModule>>>,
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
            endpoint: "http://localhost:6969/api/".to_string(),
            limiter: Arc::new(tokio::sync::Mutex::new(RateLimiter::new(
                3.0,
                Duration::new(1, 0),
            ))),
            cache_module: Arc::new(RwLock::new(None)),
            price_module: Arc::new(RwLock::new(None)),
            log_file: "qfAPICalls.log".to_string(),
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
        let mut rate_limiter = self.limiter.lock().await;
        rate_limiter.wait_for_token().await;

        let packageinfo = app.get_app_info();

        let client = Client::new();
        let new_url = format!("{}{}", self.endpoint, url);
        let request = client
            .request(method.clone(), Url::parse(&new_url).unwrap())
            .header(
                "User-Agent",
                format!("Quantframe {}", packageinfo.version.to_string()),
            );

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
        // let mut content = response_data.text().await.unwrap_or_default();
        // if is_bytes {
        //     let content_bytes = response_data.bytes().await.unwrap_or_default();
        //     content = format!("{{\"data\": \"{:?}\"}}", content_bytes.to_vec());
        // }

        let mut response: Value = serde_json::from_str(content.as_str()).map_err(|e| {
            error_def.messages.push(e.to_string());
            AppError::new_api(
                self.component.as_str(),
                error_def.clone(),
                // eyre!(format!("Could not parse response: {}, {:?}", content, e)),
                eyre!(""),
                LogLevel::Critical,
            )
        })?;

        if response.get("statusCode").is_some()
            && response.get("error").is_some()
            && response.get("message").is_some()
        {
            if response.get("message").unwrap().is_string() {
                let msg = response.get("message");
                response["message"] = json!([msg]);
            }

            let error: ErrorApiResponse = match serde_json::from_value(response.clone()) {
                Ok(payload) => payload,
                Err(e) => {
                    error_def.messages.push(e.to_string());
                    error_def.clone()
                }
            };
            error_def.error = error.error;
            error_def.messages = error.messages;
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

    pub async fn delete<T: DeserializeOwned>(&self, url: &str) -> Result<ApiResult<T>, AppError> {
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
    // pub fn update_cache_module(&self, module: CacheModule) {
    //     // Update the stored CacheModule
    //     *self.cache_module.write().unwrap() = Some(module);
    // }
    pub fn price(&self) -> PriceScraperModule {
        // Lazily initialize PriceScraperModule if not already initialized
        if self.price_module.read().unwrap().is_none() {
            *self.price_module.write().unwrap() =
                Some(PriceScraperModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the price_module is initialized
        self.price_module.read().unwrap().as_ref().unwrap().clone()
    }
}
