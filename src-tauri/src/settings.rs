use crate::database;
use crate::helper;
use crate::helper::ColumnType;
use crate::helper::ColumnValue;
use crate::helper::ColumnValues;
use crate::logger;
use crate::price_scraper;
use crate::structs::GlobleError;
use crate::structs::Order;
use crate::structs::Settings;
use crate::wfm_client;
use polars::prelude::*;
use serde_json::json;
use std::collections::HashSet;
use tokio::sync::Mutex;
extern crate csv;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;

// Structs for the Warframe Market API

#[derive(Clone, Debug)]
pub struct Settings2 {
    volume_threshold: i32,
    range_threshold: i32,
    avg_price_cap: i32,
    max_total_price_cap: i32,
    price_shift_threshold: i32,
    blacklist: Vec<String>,
    whitelist: Vec<String>,
    strict_whitelist: bool,
}
// Allow us to run AuthState::default()
impl Default for Settings2 {
    fn default() -> Self {
        Self {
            volume_threshold: 15,
            range_threshold: 10,
            avg_price_cap: 600,
            max_total_price_cap: 100000,
            price_shift_threshold: -1,
            blacklist: vec![],
            whitelist: vec![],
            strict_whitelist: false,
        }
    }
}
