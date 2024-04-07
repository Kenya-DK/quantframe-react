use crate::{
    cache::client::CacheClient,
    logger,
};
use eyre::eyre;

use serde_json::{json, Value};
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
}

impl DebugClient {
    pub fn new(cache: Arc<Mutex<CacheClient>>) -> Self {
        DebugClient {
            log_file: "debug.log".to_string(),
            cache,
            trades: Vec::new()
        }
    }
}
