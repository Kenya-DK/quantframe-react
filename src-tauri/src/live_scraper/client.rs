use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use crate::{
    auth::AuthState,
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
    pub is_running: Arc<AtomicBool>,
    pub settings: Arc<Mutex<SettingsState>>,
    pub price_scraper: Arc<Mutex<PriceScraper>>,
    pub wfm: Arc<Mutex<WFMClient>>,
    pub auth: Arc<Mutex<AuthState>>,
    pub db: Arc<Mutex<DBClient>>,
    pub mh: Arc<Mutex<MonitorHandler>>,
}

impl LiveScraperClient {
    pub fn new(
        settings: Arc<Mutex<SettingsState>>,
        price_scraper: Arc<Mutex<PriceScraper>>,
        wfm: Arc<Mutex<WFMClient>>,
        auth: Arc<Mutex<AuthState>>,
        db: Arc<Mutex<DBClient>>,
        mh: Arc<Mutex<MonitorHandler>>,
    ) -> Self {
        LiveScraperClient {
            log_file: "live_scraper.log".to_string(),
            price_scraper,
            settings,
            is_running: Arc::new(AtomicBool::new(false)),
            wfm,
            auth,
            db,
            mh,
        }
    }
    fn report_error(&self, error: AppError) {
        let component = error.component();
        let cause = error.cause();
        let backtrace = error.backtrace();
        let log_level = error.log_level();
        let extra = error.extra_data();
        if log_level == LogLevel::Critical {
            self.is_running.store(false, Ordering::SeqCst);
            crate::logger::dolog(
                log_level.clone(),
                component.as_str(),
                format!("Error: {:?}, {:?}, {:?}", backtrace, cause, extra).as_str(),
                true,
                Some(self.log_file.as_str()),
            );
            helper::send_message_to_window("LiveScraper:Error", Some(error.to_json()));
        } else {
            logger::info_con(
                "LiveScraper",
                format!("Error: {:?}, {:?}", backtrace, cause).as_str(),
            );
        }
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
            logger::info_con("LiveScraper", "Loop live scraper is started");

            db.stock_riven().reset_listed_price().await.unwrap();
            db.stock_item().reset_listed_price().await.unwrap();
            scraper
                .item()
                .delete_all_orders(OrderMode::Both)
                .await
                .unwrap();
            while is_running.load(Ordering::SeqCst) && forced_stop.load(Ordering::SeqCst) {
                let settings = scraper.settings.lock().unwrap().clone();
                if settings.live_scraper.stock_mode == StockMode::Riven
                    || settings.live_scraper.stock_mode == StockMode::All
                {
                    logger::info_con("LiveScraper", "Checking riven stock");
                    match scraper.riven().check_stock().await {
                        Ok(_) => {}
                        Err(e) => scraper.report_error(e),
                    }
                }

                if settings.live_scraper.stock_mode == StockMode::Item
                    || settings.live_scraper.stock_mode == StockMode::All
                {
                    logger::info_con("LiveScraper", "Checking item stock");
                    match scraper.item().check_stock().await {
                        Ok(_) => {}
                        Err(e) => scraper.report_error(e),
                    }
                }

                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            logger::info_con("LiveScraper", "Loop live scraper is stopped");
        });
        Ok(())
    }
    pub fn item(&self) -> ItemModule {
        ItemModule { client: self }
    }
    pub fn riven(&self) -> RivenModule {
        RivenModule { client: self }
    }
}
