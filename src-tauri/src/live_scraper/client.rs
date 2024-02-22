use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, RwLock
    },
    time::Duration,
};

use polars::frame::DataFrame;
use serde_json::json;

use crate::{
    auth::AuthState,
    cache::client::CacheClient,
    database::client::DBClient,
    enums::{LogLevel, OrderMode, StockMode},
    error::AppError,
    handler::MonitorHandler,
    helper,
    logger::{self},
    price_scraper::PriceScraper,
    settings::SettingsState,
    wfm_client::client::WFMClient,
};

use super::modules::{item::ItemModule, riven::RivenModule};

#[derive(Clone)]
pub struct LiveScraperClient {
    pub log_file: String,
    component: String,
    riven_module:Arc<RwLock<Option<RivenModule>>>,
    item_module:Arc<RwLock<Option<ItemModule>>>,
    pub cache_querieds: Arc<Mutex<HashMap<String, DataFrame>>>,
    pub is_running: Arc<AtomicBool>,
    pub settings: Arc<Mutex<SettingsState>>,
    pub price_scraper: Arc<Mutex<PriceScraper>>,
    pub wfm: Arc<Mutex<WFMClient>>,
    pub auth: Arc<Mutex<AuthState>>,
    pub db: Arc<Mutex<DBClient>>,
    pub cache: Arc<Mutex<CacheClient>>,
    pub mh: Arc<Mutex<MonitorHandler>>,
}

