use std::sync::{Arc, Mutex};

use eyre::eyre;
use polars::{
    prelude::{DataFrame, NamedFrom},
    series::Series,
};
use reqwest::{header::HeaderMap, Client, Method, Url};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};

use crate::{
    auth::AuthState,
    cache::CacheState,
    error::AppError,
    helper,
    logger::{self, LogLevel},
    wfm_client::client::WFMClient,
};

use super::modules::{ transaction::TransactionModule, stock_item::StockItemModule, stock_riven::StockRivenModule};
#[derive(Clone, Debug)]
pub struct DBClient {
    pub log_file: String,
    pub connection: Arc<Mutex<Pool<Sqlite>>>,
    pub cache: Arc<Mutex<CacheState>>,
    pub wfm: Arc<Mutex<WFMClient>>,
}

impl DBClient {
    pub async fn new(
        cache: Arc<Mutex<CacheState>>,
        wfm: Arc<Mutex<WFMClient>>,
    ) -> Result<Self, AppError> {
        let log_file = "db.log";
        let mut db_url = helper::get_app_roaming_path();
        db_url.push("quantframe.sqlite");
        let db_url: &str = db_url.to_str().unwrap();
        if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
            match Sqlite::create_database(db_url).await {
                Ok(_) => logger::info_con(
                    "Database",
                    format!("Database created at {}", db_url).as_str(),
                ),
                Err(error) => logger::critical(
                    "Database",
                    format!("Error creating database: {:?}", error).as_str(),
                    true,
                    Some(log_file),
                ),
            }
        }
        Ok(DBClient {
            log_file: log_file.to_string(),
            connection: Arc::new(Mutex::new(SqlitePool::connect(db_url).await.unwrap())),
            cache,
            wfm,
        })
    }
    pub async fn initialize(&self) -> Result<bool, AppError> {
        self.stock_item().initialize().await?;
        self.stock_riven().initialize().await?;
        self.transaction().initialize().await?;
        Ok(true)
    }
    pub fn get_connection(&self) -> Arc<Mutex<Pool<Sqlite>>> {
        self.connection.clone()
    }

    pub fn transaction(&self) -> TransactionModule {
        TransactionModule { client: self }
    }

    pub fn stock_item(&self) -> StockItemModule {
        StockItemModule { client: self }
    }

    pub fn stock_riven(&self) -> StockRivenModule {
        StockRivenModule { client: self }
    }
}
