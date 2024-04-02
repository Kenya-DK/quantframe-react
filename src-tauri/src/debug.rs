use crate::{
    cache::client::CacheClient,
    database::{client::DBClient, modules::transaction::Transaction},
    logger,
};
use eyre::eyre;
use sea_query::{InsertStatement, SqliteQueryBuilder};

use serde_json::{json, Value};
use sqlx::{Pool, Row, Sqlite, SqlitePool};
use std::{
    fs,
    io::Read as _,
    path::Path,
    sync::{Arc, Mutex},
};

#[derive(Clone, Debug)]
pub struct DebugClient {
    log_file: String,
    trades: Vec<Value>,
    cache: Arc<Mutex<CacheClient>>,
    db: Arc<Mutex<DBClient>>,
}

impl DebugClient {
    pub fn new(cache: Arc<Mutex<CacheClient>>, db: Arc<Mutex<DBClient>>) -> Self {
        DebugClient {
            log_file: "debug.log".to_string(),
            cache,
            trades: Vec::new(),
            db,
        }
    }
}
