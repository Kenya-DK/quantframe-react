use std::{
    sync::{Arc, Mutex},
    time::Duration, path::PathBuf,
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
    rate_limiter::RateLimiter, wfm_client::client::WFMClient, structs::{Item, RivenTypeInfo, RivenAttributeInfo},
};

use super::modules::{
     item::ItemModule,
        riven::RivenModule,
};

#[derive(Clone, Debug)]
pub struct CacheClient {
    pub log_file: PathBuf,
    pub wfm: Arc<Mutex<WFMClient>>,
    pub items: Arc<Mutex<Vec<Item>>>,
    pub riven_types: Arc<Mutex<Vec<RivenTypeInfo>>>,
    pub riven_attributes: Arc<Mutex<Vec<RivenAttributeInfo>>>,
}

impl CacheClient {
    pub fn new(wfm: Arc<Mutex<WFMClient>>) -> Self {
        CacheClient {
            log_file: PathBuf::from("cache"),
            wfm,
            items: Arc::new(Mutex::new(vec![])),
            riven_types: Arc::new(Mutex::new(vec![])), 
            riven_attributes: Arc::new(Mutex::new(vec![]))
        }
    }
    
    pub async fn refresh(&self) -> Result<(), AppError> {
        self.items().refresh().await?;
        self.riven().refresh().await?;
        Ok(()) 
    }
    pub fn items(&self) -> ItemModule {
        ItemModule { client: self, }
    }

    pub fn riven(&self) -> RivenModule {
        RivenModule { client: self}
    }
}
