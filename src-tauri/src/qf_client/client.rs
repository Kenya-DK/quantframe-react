use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use eyre::eyre;
use reqwest::{header::HeaderMap, Client, Method, StatusCode, Url};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    auth::AuthState,
    error::{AppError, ErrorApiResponse},
    logger::LogLevel,
    rate_limiter::RateLimiter,
};

use super::modules::{auth::AuthModule, user::UserModule};

#[derive(Debug)]
pub enum ApiResult<T> {
    Success(T, HeaderMap),
    Error(ErrorApiResponse, HeaderMap),
}

#[derive(Clone, Debug)]
pub struct QFClient {
    endpoint: String,
    limiter: Arc<tokio::sync::Mutex<RateLimiter>>,
    pub log_file: String,
    pub auth: Arc<Mutex<AuthState>>,
}

impl QFClient {
    pub fn new(auth: Arc<Mutex<AuthState>>) -> Self {
        QFClient {
            endpoint: "http://localhost:6969/api/".to_string(),
            limiter: Arc::new(tokio::sync::Mutex::new(RateLimiter::new(
                1.0,
                Duration::new(1, 0),
            ))),
            log_file: "wfmAPICalls.log".to_string(),
            auth,
        }
    }
    async fn send_request<T: DeserializeOwned>(
        &self,
        method: Method,
        url: &str,
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
                format!("JWT {}", auth.qf_access_token.unwrap_or("".to_string())),
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
                "QuantframeApi",
                error_def,
                eyre!(""),
                LogLevel::Critical,
            ));
        }
        let response_data = response.unwrap();
        error_def.status_code = response_data.status().as_u16() as i64;
        let headers = response_data.headers().clone();
        let content = response_data.text().await.unwrap_or_default();

        let mut response: Value = serde_json::from_str(content.as_str()).map_err(|e| {
            error_def.message.push(e.to_string());
            AppError::new_api(
                "QuantframeApi",
                error_def.clone(),
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
                    error_def.message.push(e.to_string());
                    error_def.clone()
                }
            };
            error_def.error = error.error;
            error_def.message = error.message;
            return Ok(ApiResult::Error(error_def, headers));
        }

        // Convert the response to a T object
        match serde_json::from_value(response.clone()) {
            Ok(payload) => Ok(ApiResult::Success(payload, headers)),
            Err(e) => {
                error_def.message.push(e.to_string());
                return Err(AppError::new_api(
                    "QuantframeApi",
                    error_def,
                    eyre!(""),
                    LogLevel::Critical,
                ));
            }
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<ApiResult<T>, AppError> {
        Ok(self.send_request(Method::GET, url, None).await?)
    }

    pub async fn post<T: DeserializeOwned>(
        &self,
        url: &str,
        body: Value,
    ) -> Result<ApiResult<T>, AppError> {
        Ok(self.send_request(Method::POST, url, Some(body)).await?)
    }

    pub async fn delete<T: DeserializeOwned>(&self, url: &str) -> Result<ApiResult<T>, AppError> {
        Ok(self.send_request(Method::DELETE, url, None).await?)
    }

    pub async fn put<T: DeserializeOwned>(
        &self,
        url: &str,
        body: Option<Value>,
    ) -> Result<ApiResult<T>, AppError> {
        Ok(self.send_request(Method::PUT, url, body).await?)
    }
    // Add an "add" method to WFMQFClient
    pub fn auth(&self) -> AuthModule {
        AuthModule { client: self }
    }
    // Add an "add" method to WFMQFClient
    pub fn user(&self) -> UserModule {
        UserModule { client: self }
    }
}
