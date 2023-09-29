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
pub struct LiveScraperClient {
    pub log_file: String,
    pub is_running: Arc<AtomicBool>,
    pub settings: Arc<Mutex<SettingsState>>,
    pub price_scraper: Arc<Mutex<PriceScraper>>,
    pub wfm: Arc<Mutex<WFMClient>>,
    pub auth: Arc<Mutex<AuthState>>,
    pub db: Arc<Mutex<DBClient>>,
}

impl LiveScraperClient {
    pub fn new(
        settings: Arc<Mutex<SettingsState>>,
        price_scraper: Arc<Mutex<PriceScraper>>,
        wfm: Arc<Mutex<WFMClient>>,
        auth: Arc<Mutex<AuthState>>,
        db: Arc<Mutex<DBClient>>
    ) -> Self {
        LiveScraperClient {
            settings: Arc<Mutex<SettingsState>>,
            price_scraper: Arc<Mutex<PriceScraper>>,
            wfm: Arc<Mutex<WFMClient>>,
            auth: Arc<Mutex<AuthState>>,
            db: Arc<Mutex<DBClient>>,
        }
    }    

    pub fn start_loop(&mut self) -> Result<(), AppError> {
        self.is_running.store(true, Ordering::SeqCst);
        let is_running = Arc::clone(&self.is_running);
        let forced_stop = Arc::clone(&self.is_running);
        let scraper = self.clone();
        tauri::async_runtime::spawn(async move {
            
            logger::info_con("LiveScraper", "Loop live scraper is started");
            match scraper.delete_all_orders().await {
                Ok(_) => {
                    logger::info_con("LiveScraper", "Delete all orders success");
                }
                Err(e) => scraper.report_error(e),
            }

            while is_running.load(Ordering::SeqCst) && forced_stop.load(Ordering::SeqCst) {
                logger::info_con("LiveScraper", "Checking item stock");
                match scraper.riven().check_stock().await {
                    Ok(_) => {}
                    Err(e) => scraper.report_error(e),
                }                
                
                logger::info_con("LiveScraper", "Checking riven stock");
                match scraper.item().check_stock().await {
                    Ok(_) => {}
                    Err(e) => scraper.report_error(e),
                } 

                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            logger::info_con("LiveScraper", "Loop live scraper is stopped");
        });
    }
    pub fn item(&self) -> ItemModule {
        ItemModule { client: self }
    }
    pub fn riven(&self) -> RivenModule {
        RivenModule { client: self }
    }
}
