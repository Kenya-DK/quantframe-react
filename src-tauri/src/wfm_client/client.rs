use std::{
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
    error::AppError,
    helper,
    logger::{self, LogLevel},
    rate_limiter::RateLimiter,
};

use super::modules::{
    auction::AuctionModule, auth::AuthModule, item::ItemModule, order::OrderModule, chat::ChatModule,
};

#[derive(Clone, Debug)]
pub struct WFMClient {
    endpoint: String,
    limiter: Arc<tokio::sync::Mutex<RateLimiter>>,
    pub log_file: String,
    pub auth: Arc<Mutex<AuthState>>,
}

impl WFMClient {
    pub fn new(auth: Arc<Mutex<AuthState>>) -> Self {
        WFMClient {
            endpoint: "https://api.warframe.market/v1/".to_string(),
            limiter: Arc::new(tokio::sync::Mutex::new(RateLimiter::new(
                1.0,
                Duration::new(1, 0),
            ))),
            log_file: "wfmAPICalls.log".to_string(),
            auth,
        }
    }

    fn create_error(
        &self,
        url: &str,
        method: Method,
        status: StatusCode,
        raw_response: String,
        body: Option<Value>,
        error: Option<String>,
    ) -> AppError {
        let body = match body {
            Some(mut content) => {
                if content["password"].is_string() {
                    content["password"] = json!("********");
                }
                if content["access_token"].is_string() {
                    content["access_token"] = json!("********");
                }
                if content["email"].is_string() {
                    content["email"] = json!("********");
                }
                content.clone()
            }
            None => json!({}),
        };
        let data = json!({
            "response": raw_response,
            "payload": body,
        });

        AppError::new_with_level(
            "WarframeMarket",
            eyre!(
                "Error Message: {}, Url: {}, Method: {}, Status: {}, Raw Response: [J]{}[J]",
                error.unwrap_or("NONE".to_string()),
                url,
                method,
                status,
                data
            ),
            LogLevel::Error,
        )
    }

    async fn send_request<T: DeserializeOwned>(
        &self,
        method: Method,
        url: &str,
        payload_key: Option<&str>,
        body: Option<Value>,
    ) -> Result<(T, HeaderMap), AppError> {
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
        // let response: Value = request.send().await?.json().await;
        let response = request.send().await;

        if let Err(e) = response {
            return Err(self.create_error(
                &new_url,
                method,
                StatusCode::BAD_REQUEST,
                "NO".to_string(),
                body,
                Some(e.to_string()),
            ));
        }
        let response_data = response.unwrap();
        let status = response_data.status();
        let headers = response_data.headers().clone();
        let content = response_data.text().await.unwrap_or_default();

        if status != 200 {
            return Err(self.create_error(
                &new_url,
                method,
                status.clone(),
                content.clone(),
                body.clone(),
                None,
            ));
        }

        let response: Value = serde_json::from_str(content.as_str()).map_err(|e| {
            self.create_error(
                &new_url,
                method.clone(),
                status.clone(),
                content.clone(),
                body.clone(),
                Some(e.to_string()),
            )
        })?;

        let mut data = response["payload"].clone();
        if let Some(payload_key) = payload_key {
            data = response["payload"][payload_key].clone();
        }

        // Convert the response to a T object
        match serde_json::from_value(data.clone()) {
            Ok(payload) => Ok((payload, headers)),
            Err(e) => Err(self.create_error(
                &new_url,
                method,
                status.clone(),
                content,
                body,
                Some(e.to_string()),
            )),
        }
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        url: &str,
        payload_key: Option<&str>,
    ) -> Result<(T, HeaderMap), AppError> {
        let payload: (T, HeaderMap) = self
            .send_request(Method::GET, url, payload_key, None)
            .await?;
        Ok(payload)
    }

    pub async fn post<T: DeserializeOwned>(
        &self,
        url: &str,
        payload_key: Option<&str>,
        body: Value,
    ) -> Result<(T, HeaderMap), AppError> {
        let payload: (T, HeaderMap) = self
            .send_request(Method::POST, url, payload_key, Some(body))
            .await?;
        Ok(payload)
    }

    pub async fn delete<T: DeserializeOwned>(
        &self,
        url: &str,
        payload_key: Option<&str>,
    ) -> Result<(T, HeaderMap), AppError> {
        let payload: (T, HeaderMap) = self
            .send_request(Method::DELETE, url, payload_key, None)
            .await?;
        Ok(payload)
    }

    pub async fn put<T: DeserializeOwned>(
        &self,
        url: &str,
        payload_key: Option<&str>,
        body: Option<Value>,
    ) -> Result<(T, HeaderMap), AppError> {
        let payload: (T, HeaderMap) = self
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
