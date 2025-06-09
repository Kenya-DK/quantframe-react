use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    },
    time::{Duration, Instant},
};

use serde_json::json;

use crate::{
    enums::{stock_mode::StockMode, trade_mode::TradeMode},
    logger,
    utils::{
        enums::{log_level::LogLevel, ui_events::UIEvent},
        modules::{error::AppError, logger::LoggerOptions, states},
    },
};

use super::modules::{item::ItemModule, riven::RivenModule};

#[derive(Clone)]
pub struct LiveScraperClient {
    pub log_file: &'static str,
    pub component: String,
    riven_module: Arc<RwLock<Option<RivenModule>>>,
    item_module: Arc<RwLock<Option<ItemModule>>>,
    pub is_running: Arc<AtomicBool>,
}

impl LiveScraperClient {
    pub fn new() -> Self {
        LiveScraperClient {
            log_file: "live_scraper.log",
            component: "LiveScraper".to_string(),
            is_running: Arc::new(AtomicBool::new(false)),
            riven_module: Arc::new(RwLock::new(None)),
            item_module: Arc::new(RwLock::new(None)),
        }
    }
    pub fn report_error(&self, error: &AppError) {
        let notify = states::notify_client().unwrap();
        let component = error.component();
        let log_level = error.log_level();
        if log_level == LogLevel::Critical || log_level == LogLevel::Error {
            self.is_running.store(false, Ordering::SeqCst);
        }
        crate::logger::dolog(
            log_level.clone(),
            format!("{}:{}", self.component, component).as_str(),
            &error.to_string(),
            LoggerOptions::default()
                .set_console(true)
                .set_file(self.log_file),
        );
        notify.gui().send_event(
            crate::utils::enums::ui_events::UIEvent::OnLiveTradingError,
            Some(json!(error)),
        );
    }

    pub fn stop_loop(&self) {
        self.send_gui_update("idle", None);
        self.is_running.store(false, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    pub fn set_can_run(&self, can_run: bool) {
        let notify = states::notify_client().unwrap();
        notify.gui().send_event(
            UIEvent::OnToggleControl,
            Some(json!({"id": "live_trading", "state": can_run})),
        );
    }

    pub fn start_loop(&mut self) -> Result<(), AppError> {
        self.is_running.store(true, Ordering::SeqCst);
        let is_running = Arc::clone(&self.is_running);
        let scraper = self.clone();
        // Reset riven stocks on start
        tauri::async_runtime::spawn(async move {
            logger::info(
                &scraper.component,
                "Loop live scraper is started",
                LoggerOptions::default(),
            );

            let mut settings = states::settings().unwrap().clone();

            // Check if StockMode is set to Riven or All
            if settings.live_scraper.stock_mode == StockMode::Riven
                || settings.live_scraper.stock_mode == StockMode::All
            {
                // db.stock_riven().reset_listed_price().await.unwrap();
            }

            if settings.live_scraper.stock_mode == StockMode::Item
                || settings.live_scraper.stock_mode == StockMode::All
            {
                if settings.live_scraper.stock_item.auto_delete {
                    scraper
                        .item()
                        .delete_all_orders(vec![TradeMode::Buy, TradeMode::Sell])
                        .await
                        .unwrap();
                }
            }

            // Start Riven last update timer
            let riven_interval = 1; // 5 min
            let mut last_riven_update = Instant::now() - Duration::from_secs(riven_interval + 20);

            while is_running.load(Ordering::SeqCst) {
                settings = states::settings().unwrap().clone();

                if (settings.live_scraper.stock_mode == StockMode::Riven
                    || settings.live_scraper.stock_mode == StockMode::All)
                    && last_riven_update.elapsed() > Duration::from_secs(riven_interval)
                {
                    last_riven_update = Instant::now();
                    match scraper.riven().check_stock().await {
                        Ok(_) => {}
                        Err(e) => scraper.report_error(&e),
                    }
                }

                if settings.live_scraper.stock_mode == StockMode::Item
                    || settings.live_scraper.stock_mode == StockMode::All
                {
                    match scraper.item().check_stock().await {
                        Ok(_) => {}
                        Err(e) => scraper.report_error(&e),
                    }
                }
                // scraper.stop_loop();
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            logger::info(
                &scraper.component,
                "Loop live scraper is stopped",
                LoggerOptions::default(),
            );
            states::notify_client().unwrap().gui().send_event(
                crate::utils::enums::ui_events::UIEvent::UpdateLiveTradingRunningState,
                Some(json!(false)),
            );
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

    pub fn send_gui_update(&self, i18n_key: &str, values: Option<serde_json::Value>) {
        let notify = states::notify_client().unwrap();
        if self.is_running() {
            notify.gui().send_event(
                crate::utils::enums::ui_events::UIEvent::OnLiveTradingMessage,
                Some(json!({ "i18nKey": i18n_key, "values": values })),
            );
        }
    }
}
