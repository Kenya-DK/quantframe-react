use crate::{
    emit_error,
    enums::*,
    live_scraper::modules::*,
    play_sound, send_event,
    types::UIEvent,
    utils::{modules::states, OrderListExt},
};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, OnceLock,
    },
    time::{Duration, Instant},
};
use utils::{get_location, warning, LogLevel, LoggerOptions};

<<<<<<< HEAD
use serde_json::json;

use crate::{
    enums::{stock_mode::StockMode, trade_mode::TradeMode},
    helper::add_metric,
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
=======
#[derive(Debug)]
pub struct LiveScraperState {
>>>>>>> better-backend
    pub is_running: Arc<AtomicBool>,
    pub just_started: Arc<AtomicBool>,
    item_module: OnceLock<Arc<ItemModule>>,
    riven_module: OnceLock<Arc<RivenModule>>,
}

impl LiveScraperState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            is_running: Arc::new(AtomicBool::new(false)),
<<<<<<< HEAD
            riven_module: Arc::new(RwLock::new(None)),
            item_module: Arc::new(RwLock::new(None)),
        }
    }
    pub fn report_error(&self, error: &AppError) {
        let notify = states::notify_client().unwrap();
        let component = error.component();
        let log_level = error.log_level();
        if log_level == LogLevel::Critical || log_level == LogLevel::Error {
            add_metric("LiveScraper_Error", log_level.as_str());
            // self.is_running.store(false, Ordering::SeqCst);
            notify.gui().send_event(
                crate::utils::enums::ui_events::UIEvent::OnLiveTradingError,
                Some(json!(error)),
            );
        }
        crate::logger::dolog(
            log_level.clone(),
            format!("{}:{}", self.component, component).as_str(),
            &error.to_string(),
            LoggerOptions::default()
                .set_console(true)
                .set_file(self.log_file),
        );
=======
            just_started: Arc::new(AtomicBool::new(true)),
            item_module: OnceLock::new(),
            riven_module: OnceLock::new(),
        })
>>>>>>> better-backend
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
                    e.with_location(get_location!()).log("live_scraper.log");
                }
            }
        }
        self.init_modules();
        let is_running = Arc::clone(&self.is_running);
        let just_started = Arc::clone(&self.just_started);
        let this = self.clone();
        tauri::async_runtime::spawn({
            async move {
                // Start Riven last update timer
                let riven_interval = settings.live_scraper.stock_riven.update_interval as u64;
                let mut last_riven_update =
                    Instant::now() - Duration::from_secs(riven_interval * 2);

                while is_running.load(Ordering::SeqCst) {
                    let app = states::app_state().expect("App state not initialized");
                    if matches!(
                        app.settings.live_scraper.stock_mode,
                        StockMode::Riven | StockMode::All
                    ) {
                        // Check Time
                        let time_elapsed = last_riven_update.elapsed();
                        if time_elapsed > Duration::from_secs(riven_interval) {
                            last_riven_update = Instant::now();
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
                                            play_sound!("windows_xp_error.mp3", 1.0);
                                            emit_error!(e);
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        } else {
                            send_event!(
                                UIEvent::SendLiveScraperMessage,
                                json!({"i18nKey": "riven.cooldown", "values": json!({"seconds": (riven_interval - time_elapsed.as_secs())})})
                            );
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
                                    LogLevel::Critical => {
                                        // Stop the live scraper
                                        is_running.store(false, Ordering::SeqCst);
                                        play_sound!("windows_xp_error.mp3", 1.0);
                                        emit_error!(e);
                                    }
                                    LogLevel::Error => {
                                        if !just_started.load(Ordering::SeqCst) {
                                            play_sound!("windows_xp_error.mp3", 1.0);
                                            emit_error!(e);
                                        }
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

<<<<<<< HEAD
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
            let wfm = states::wfm_client().unwrap();
            // Get My Orders from Warframe Market.
            let mut my_orders = wfm.orders().get_my_orders().await.unwrap();
            my_orders.apply_trade_info().unwrap();
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
            let riven_interval = settings.live_scraper.stock_riven.update_interval as u64;
            let mut last_riven_update = Instant::now() - Duration::from_secs(riven_interval * 2);

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
                } else if settings.live_scraper.stock_mode == StockMode::Riven {
                    scraper.send_gui_update(
                        "riven.cooldown",
                        Some(json!({"seconds": riven_interval - last_riven_update.elapsed().as_secs()})),
                    );
                }

                if settings.live_scraper.stock_mode == StockMode::Item
                    || settings.live_scraper.stock_mode == StockMode::All
                {
                    match scraper.item().check_stock(&mut my_orders).await {
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
=======
    pub fn riven(&self) -> Arc<RivenModule> {
        self.riven_module
            .get()
            .expect("Riven module not initialized")
            .clone()
>>>>>>> better-backend
    }
}
