use crate::{
    emit_error,
    enums::*,
    live_scraper::modules::*,
    notify_gui, play_sound, send_event, track_event,
    types::UIEvent,
    utils::{modules::states, OrderListExt},
};
use qf_api::enums::app_events::ApplicationEvent as EventType;
use serde_json::json;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, OnceLock,
    },
    time::{Duration, Instant},
};
use utils::{get_location, info, warning, LogLevel, LoggerOptions};

#[derive(Debug)]
pub struct LiveScraperState {
    pub is_running: Arc<AtomicBool>,
    pub just_started: Arc<AtomicBool>,
    started_at: Mutex<Option<Instant>>,
    item_module: OnceLock<Arc<ItemModule>>,
    riven_module: OnceLock<Arc<RivenModule>>,
}

impl LiveScraperState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            is_running: Arc::new(AtomicBool::new(false)),
            just_started: Arc::new(AtomicBool::new(true)),
            started_at: Mutex::new(None),
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
        *self.started_at.lock().unwrap() = Some(Instant::now());
        info(
            "LiveScraper:Start",
            format!("Live Scraper started. Elapsed: {}", self.elapsed()),
            &LoggerOptions::default(),
        );
        if settings.live_scraper.general.stock_mode == StockMode::All
            || settings.live_scraper.general.stock_mode == StockMode::Item
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
                let riven_interval = settings.live_scraper.rivens.general.update_interval as u64;
                let mut last_riven_update = Instant::now()
                    .checked_sub(Duration::from_secs(riven_interval * 2))
                    .unwrap_or(Instant::now());

                while is_running.load(Ordering::SeqCst) {
                    let app = states::app_state().expect("App state not initialized");
                    // let wfm_client = app.wfm_client;
                    // println!("{}", wfm_client.order().cache_orders());
                    if matches!(
                        app.settings.live_scraper.general.stock_mode,
                        StockMode::Riven | StockMode::All
                    ) {
                        // Check Time
                        let time_elapsed = last_riven_update.elapsed();
                        if time_elapsed > Duration::from_secs(riven_interval) {
                            last_riven_update = Instant::now();
                            match this.riven().check().await {
                                Ok(_) => {}
                                Err(e) => {
                                    let err_type = e
                                        .properties
                                        .get_property_value("type", "Unknown".to_string());
                                    e.clone()
                                        .with_location(get_location!())
                                        .log("live_scraper_riven.log");
                                    match e.log_level {
                                        LogLevel::Critical | LogLevel::Error => {
                                            // Stop the live scraper
                                            info(
                                                "LiveScraper:Stop",
                                                format!(
                                                    "Live Scraper stopped due to error. Elapsed: {}",
                                                    this.elapsed()
                                                ),
                                                &LoggerOptions::default(),
                                            );
                                            track_event!(
                                                EventType::LiveScraperError,
                                                [
                                                    ("success", "false".to_string()),
                                                    ("error_type", err_type),
                                                    ("type", "riven".to_string()),
                                                    ("log_level", format!("{:?}", e.log_level)),
                                                    ("elapsed_seconds", this.elapsed().to_string()),
                                                ]
                                            );
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
                        app.settings.live_scraper.general.stock_mode,
                        StockMode::Item | StockMode::All
                    ) {
                        match this.item().check().await {
                            Ok(_) => {}
                            Err(mut e) => {
                                let err_type =
                                    e.properties.get_property_value("type", String::new());

                                e.log_level = match err_type.as_str() {
                                    "ParsingError"
                                    | "BadRequest"
                                    | "Unknown"
                                    | "InternalServerError"
                                    | "InvalidType" => LogLevel::Critical,
                                    _ => LogLevel::Warning,
                                };

                                e.clone()
                                    .with_location(get_location!())
                                    .log("live_scraper_item.log");

                                if matches!(e.log_level, LogLevel::Critical | LogLevel::Error) {
                                    notify_gui!(
                                        "app_error",
                                        e.log_level.to_color(),
                                        "error",
                                        json!(e),
                                        json!({ "autoClose": false })
                                    );
                                    info(
                                        "LiveScraper:Stop",
                                        format!(
                                            "Live Scraper stopped due to error. Elapsed: {}",
                                            this.elapsed()
                                        ),
                                        &LoggerOptions::default(),
                                    );
                                    track_event!(
                                        EventType::LiveScraperError,
                                        [
                                            ("success", "false".to_string()),
                                            ("error_type", err_type),
                                            ("type", "item".to_string()),
                                            ("log_level", format!("{:?}", e.log_level)),
                                            ("elapsed_seconds", this.elapsed().to_string()),
                                        ]
                                    );
                                    is_running.store(false, Ordering::SeqCst);
                                    play_sound!("windows_xp_error.mp3", 1.0);
                                    emit_error!(e);
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
        info(
            "LiveScraper:Stop",
            format!("Live Scraper stopped. Elapsed: {}", self.elapsed()),
            &LoggerOptions::default(),
        );
        track_event!(
            EventType::LiveScraperStop,
            [
                ("success", "true".to_string()),
                ("elapsed_seconds", self.elapsed().to_string()),
            ]
        );
        *self.started_at.lock().unwrap() = None;
        self.is_running.store(false, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    pub fn just_started(&self) -> bool {
        self.just_started.load(Ordering::SeqCst)
    }

    pub fn elapsed(&self) -> u64 {
        let started_at = self.started_at.lock().map(|g| *g).unwrap_or(None);
        let total_secs = match started_at {
            Some(start) => start.elapsed().as_secs(),
            None => 0,
        };
        total_secs
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
