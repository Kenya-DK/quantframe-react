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
}
