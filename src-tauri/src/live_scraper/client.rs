use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, RwLock,
    },
    time::Duration,
};

use serde_json::json;

use crate::{
    app::client::AppState, auth::AuthState, cache::client::CacheClient, enums::{order_mode::OrderMode, stock_mode::StockMode}, helper, logger, notification::client::NotifyClient, settings::SettingsState, utils::{enums::log_level::LogLevel, modules::error::AppError}, wfm_client::client::WFMClient
};

use super::modules::{item::ItemModule, riven::RivenModule};

#[derive(Clone)]
pub struct LiveScraperClient {
    pub log_file: String,
    pub component: String,
    riven_module: Arc<RwLock<Option<RivenModule>>>,
    item_module: Arc<RwLock<Option<ItemModule>>>,
    pub is_running: Arc<AtomicBool>,
    pub settings: Arc<Mutex<SettingsState>>,
    pub wfm: Arc<Mutex<WFMClient>>,
    pub auth: Arc<Mutex<AuthState>>,
    pub cache: Arc<Mutex<CacheClient>>,
    pub notify: Arc<Mutex<NotifyClient>>,
    pub app: Arc<Mutex<AppState>>,
}

impl LiveScraperClient {
    pub fn new(
        app: Arc<Mutex<AppState>>,
        settings: Arc<Mutex<SettingsState>>,
        wfm: Arc<Mutex<WFMClient>>,
        auth: Arc<Mutex<AuthState>>,
        cache: Arc<Mutex<CacheClient>>,
        notify: Arc<Mutex<NotifyClient>>,
    ) -> Self {
        LiveScraperClient {
            log_file: "live_scraper.log".to_string(),
            component: "LiveScraper".to_string(),
            settings,
            is_running: Arc::new(AtomicBool::new(false)),
            app,
            wfm,
            auth,
            cache,
            notify,
            riven_module: Arc::new(RwLock::new(None)),
            item_module: Arc::new(RwLock::new(None)),
        }
    }
    pub fn report_error(&self, error: &AppError) {
        let notify = self.notify.lock().unwrap().clone();
        let component = error.component();
        let cause = error.cause();
        let backtrace = error.backtrace();
        let log_level = error.log_level();
        let extra = error.extra_data();
        if log_level == LogLevel::Critical || log_level == LogLevel::Error {
            self.is_running.store(false, Ordering::SeqCst);
        } 
        crate::logger::dolog(
            log_level.clone(),
            format!("{}:{}", self.component, component).as_str(),
            format!("{}, {}, {}", backtrace, cause, extra.to_string()).as_str(),
            true,
            Some(self.log_file.as_str()),
        );
        notify.gui().send_event(crate::utils::enums::ui_events::UIEvent::OnLiveTradingError, Some(json!(error)));
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
        self.send_gui_update("idle", None);
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
        // Reset riven stocks on start
        tauri::async_runtime::spawn(async move {
            logger::info_con(&scraper.component, "Loop live scraper is started");

            let mut settings = scraper.settings.lock().unwrap().clone();

            // Check if StockMode is set to Riven or All
            if settings.live_scraper.stock_mode == StockMode::Riven
                || settings.live_scraper.stock_mode == StockMode::All
            {
                // db.stock_riven().reset_listed_price().await.unwrap();
            }

            if settings.live_scraper.stock_mode == StockMode::Item
                || settings.live_scraper.stock_mode == StockMode::All
            {
                // db.stock_item().reset_listed_price().await.unwrap();
                if settings.live_scraper.stock_item.auto_delete {
                    scraper
                        .item()
                        .delete_all_orders(OrderMode::Both)
                        .await
                        .unwrap();
                }
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
                current_riven_interval += 1;
                tokio::time::sleep(Duration::from_secs(1)).await;
            }            
            logger::info_con(&scraper.component, "Loop live scraper is stopped");
            scraper.notify.lock().unwrap().gui().send_event(crate::utils::enums::ui_events::UIEvent::UpdateLiveTradingRunningState, Some(json!(false)));
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
        let notify = self.notify.lock().unwrap().clone();
        if self.is_running() {
            notify.gui().send_event(crate::utils::enums::ui_events::UIEvent::OnLiveTradingMessage, Some(json!({ "i18nKey": i18n_key, "values": values })));            
        }
    }
}
