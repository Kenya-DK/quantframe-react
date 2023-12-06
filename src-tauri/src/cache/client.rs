use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

use eyre::eyre;
use polars::{
    prelude::{DataFrame, NamedFrom},
    series::Series,
};
use reqwest::{header::HeaderMap, Client, Method, Url};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    auth::AuthState,
    error::AppError,
    helper,
    logger::{self},
    rate_limiter::RateLimiter,
    structs::{Item, RivenAttributeInfo, RivenTypeInfo},
    wfm_client::client::WFMClient,
};

use super::modules::{item::ItemModule, riven::RivenModule};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct CacheDataStruct {
    pub last_refresh: Option<String>,
    pub item: CacheDataItemStruct,
    pub riven: CacheDataRivenStruct,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct CacheDataItemStruct {
    pub items: Vec<Item>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CacheDataRivenStruct {
    pub items: Vec<RivenTypeInfo>,
    pub attributes: Vec<RivenAttributeInfo>,
}

#[derive(Clone, Debug)]
pub struct CacheClient {
    pub log_file: PathBuf,
    pub wfm: Arc<Mutex<WFMClient>>,
    pub cache_data: Arc<Mutex<CacheDataStruct>>,
}

impl CacheClient {
    pub fn new(wfm: Arc<Mutex<WFMClient>>) -> Self {
        CacheClient {
            log_file: PathBuf::from("cache"),
            wfm,
            cache_data: Arc::new(Mutex::new(CacheDataStruct {
                last_refresh: None,
                item: CacheDataItemStruct { items: vec![] },
                riven: CacheDataRivenStruct {
                    items: vec![],
                    attributes: vec![],
                },
            })),
        }
    }
    fn get_file_path() -> PathBuf {
        let app_path = helper::get_app_roaming_path();
        let settings_path = app_path.join("cache.json");
        settings_path
    }

    pub async fn load(&self) -> Result<CacheDataStruct, AppError> {
        let path_ref = Self::get_file_path();

        if path_ref.exists() {
            let (se, vaild) = Self::read_from_file()?;
            if vaild {
                let last_refresh = se.last_refresh.clone();
                match last_refresh {
                    Some(last_refresh) => {
                        let last_refresh = chrono::DateTime::parse_from_rfc3339(&last_refresh)
                            .map_err(|e| AppError::new("Cache", eyre!(e.to_string())))?;
                        let now = chrono::Utc::now();
                        let diff = now.signed_duration_since(last_refresh);
                        if diff.num_hours() < 24 {
                            let arced_mutex = Arc::clone(&self.cache_data);
                            let mut my_lock = arced_mutex.lock()?;
                            my_lock.last_refresh = Some(last_refresh.to_string());
                            my_lock.item = se.item;
                            my_lock.riven = se.riven;
                            return Ok(my_lock.clone());
                        } else {
                            let data = self.refresh().await?;
                            self.save_to_file()?;
                            return Ok(data);
                        }
                    }
                    None => {
                        let data = self.refresh().await?;
                        self.save_to_file()?;
                        return Ok(data);
                    }
                }
            } else {
                let data = self.refresh().await?;
                self.save_to_file()?;
                return Ok(data);
            }
        } else {
            let data = self.refresh().await?;
            self.save_to_file()?;
            return Ok(data);
        }
    }

    pub async fn refresh(&self) -> Result<CacheDataStruct, AppError> {
        self.items().refresh().await?;
        self.riven().refresh().await?;
        self.set_last_refresh(chrono::Utc::now().to_rfc3339())?;
        let cache_data = self.cache_data.lock()?.clone();
        Ok(cache_data)
    }
    pub fn items(&self) -> ItemModule {
        ItemModule { client: self }
    }

    pub fn set_last_refresh(&self, last_refresh: String) -> Result<(), AppError> {
        let arced_mutex = Arc::clone(&self.cache_data);
        let mut my_lock = arced_mutex.lock()?;
        my_lock.last_refresh = Some(last_refresh);
        Ok(())
    }

    pub fn riven(&self) -> RivenModule {
        RivenModule { client: self }
    }

    pub fn save_to_file(&self) -> Result<(), AppError> {
        let chache_data = self.cache_data.clone();
        let json = serde_json::to_string_pretty(&chache_data)
            .map_err(|e| AppError::new("Cache", eyre!(e.to_string())))?;

        let mut file = File::create(Self::get_file_path())
            .map_err(|e| AppError::new("Cache", eyre!(e.to_string())))?;

        file.write_all(json.as_bytes())
            .map_err(|e| AppError::new("Cache", eyre!(e.to_string())))?;

        Ok(())
    }

    pub fn read_from_file() -> Result<(CacheDataStruct, bool), AppError> {
        let mut file = File::open(Self::get_file_path())
            .map_err(|e| AppError::new("Cache", eyre!(e.to_string())))?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| AppError::new("Cache", eyre!(e.to_string())))?;

        Ok(Self::validate_json(&content)?)
    }

    fn validate_json(json_str: &str) -> Result<(CacheDataStruct, bool), AppError> {
        let mut is_valid = true;
        // Parse the JSON string into a Value object
        let mut json_value: Value = serde_json::from_str(json_str)
            .map_err(|e| AppError::new("Cache", eyre!(e.to_string())))?;

        if json_value.get("last_refresh").is_none() {
            let now = chrono::Utc::now();
            // Set the 'last_refresh' property to None
            json_value["last_refresh"] = json!(now.to_rfc3339());
            is_valid = false;
        }

        // Check for nested properties within 'item'
        if let Some(item_data) = json_value.get_mut("item") {
            if item_data.get("items").is_none() {
                item_data["items"] = json!([]);
                is_valid = false;
            }
        }

        // Check for nested properties within 'riven'
        if let Some(riven_data) = json_value.get_mut("riven") {
            if riven_data.get("items").is_none() {
                riven_data["items"] = json!([]);
                is_valid = false;
            }
            if riven_data.get("attributes").is_none() {
                riven_data["attributes"] = json!([]);
                is_valid = false;
            }
        }

        // Deserialize the updated JSON object into a SettingsState struct
        let deserialized: CacheDataStruct = serde_json::from_value(json_value)
            .map_err(|e| AppError::new("Settings", eyre!(e.to_string())))?;
        Ok((deserialized, is_valid))
    }
}
