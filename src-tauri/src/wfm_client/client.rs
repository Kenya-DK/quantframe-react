use std::sync::{Arc, Mutex};

use eyre::eyre;
use polars::{
    prelude::{DataFrame, NamedFrom},
    series::Series,
};
use reqwest::{header::HeaderMap, Client, Method, Url};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

use crate::{
    auth::AuthState,
    error::AppError,
    helper,
    logger::{self, LogLevel},
};

use super::modules::{auth::AuthModule, item::ItemModule, order::OrderModule};

#[derive(Clone, Debug)]
pub struct WFMClient {
    endpoint: String,
    pub log_file: String,
    pub auth: Arc<Mutex<AuthState>>,
}

impl WFMClient {
    pub fn new(auth: Arc<Mutex<AuthState>>) -> Self {
        WFMClient {
            endpoint: "https://api.warframe.market/v1/".to_string(),
            log_file: "wfmAPICalls.log".to_string(),
            auth,
        }
    }
    async fn send_request<T: DeserializeOwned>(
        &self,
        method: Method,
        url: &str,
        payload_key: Option<&str>,
        body: Option<Value>,
    ) -> Result<(T, HeaderMap), AppError> {
        let auth = self.auth.lock()?.clone();
        // Sleep for 1 seconds before sending a new request, to avoid 429 error
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let client = Client::new();
        let new_url = format!("{}{}", self.endpoint, url);

        let request = client
            .request(method, Url::parse(&new_url).unwrap())
            .header(
                "Authorization",
                format!("JWT {}", auth.access_token.unwrap_or("".to_string())),
            )
            .header("User-Agent", format!("Quantframe {}", "0.0.0".to_string()))
            .header("Language", auth.region);

        let request = match body.clone() {
            Some(content) => request.json(&content),
            None => request,
        };
        // let response: Value = request.send().await?.json().await;
        let response = request.send().await;

        if let Err(e) = response {
            return Err(AppError::new_with_level(
                "WFMWFMClient",
                eyre!("Error: {:?}, Url: {:?}", e.to_string(), new_url),
                LogLevel::Error,
            ));
        }
        let response_data = response.unwrap();
        let status = response_data.status();

        if status != 200 {
            let rep = response_data.text().await.unwrap_or_default();
            return Err(AppError::new_with_level(
                "WFMWFMClient",
                eyre!("Status: {:?}[J]{rep}[J], Url: {:?}", status, new_url),
                LogLevel::Error,
            ));
        }

        let headers = response_data.headers().clone();
        let response = response_data.json::<Value>().await.map_err(|e| {
            AppError::new_with_level(
                "WFMWFMClient",
                eyre!(
                    "Error: {}, Url: {}, Status: {}",
                    e.to_string(),
                    new_url,
                    status
                ),
                LogLevel::Error,
            )
        })?;

        let mut data = response["payload"].clone();
        if let Some(payload_key) = payload_key {
            data = response["payload"][payload_key].clone();
        }

        // Convert the response to a T object
        match serde_json::from_value(data.clone()) {
            Ok(payload) => Ok((payload, headers)),
            Err(e) => Err(AppError::new(
                "WFMWFMClient",
                eyre!("Error: {:?},[J]{}[J] Url: {:?}", e, data, new_url),
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
}
