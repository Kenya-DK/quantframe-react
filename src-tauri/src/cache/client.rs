use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

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
    rate_limiter::RateLimiter,
};

use super::modules::{
    auction::AuctionModule, riven::RivenModule, item::ItemModule, order::OrderModule,
};

#[derive(Clone, Debug)]
pub struct CacheClient {
    pub log_file: PathBuf,
    pub wfm: Arc<Mutex<WFMClient>>,
}

impl CacheClient {
    pub fn new(wfm: Arc<Mutex<WFMClient>>) -> Self {
        CacheClient {
            log_file: PathBuf::from("cache"),
            wfm,
        }
    }
    pub async fn refresh(&self) -> Result<(), AppError> {
        self.items().refresh().await?;
        self.riven().refresh().await?;
        Ok(()) 
    }
    pub fn items(&self) -> ItemModule {
        ItemModule { client: self, items: Arc::new(Mutex::new(vec![])), }
    }

    pub fn riven(&self) -> RivenModule {
        RivenModule { client: self, rivens: Arc::new(Mutex::new(vec![])), }
    }
}
