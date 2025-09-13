use crate::{
    emit_error,
    enums::*,
    live_scraper::modules::*,
    utils::{modules::states, OrderListExt},
};
use serde_json::Value;
use std::{
    collections::HashMap,
    num::NonZeroU32,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, OnceLock,
    },
};
use utils::{critical, get_location, warning, LogLevel, LoggerOptions};

#[derive(Debug)]
pub struct LiveScraperState {
    pub is_running: Arc<AtomicBool>,
    pub just_started: Arc<AtomicBool>,
    item_module: OnceLock<Arc<ItemModule>>,
    riven_module: OnceLock<Arc<RivenModule>>,
}

impl LiveScraperState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            is_running: Arc::new(AtomicBool::new(false)),
            just_started: Arc::new(AtomicBool::new(true)),
            item_module: OnceLock::new(),
            riven_module: OnceLock::new(),
        })
    }

    fn init_modules(self: &Arc<Self>) {
        self.item_module
            .get_or_init(|| ItemModule::new(self.clone()));
        self.riven_module
            .get_or_init(|| RivenModule::new(self.clone()));
    }

    pub fn start(self: &Arc<Self>) {
        let settings = states::get_settings().expect("Settings not initialized");
        let app = states::app_state().expect("App state not initialized");

        if self.is_running.swap(true, Ordering::SeqCst) {
            warning(
                "LiveScraper:Start",
                "Live Scraper is already running",
                &LoggerOptions::default(),
            );
            return;
        }
        self.just_started.store(true, Ordering::SeqCst);
        if settings.live_scraper.stock_mode == StockMode::All
            || settings.live_scraper.stock_mode == StockMode::Item
        {
            match app.wfm_client.order().cache_orders_mut().apply_trade_info() {
                Ok(_) => {}
                Err(e) => {
                    e.with_location(get_location!())
                        .log("live_scraper.log");
                }
            }
        }
        self.init_modules();
        let is_running = Arc::clone(&self.is_running);
        let just_started = Arc::clone(&self.just_started);
        let this = self.clone();
        tauri::async_runtime::spawn({
            async move {
                while is_running.load(Ordering::SeqCst) {
                    let app = states::app_state().expect("App state not initialized");
                    if matches!(
                        app.settings.live_scraper.stock_mode,
                        StockMode::Riven | StockMode::All
                    ) {
                        match this.riven().check().await {
                            Ok(_) => {}
                            Err(e) => {
                                e.clone()
                                    .with_location(get_location!())
                                    .log("live_scraper_riven.log");
                                match e.log_level {
                                    LogLevel::Critical | LogLevel::Error => {
                                        // Stop the live scraper
                                        is_running.store(false, Ordering::SeqCst);
                                        emit_error!(e);
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }

                    if matches!(
                        app.settings.live_scraper.stock_mode,
                        StockMode::Item | StockMode::All
                    ) {
                        match this.item().check().await {
                            Ok(_) => {}
                            Err(e) => {
                                e.clone()
                                    .with_location(get_location!())
                                    .log("live_scraper_item.log");
                                match e.log_level {
                                    LogLevel::Critical | LogLevel::Error => {
                                        // Stop the live scraper
                                        is_running.store(false, Ordering::SeqCst);
                                        emit_error!(e);
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    just_started.store(false, Ordering::SeqCst);
                }
            }
        });
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }

    pub fn item(&self) -> Arc<ItemModule> {
        self.item_module
            .get()
            .expect("Item module not initialized")
            .clone()
    }

    pub fn riven(&self) -> Arc<RivenModule> {
        self.riven_module
            .get()
            .expect("Riven module not initialized")
            .clone()
    }

    /// Use this if you need to replace modules while preserving shared state
    pub fn update_modules(self: &Arc<Self>) {
        if let Some(_old_item) = self.item_module.get().cloned() {
            critical(
                "LiveScraper:UpdateModules",
                "Not implemented",
                &LoggerOptions::default(),
            );
        }

        if let Some(old_riven) = self.riven_module.get().cloned() {
            let new_riven = RivenModule::from_existing(&old_riven, self.clone());
            let _ = self.riven_module.set(new_riven);
        }
    }
}
