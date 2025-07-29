use crate::{endpoints::*, enums::*, errors::*, utils::*};
use governor::{
    RateLimiter,
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
};
use reqwest::{
    Method,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde_json::Value;
use std::{
    collections::HashMap,
    num::{NonZero, NonZeroU32},
    sync::{Arc, OnceLock},
};

const REQUESTS_PER_SECOND: NonZeroU32 = NonZero::new(3).unwrap();

#[derive(Debug, Clone)]
pub struct Client {
    self_arc: OnceLock<Arc<Client>>,
    pub token: String,
    app_id: String,
    platform: String,
    pub device: String,
    is_development: bool,
    app: String,
    version: String,
    wfm_platform: String,
    wfm_username: String,
    wfm_id: String,
    limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
    // Routes
    authentication_route: OnceLock<Arc<AuthenticationRoute>>,
    analytics_route: OnceLock<Arc<AnalyticsRoute>>,
    alert_route: OnceLock<Arc<AlertRoute>>,
    cache_route: OnceLock<Arc<CacheRoute>>,
    item_price_route: OnceLock<Arc<ItemPriceRoute>>,
}
impl Client {
    fn arc(&self) -> Arc<Self> {
        self.self_arc
            .get_or_init(|| {
                Arc::new(Self {
                    self_arc: OnceLock::new(),
                    token: self.token.clone(),
                    app_id: self.app_id.clone(),
                    platform: self.platform.clone(),
                    device: self.device.clone(),
                    is_development: self.is_development.clone(),
                    app: self.app.clone(),
                    version: self.version.clone(),
                    wfm_platform: self.wfm_platform.clone(),
                    wfm_username: self.wfm_username.clone(),
                    wfm_id: self.wfm_id.clone(),
                    // Initialize the routes with the new client
                    authentication_route: self.authentication_route.clone(),
                    analytics_route: self.analytics_route.clone(),
                    alert_route: self.alert_route.clone(),
                    cache_route: self.cache_route.clone(),
                    item_price_route: self.item_price_route.clone(),
                    limiter: self.limiter.clone(),
                })
            })
            .clone()
    }

    /**
     * Creates a new `Client` instance with the provided parameters.
     * This method initializes the client with the necessary parameters and sets up the rate limiter.
     * # Arguments
     * * `token` - A string representing the JWT token for authentication.
     * * `app_id` - A string representing the application ID.
     * * `platform` - A string representing the platform (e.g., "web", "mobile").
     * * `device` - A string representing the device name.
     * * `is_development` - A boolean indicating if the client is in development mode.
     * * `app` - A string representing the application name.
     * * `version` - A string representing the application version.
     * * `wfm_platform` - A string representing the WFM platform (e.g "web", "mobile").
     * * `wfm_username` - A string representing the WFM username.
     * * `wfm_id` - A string representing the WFM user ID.
     * # Returns
     * A new `Client` instance initialized with the provided parameters.
     * This client can be used to make API requests and manage routes.
     */
    pub fn new(
        token: &str,
        app_id: &str,
        platform: &str,
        device: &str,
        is_development: bool,
        app: &str,
        version: &str,
        wfm_platform: &str,
        wfm_username: &str,
        wfm_id: &str,
    ) -> Self {
        Self {
            self_arc: OnceLock::new(),
            token: token.to_string(),
            app_id: app_id.to_string(),
            platform: platform.to_string(),
            device: device.to_string(),
            is_development: is_development,
            app: app.to_string(),
            version: version.to_string(),
            wfm_platform: wfm_platform.to_string(),
            wfm_username: wfm_username.to_string(),
            wfm_id: wfm_id.to_string(),
            authentication_route: OnceLock::new(),
            analytics_route: OnceLock::new(),
            alert_route: OnceLock::new(),
            cache_route: OnceLock::new(),
            item_price_route: OnceLock::new(),
            limiter: build_limiter(REQUESTS_PER_SECOND).into(),
        }
    }
    pub async fn call_api<T: serde::de::DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<Value>,
        headers: Option<HashMap<String, String>>,
        response_format: ResponseFormat,
    ) -> Result<(ApiResponse<T>, HeaderMap, RequestError), ApiError> {
        let url = format!("{}{}", "http://localhost:6969", path);
        let mut default_headers = reqwest::header::HeaderMap::new();

        // Create the error object for logging
        let mut error = RequestError::new(method.to_string(), url.clone(), body.clone());

        // Add the required headers
        default_headers.insert("Content-Type", "application/json".parse().unwrap());
        default_headers.insert("AppId", self.app_id.parse().unwrap());
        default_headers.insert("Platform", self.platform.parse().unwrap());
        default_headers.insert("Device", self.device.parse().unwrap());
        default_headers.insert(
            "IsDevelopment",
            self.is_development.to_string().parse().unwrap(),
        );
        default_headers.insert("App", self.app.parse().unwrap());
        default_headers.insert("Version", self.version.parse().unwrap());
        default_headers.insert("WFMPlatform", self.wfm_platform.parse().unwrap());
        default_headers.insert("WFMUsername", self.wfm_username.parse().unwrap());
        default_headers.insert("WFMId", self.wfm_id.parse().unwrap());
        default_headers.insert(
            "User-Agent",
            format!(
                "QF API Client/{} ({}, {}, {}, {})",
                env!("CARGO_PKG_VERSION"),
                self.platform,
                self.device,
                self.app,
                self.version
            )
            .parse()
            .unwrap(),
        );

        // If the client is authenticated, add the token to the headers
        if self.token != "" {
            default_headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("JWT {}", self.token).parse().unwrap(),
            );
        }

        // Add any additional headers provided
        if let Some(ref items) = headers {
            for (key, value) in items.iter() {
                default_headers.insert(
                    HeaderName::from_bytes(key.as_bytes()).unwrap(),
                    HeaderValue::from_str(value).unwrap(),
                );
            }
        }

        // Add Headers for the error object
        error.set_headers(
            default_headers
                .iter()
                .map(|(k, v)| {
                    (
                        k.to_string(),
                        v.to_str().unwrap_or("Invalid Header").to_string(),
                    )
                })
                .collect(),
        );

        // Create the HTTP client with the headers
        let http_client = reqwest::Client::builder()
            .default_headers(default_headers)
            .build()
            .unwrap();

        let mut builder = http_client.request(method, &url);
        // If the client needs a body, serialize it
        if let Some(b) = body {
            builder = builder.json(&b);
        }
        self.limiter.until_ready().await;

        match builder.send().await {
            Ok(resp) => {
                let headers = resp.headers().clone();
                let status = resp.status();

                error.set_status_code(status.as_u16());
                // Check if the status code indicates an error
                match status {
                    reqwest::StatusCode::OK | reqwest::StatusCode::CREATED => {}
                    reqwest::StatusCode::TOO_MANY_REQUESTS => {
                        return Err(ApiError::TooManyRequests(error));
                    }
                    reqwest::StatusCode::BAD_REQUEST
                    | reqwest::StatusCode::FORBIDDEN
                    | reqwest::StatusCode::UNAUTHORIZED
                    | reqwest::StatusCode::NOT_FOUND => {
                        let body = resp.text().await.map_err(|_| {
                            ApiError::Unknown("Failed to read response body".to_string())
                        })?;
                        match serde_json::from_str::<ResponseError>(&body) {
                            Ok(r) => error.set_error(r),
                            Err(e) => return Err(ApiError::ParsingError(error, e)),
                        };
                        if status == reqwest::StatusCode::FORBIDDEN {
                            return Err(ApiError::Forbidden(error));
                        } else if status == reqwest::StatusCode::NOT_FOUND {
                            return Err(ApiError::NotFound(error));
                        } else if status == reqwest::StatusCode::UNAUTHORIZED {
                            return Err(ApiError::Unauthorized(error));
                        } else {
                            return Err(ApiError::BadRequest(error));
                        }
                    }
                    _ => {
                        return Err(ApiError::Unknown(format!(
                            "Unexpected status code: {}",
                            status
                        )));
                    }
                }

                match response_format {
                    ResponseFormat::Json => {
                        let body = resp
                            .text()
                            .await
                            .map_err(|_| ApiError::Unknown("Failed to read JSON".to_string()))?;
                        error.set_content(body.clone());
                        error.set_status_code(status.as_u16());

                        match serde_json::from_str::<T>(&body) {
                            Ok(data) => Ok((ApiResponse::Json(data), headers, error)),
                            Err(e) => Err(ApiError::ParsingError(error, e)),
                        }
                    }
                    ResponseFormat::String => {
                        let body = resp
                            .text()
                            .await
                            .map_err(|_| ApiError::Unknown("Failed to read string".to_string()))?;
                        error.set_content(body.clone());
                        error.set_status_code(status.as_u16());
                        Ok((ApiResponse::String(body), headers, error))
                    }
                    ResponseFormat::Bytes => {
                        let bytes = resp
                            .bytes()
                            .await
                            .map_err(|_| ApiError::Unknown("Failed to read bytes".to_string()))?;
                        error.set_content("[binary data]".to_string());
                        error.set_status_code(status.as_u16());
                        Ok((ApiResponse::Bytes(bytes.to_vec()), headers, error))
                    }
                }
            }
            Err(_) => Err(ApiError::RequestError(error)),
        }
    }

    // Endpoint methods to access routes
    pub fn authentication(&self) -> Arc<AuthenticationRoute> {
        self.authentication_route
            .get_or_init(|| AuthenticationRoute::new(self.arc()))
            .clone()
    }
    pub fn analytics(&self) -> Arc<AnalyticsRoute> {
        self.analytics_route
            .get_or_init(|| AnalyticsRoute::new(self.arc()))
            .clone()
    }
    pub fn alert(&self) -> Arc<AlertRoute> {
        self.alert_route
            .get_or_init(|| AlertRoute::new(self.arc()))
            .clone()
    }
    pub fn cache(&self) -> Arc<CacheRoute> {
        self.cache_route
            .get_or_init(|| CacheRoute::new(self.arc()))
            .clone()
    }
    pub fn item_price(&self) -> Arc<ItemPriceRoute> {
        self.item_price_route
            .get_or_init(|| ItemPriceRoute::new(self.arc()))
            .clone()
    }
}

