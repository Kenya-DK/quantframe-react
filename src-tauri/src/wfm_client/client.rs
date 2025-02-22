use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};

use eyre::eyre;
use reqwest::{Client, Method, Url};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

use crate::{
    logger,
    utils::{
        enums::{
            log_level::LogLevel,
            ui_events::{UIEvent, UIOperationEvent},
        },
        modules::{
            error::{ApiResult, AppError, ErrorApiResponse},
            rate_limiter::RateLimiter,
            states,
        },
    },
};

use super::modules::{
    auction::AuctionModule, auth::AuthModule, chat::ChatModule, order::OrderModule,
};

#[derive(Clone, Debug)]
pub struct WFMClient {
    endpoint: String,
    pub component: String,
    limiter: Arc<tokio::sync::Mutex<RateLimiter>>,
    order_module: Arc<RwLock<Option<OrderModule>>>,
    chat_module: Arc<RwLock<Option<ChatModule>>>,
    auction_module: Arc<RwLock<Option<AuctionModule>>>,
    auth_module: Arc<RwLock<Option<AuthModule>>>,
    pub log_file: String,
}

impl WFMClient {
    pub fn new() -> Self {
        WFMClient {
            endpoint: "https://api.warframe.market/v1/".to_string(),
            component: "WarframeMarket".to_string(),
            limiter: Arc::new(tokio::sync::Mutex::new(RateLimiter::new(
                1.0,
                Duration::new(1, 0),
            ))),
            log_file: "wfmAPICalls.log".to_string(),
            order_module: Arc::new(RwLock::new(None)),
            chat_module: Arc::new(RwLock::new(None)),
            auction_module: Arc::new(RwLock::new(None)),
            auth_module: Arc::new(RwLock::new(None)),
        }
    }