impl LiveScraperClient {
    pub fn new(
        settings: Arc<Mutex<SettingsState>>,
        price_scraper: Arc<Mutex<PriceScraper>>,
        wfm: Arc<Mutex<WFMClient>>,
        auth: Arc<Mutex<AuthState>>,
        db: Arc<Mutex<DBClient>>,
        cache: Arc<Mutex<CacheClient>>,
        mh: Arc<Mutex<MonitorHandler>>,
    ) -> Self {
        LiveScraperClient {
            log_file: "live_scraper.log".to_string(),
            component: "LiveScraper".to_string(),
            cache_querieds: Arc::new(Mutex::new(HashMap::new())),
            price_scraper,
            settings,
            is_running: Arc::new(AtomicBool::new(false)),
            wfm,
            auth,
            db,
            cache,
            mh,
            riven_module:Arc::new(RwLock::new(None)),
            item_module:Arc::new(RwLock::new(None)),
        }
    }
    fn report_error(&self, error: AppError) {
        let component = error.component();
        let cause = error.cause();
        let backtrace = error.backtrace();
        let log_level = error.log_level();
        let extra = error.extra_data();
        if log_level == LogLevel::Critical || log_level == LogLevel::Error {
            self.is_running.store(false, Ordering::SeqCst);
            crate::logger::dolog(
                log_level.clone(),
                format!("{}:{}", self.component, component).as_str(),
                format!("{}, {}, {}", backtrace, cause, extra.to_string()).as_str(),
                true,
                Some(self.log_file.as_str()),
            );
            helper::send_message_to_window("LiveScraper:Error", Some(error.to_json()));
        } else {
            crate::logger::dolog(
                log_level.clone(),
                format!("{}:{}", self.component, component).as_str(),
                format!("{}, {}, {}", backtrace, cause, extra.to_string()).as_str(),
                true,
                Some(self.log_file.as_str()),
            );
        }
    }
    pub fn debug(&self, id: &str, component: &str, msg: &str, file: Option<bool>) {
        let settings = self.settings.lock().unwrap().clone();
        if !settings.debug.contains(&"*".to_owned()) && !settings.debug.contains(&id.to_owned()) {
            return;
        }

        if file.is_none() {
            logger::debug(
                format!("{}:{}", self.component, component).as_str(),
                msg,
                true,
                None,
            );
            return;
        }
        logger::debug(
            format!("{}:{}", self.component, component).as_str(),
            msg,
            true,
            Some(&self.log_file),
        );
    }
    pub fn stop_loop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    pub fn start_loop(&mut self) -> Result<(), AppError> {
        self.is_running.store(true, Ordering::SeqCst);
        let is_running = Arc::clone(&self.is_running);
        let forced_stop = Arc::clone(&self.is_running);
        let scraper = self.clone();
        let db = self.db.lock()?.clone();
        // Reset riven stocks on start
        tauri::async_runtime::spawn(async move {
            logger::info_con(&scraper.component, "Loop live scraper is started");

            let mut settings = scraper.settings.lock().unwrap().clone();
            scraper.send_message("riven.reset", None);
            db.stock_riven().reset_listed_price().await.unwrap();
            scraper.send_message("item.reset", None);
            if settings.live_scraper.stock_item.auto_delete {
                db.stock_item().reset_listed_price().await.unwrap();
                scraper
                    .item()
                    .delete_all_orders(OrderMode::Both)
                    .await
                    .unwrap();
            }

            let riven_interval = 5;
            let mut current_riven_interval = riven_interval.clone();

            while is_running.load(Ordering::SeqCst) && forced_stop.load(Ordering::SeqCst) {
                settings = scraper.settings.lock().unwrap().clone();

                if (settings.live_scraper.stock_mode == StockMode::Riven
                    || settings.live_scraper.stock_mode == StockMode::All)
                    && current_riven_interval >= riven_interval
                {
                    current_riven_interval = 0;
                    scraper.send_message("riven.starting", None);
                    match scraper.riven().check_stock().await {
                        Ok(_) => {}
                        Err(e) => scraper.report_error(e),
                    }
                }

                if settings.live_scraper.stock_mode == StockMode::Item
                    || settings.live_scraper.stock_mode == StockMode::All
                {
                    scraper.send_message("riven.starting", None);
                    match scraper.item().check_stock().await {
                        Ok(_) => {}
                        Err(e) => scraper.report_error(e),
                    }
                }
                current_riven_interval += 1;
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            scraper.send_message("", None);
            logger::info_con(&scraper.component, "Loop live scraper is stopped");
        });
        Ok(())
    }
    pub fn item(&self) -> ItemModule {
        // Lazily initialize ItemModule if not already initialized
        if self.item_module.read().unwrap().is_none() {
            *self.item_module.write().unwrap() = Some(ItemModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the item_module is initialized
        self.item_module.read().unwrap().as_ref().unwrap().clone()
    }
    pub fn update_item_module(&self, module: ItemModule) {
        // Update the stored ItemModule
        *self.item_module.write().unwrap() = Some(module);
    }
    // pub fn item(&self) -> ItemModule {
    //     ItemModule {
    //         client: self,
    //         debug_id: "live_scraper_item".to_string(),
    //     }
    // }
    pub fn riven(&self) -> RivenModule {
        // Lazily initialize ItemModule if not already initialized
        if self.riven_module.read().unwrap().is_none() {
            *self.riven_module.write().unwrap() = Some(RivenModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the riven_module is initialized
        self.riven_module.read().unwrap().as_ref().unwrap().clone()
    }
    pub fn update_riven_module(&self, module: RivenModule) {
        // Update the stored ItemModule
        *self.riven_module.write().unwrap() = Some(module);
    }

    pub fn send_message(&self, i18n_key: &str, data: Option<serde_json::Value>) {
        helper::send_message_to_window(
            "LiveScraper:UpdateMessage",
            Some(json!({
                "i18n_key": i18n_key,
                "values": data
            })),
        );
    }
    pub fn add_cache_queried(&self, key: String, df: DataFrame) {
        self.cache_querieds.lock().unwrap().insert(key, df);
    }

    pub fn get_cache_queried(&self, key: &str) -> Option<DataFrame> {
        self.cache_querieds.lock().unwrap().get(key).cloned()
    }

    pub fn remove_cache_queried(&self, key: &str) {
        self.cache_querieds
            .lock()
            .unwrap()
            .retain(|k, _| !k.starts_with(key));
    }
}