// ---------- Client Get/Set Methods ----------
impl Client {
    /**
     * Sets the WFM platform for the client.
     * This is used to identify the WFM platform making the requests.
     * # Arguments
     * * `wfm_platform` - A string representing the WFM platform (e.g., "web", "mobile").
     */
    pub fn set_wfm_platform(&mut self, wfm_platform: impl Into<String>) {
        self.wfm_platform = wfm_platform.into();
        // Update routes with new client reference
        self.update_routes_client();
    }
    /**
     * Sets the WFM username for the client.
     * This is used to identify the WFM user making the requests.
     * # Arguments
     * * `wfm_username` - A string representing the WFM username.
     */
    pub fn set_wfm_username(&mut self, wfm_username: impl Into<String>) {
        self.wfm_username = wfm_username.into();
        // Update routes with new client reference
        self.update_routes_client();
    }
    /**
     * Sets JWT token for the client.
     * This is used to identify the WFM user ID making the requests.
     * # Arguments
     * * `token` - A string representing the JWT token.
     */
    pub fn set_token(&mut self, token: impl Into<String>) {
        self.token = token.into();
        // Update routes with new client reference
        self.update_routes_client();
    }
    /**
     * Sets the WFM ID for the client.
     * This is used to identify the WFM user ID making the requests.
     * # Arguments
     * * `wfm_id` - A string representing the WFM user ID.
     */
    pub fn set_wfm_id(&mut self, wfm_id: impl Into<String>) {
        self.wfm_id = wfm_id.into();
        // Update routes with new client reference
        self.update_routes_client();
    }
    /**
     * Updates the client reference in the routes.
     * This is useful for cloning routes when the client state changes.
     * This method resets the `self_arc` to force creation of a new Arc with updated data.
     */
    fn update_routes_client(&mut self) {
        // Reset the self_arc to force creation of new Arc with updated data
        self.self_arc = OnceLock::new();

        // If routes existed, recreate them with preserved data and new client reference
        if let Some(old_auth) = self.authentication_route.get().cloned() {
            let new_auth = AuthenticationRoute::from_existing(&old_auth, self.arc());
            self.authentication_route = OnceLock::new();
            let _ = self.authentication_route.set(new_auth);
        }

        if let Some(old_analytics) = self.analytics_route.get().cloned() {
            let new_analytics = AnalyticsRoute::from_existing(&old_analytics, self.arc());
            self.analytics_route = OnceLock::new();
            let _ = self.analytics_route.set(new_analytics);
        }
        if let Some(old_alert) = self.alert_route.get().cloned() {
            let new_alert = AlertRoute::from_existing(&old_alert, self.arc());
            self.alert_route = OnceLock::new();
            let _ = self.alert_route.set(new_alert);
        }
        if let Some(old_cache) = self.cache_route.get().cloned() {
            let new_cache = CacheRoute::from_existing(&old_cache, self.arc());
            self.cache_route = OnceLock::new();
            let _ = self.cache_route.set(new_cache);
        }
        if let Some(old_item_price) = self.item_price_route.get().cloned() {
            let new_item_price = ItemPriceRoute::from_existing(&old_item_price, self.arc());
            self.item_price_route = OnceLock::new();
            let _ = self.item_price_route.set(new_item_price);
        }
    }
}
