use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use eyre::eyre;
use polars::{
    prelude::{DataFrame, NamedFrom},
    series::Series,
};
use reqwest::{header::HeaderMap, Client, Method, StatusCode, Url};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

use crate::{
    auth::AuthState,
    enums::LogLevel,
    error::{ApiResult, AppError, ErrorApiResponse},
    helper,
    logger::{self},
    rate_limiter::RateLimiter,
};

use super::modules::{
    auction::AuctionModule, auth::AuthModule, chat::ChatModule, item::ItemModule,
    order::OrderModule,
};

#[derive(Clone, Debug)]
pub struct WFMClient {
    endpoint: String,
    limiter: Arc<tokio::sync::Mutex<RateLimiter>>,
    pub log_file: String,
    pub auth: Arc<Mutex<AuthState>>,
    pub settings: Arc<Mutex<crate::settings::SettingsState>>,
}

impl WFMClient {
    pub fn new(auth: Arc<Mutex<AuthState>>, settings: Arc<Mutex<crate::settings::SettingsState>>) -> Self {
        WFMClient {
            endpoint: "https://api.warframe.market/v1/".to_string(),
            limiter: Arc::new(tokio::sync::Mutex::new(RateLimiter::new(
                1.0,
                Duration::new(1, 0),
            ))),
            log_file: "wfmAPICalls.log".to_string(),
            auth,
            settings,
        }
    }

    pub fn debug(&self, component: &str, msg: &str, file: Option<bool>) {
        let settings = self.settings.lock().unwrap().clone();
        if !settings.debug {
            return;
        }
        if file.is_none() {
            logger::debug(format!("WarframeMarket:{}", component).as_str(), msg, true, None);
            return;
        }        
        logger::debug(format!("WarframeMarket:{}", component).as_str(), msg, true, Some(&self.log_file));
    }

    async fn send_request<T: DeserializeOwned>(
        &self,
        method: Method,
        url: &str,
        payload_key: Option<&str>,
        body: Option<Value>,
    ) -> Result<ApiResult<T>, AppError> {
        let auth = self.auth.lock()?.clone();
        let mut rate_limiter = self.limiter.lock().await;

        rate_limiter.wait_for_token().await;

        let packageinfo = crate::PACKAGEINFO
            .lock()
            .unwrap()
            .clone()
            .expect("Could not get package info");

        let client = Client::new();
        let new_url = format!("{}{}", self.endpoint, url);
        let request = client
            .request(method.clone(), Url::parse(&new_url).unwrap())
            .header(
                "Authorization",
                format!("JWT {}", auth.access_token.unwrap_or("".to_string())),
            )
            .header(
                "User-Agent",
                format!("Quantframe {}", packageinfo.version.to_string()),
            )
            .header("Language", auth.region);

        let request = match body.clone() {
            Some(content) => request.json(&content),
            None => request,
        };

        let response = request.send().await;

        // Create default error response
        let mut error_def = ErrorApiResponse {
            status_code: 500,
            error: "UnknownError".to_string(),
            message: vec![],
            raw_response: None,
            body: body.clone(),
            url: Some(new_url.clone()),
            method: Some(method.to_string()),
        };

        if let Err(e) = response {
            error_def.message.push(e.to_string());
            return Err(AppError::new_api(
                "WarframeMarket",
                error_def,
                eyre!(format!("There was an error sending the request: {}", e)),
                LogLevel::Critical,
            ));
        }

        // Get the response data from the response
        let response_data = response.unwrap();
        error_def.status_code = response_data.status().as_u16() as i64;
        let headers = response_data.headers().clone();
        let content = response_data.text().await.unwrap_or_default();
        error_def.raw_response = Some(content.clone());

        // Convert the response to a Value object
        let response: Value = serde_json::from_str(content.as_str()).map_err(|e| {
            error_def.message.push(e.to_string());
            error_def.error = "RequestError".to_string();
            AppError::new_api(
                "WarframeMarket",
                error_def.clone(),
                eyre!(""),
                LogLevel::Critical,
            )
        })?;

        // Check if the response is an error
        if response.get("error").is_some() {            
            error_def.error = "ApiError".to_string();
            // Loop through the error object and add each message to the error_def
            let errors: HashMap<String, Value> = serde_json::from_value(response["error"].clone())
                .map_err(|e| {
                    error_def.message.push(e.to_string());
                    AppError::new_api(
                        "WarframeMarket",
                        error_def.clone(),
                        eyre!(""),
                        LogLevel::Critical,
                    )
                })?;

            for (key, value) in errors {
                if value.is_array() {
                    let messages: Vec<String> =
                        serde_json::from_value(value.clone()).map_err(|e| {
                            AppError::new_api(
                                "WarframeMarket",
                                error_def.clone(),
                                eyre!(format!("Could not parse error messages: {}", e)),
                                LogLevel::Critical,
                            )
                        })?;
                    error_def
                        .message
                        .push(format!("{}: {}", key, messages.join(", ")));
                } else {
                    error_def.message.push(format!("{}: {:?}", key, value));
                }
            }
            return Ok(ApiResult::Error(error_def, headers));
        }

        // Get the payload from the response if it exists
        let mut data = response["payload"].clone();
        if let Some(payload_key) = payload_key {
            data = response["payload"][payload_key].clone();
        }

        // Convert the response to a T object
        match serde_json::from_value(data.clone()) {
            Ok(payload) => Ok(ApiResult::Success(payload, headers)),
            Err(e) => {
                error_def.message.push(e.to_string());
                error_def.error = "ParseError".to_string();
                return Err(AppError::new_api(
                    "WarframeMarket",
                    error_def,
                    eyre!(""),
                    LogLevel::Critical,
                ));
            }
        }
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        url: &str,
        payload_key: Option<&str>,
    ) -> Result<ApiResult<T>, AppError> {
        let payload: ApiResult<T> = self
            .send_request(Method::GET, url, payload_key, None)
            .await?;
        Ok(payload)
    }

    pub async fn post<T: DeserializeOwned>(
        &self,
        url: &str,
        payload_key: Option<&str>,
        body: Value,
    ) -> Result<ApiResult<T>, AppError> {
        let payload: ApiResult<T> = self
            .send_request(Method::POST, url, payload_key, Some(body))
            .await?;
        Ok(payload)
    }

    pub async fn delete<T: DeserializeOwned>(
        &self,
        url: &str,
        payload_key: Option<&str>,
    ) -> Result<ApiResult<T>, AppError> {
        let payload: ApiResult<T> = self
            .send_request(Method::DELETE, url, payload_key, None)
            .await?;
        Ok(payload)
    }

    pub async fn put<T: DeserializeOwned>(
        &self,
        url: &str,
        payload_key: Option<&str>,
        body: Option<Value>,
    ) -> Result<ApiResult<T>, AppError> {
        let payload: ApiResult<T> = self
            .send_request(Method::PUT, url, payload_key, body)
            .await?;
        Ok(payload)
    }
    // Add an "add" method to WFMWFMClient
    pub fn auth(&self) -> AuthModule {
        AuthModule { client: self }
    }

    pub fn orders(&self) -> OrderModule {
        OrderModule { client: self }
    }

    pub fn items(&self) -> ItemModule {
        ItemModule { client: self }
    }
    pub fn auction(&self) -> AuctionModule {
        AuctionModule { client: self }
    }
    pub fn chat(&self) -> ChatModule {
        ChatModule { client: self }
    }
}
