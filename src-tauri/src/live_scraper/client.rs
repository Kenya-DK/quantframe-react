use crate::{enums::*, live_scraper::modules::*, utils::modules::states};
use serde_json::Value;
use std::{
    collections::HashMap,
    num::NonZeroU32,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, OnceLock,
    },
};
use utils::{warning, LoggerOptions};

#[derive(Debug)]
pub struct LiveScraperState {
    pub is_running: AtomicBool,
    item_module: OnceLock<Arc<ItemModule>>,
    riven_module: OnceLock<Arc<RivenModule>>,
}

impl LiveScraperState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            is_running: AtomicBool::new(false),
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
        if self.is_running.swap(true, Ordering::SeqCst) {
            warning(
                "LiveScraper:Start",
                "Live Scraper is already running",
                LoggerOptions::default(),
            );
            return;
        }

        self.init_modules();
        let this = self.clone();

        tauri::async_runtime::spawn(async move {
            while this.is_running.load(Ordering::SeqCst) {
                let app = states::app_state().expect("App state not initialized");
                if matches!(
                    app.settings.live_scraper.stock_mode,
                    StockMode::Riven | StockMode::All
                ) {
                    let _ = this.riven().check().await;
                }

                if matches!(
                    app.settings.live_scraper.stock_mode,
                    StockMode::Item | StockMode::All
                ) {
                    let _ = this.item().check().await;
                }

                println!("Live Scraper is running...");
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
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
        if let Some(old_item) = self.item_module.get().cloned() {
            let new_item = ItemModule::from_existing(&old_item, self.clone());
            let _ = self.item_module.set(new_item);
        }

        if let Some(old_riven) = self.riven_module.get().cloned() {
            let new_riven = RivenModule::from_existing(&old_riven, self.clone());
            let _ = self.riven_module.set(new_riven);
        }
    }
}