    pub fn debug(&self, id: &str, component: &str, msg: &str, file: Option<bool>) {
        let settings = states::settings().expect("Could not get settings");
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

    fn handle_error(&self, errors: Vec<String>) {
        let mut auth = states::auth().expect("Could not get auth state");
        let notify = states::notify_client().expect("Could not get notify client");
        if errors.contains(&"app.errors.unauthorized".to_string()) {
            auth.reset();
            notify.gui().send_event_update(
                UIEvent::UpdateUser,
                UIOperationEvent::Set,
                Some(json!(&auth.clone())),
            );
        } else if errors.contains(&"app.errors.banned".to_string()) {
            auth.ban_user_wfm(&"Banned from Warframe Market");
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
        payload_key: Option<&str>,
        body: Option<Value>,
    ) -> Result<ApiResult<T>, AppError> {
        let auth = states::auth()?;
        let settings = states::settings()?;
        let app = states::app_state()?;
        let mut rate_limiter = self.limiter.lock().await;

        rate_limiter.wait_for_token().await;

        let packageinfo = app.get_app_info();

        let client = Client::new();
        let new_url = format!("{}{}", self.endpoint, url);

        let request = client
            .request(method.clone(), Url::parse(&new_url).unwrap())
            .header(
                "Authorization",
                format!(
                    "JWT {}",
                    auth.wfm_access_token.clone().unwrap_or("".to_string())
                ),
            )
            .header(
                "User-Agent",
                format!("Quantframe {}", packageinfo.version.to_string()),
            )
            .header("Language", auth.region.clone())
            .header("Crossplay", settings.cross_play.clone());

        let request = match body.clone() {
            Some(content) => request.json(&content),
            None => request,
        };

        let response = request.send().await;

        // Create default error response
        let mut error_def = ErrorApiResponse {
            status_code: 999,
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
                LogLevel::Warning,
            ));
        }

        // Get the response data from the response
        let response_data = response.unwrap();
        error_def.status_code = response_data.status().as_u16() as i64;
        let headers = response_data.headers().clone();
        let content = response_data.text().await.unwrap_or_default();
        error_def.raw_response = Some(content.clone());

        // Convert the response to a Value object
        let response: Value = match serde_json::from_str(content.as_str()) {
            Ok(response) => response,
            Err(e) => {
                error_def.messages.push(e.to_string());
                error_def.error = "RequestError".to_string();

                let log_level = match error_def.status_code {
                    400 => LogLevel::Warning,
                    502 => LogLevel::Warning,
                    200 => LogLevel::Warning,
                    _ => LogLevel::Critical,
                };
                return Err(AppError::new_api(
                    self.component.as_str(),
                    error_def,
                    eyre!(format!("Could not parse response: {}, {:?}", content, e)),
                    log_level,
                ));
            }
        };

        // Check if the response is an error
        if response.get("error").is_some() {
            error_def.error = "ApiError".to_string();

            let error: Value = response["error"].clone();
            if error.is_string() {
                error_def.messages.push(error.as_str().unwrap().to_string());
            } else if error.is_object() {
                match serde_json::from_value::<HashMap<String, Value>>(response["error"].clone()) {
                    Ok(errors) => {
                        for (key, value) in errors {
                            if value.is_array() {
                                let messages: Vec<String> = serde_json::from_value(value.clone())
                                    .map_err(|e| {
                                    AppError::new_api(
                                        self.component.as_str(),
                                        error_def.clone(),
                                        eyre!(format!("Could not parse error messages: {}", e)),
                                        LogLevel::Critical,
                                    )
                                })?;
                                error_def.messages.push(format!(
                                    "{}: {}",
                                    key,
                                    messages.join(", ")
                                ));
                            } else {
                                error_def.messages.push(format!("{}: {:?}", key, value));
                            }
                        }
                    }
                    Err(e) => {
                        error_def.messages.push(e.to_string());
                        error_def.error = "ParseError".to_string();
                        return Err(AppError::new_api(
                            self.component.as_str(),
                            error_def,
                            eyre!(format!("Could not parse error messages: {}", e)),
                            LogLevel::Critical,
                        ));
                    }
                }
            }
            self.handle_error(error_def.messages.clone());
            return Ok(ApiResult::Error(error_def, headers));
        }

        // Get the payload from the response if it exists
        let mut data = response.clone();
        if response.get("payload").is_some() {
            data = response["payload"].clone();
        }

        if let Some(payload_key) = payload_key {
            data = data[payload_key].clone();
        }

        // Convert the response to a T object
        match serde_json::from_value(data.clone()) {
            Ok(payload) => Ok(ApiResult::Success(payload, headers)),
            Err(e) => {
                error_def.messages.push(e.to_string());
                error_def.error = "ParseError".to_string();
                return Err(AppError::new_api(
                    self.component.as_str(),
                    error_def,
                    eyre!(format!("Could not parse payload: {}", e)),
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

    pub fn auth(&self) -> AuthModule {
        // Lazily initialize ItemModule if not already initialized
        if self.auth_module.read().unwrap().is_none() {
            *self.auth_module.write().unwrap() = Some(AuthModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the item_module is initialized
        self.auth_module.read().unwrap().as_ref().unwrap().clone()
    }

    pub fn orders(&self) -> OrderModule {
        // Lazily initialize ItemModule if not already initialized
        if self.order_module.read().unwrap().is_none() {
            *self.order_module.write().unwrap() = Some(OrderModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the order_module is initialized
        self.order_module.read().unwrap().as_ref().unwrap().clone()
    }
    pub fn update_order_module(&self, module: OrderModule) {
        // Update the stored ItemModule
        *self.order_module.write().unwrap() = Some(module);
    }

    pub fn auction(&self) -> AuctionModule {
        // Lazily initialize AuctionModule if not already initialized
        if self.auction_module.read().unwrap().is_none() {
            *self.auction_module.write().unwrap() = Some(AuctionModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the item_module is initialized
        self.auction_module
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone()
    }
    pub fn update_auction_module(&self, module: AuctionModule) {
        // Update the stored AuctionModule
        *self.auction_module.write().unwrap() = Some(module);
    }

    pub fn chat(&self) -> ChatModule {
        // Lazily initialize ChatModule if not already initialized
        if self.chat_module.read().unwrap().is_none() {
            *self.chat_module.write().unwrap() = Some(ChatModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the chat_module is initialized
        self.chat_module.read().unwrap().as_ref().unwrap().clone()
    }
    pub fn update_chat_module(&self, module: ChatModule) {
        // Update the stored ChatModule
        *self.chat_module.write().unwrap() = Some(module);
    }
}
