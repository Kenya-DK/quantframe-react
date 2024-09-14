use std::{
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom},
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, RwLock,
    },
    time::Duration,
};

use crate::{
    app::client::AppState,
    auth::AuthState,
    cache::client::CacheClient,
    helper,
    notification::client::NotifyClient,
    qf_client::client::QFClient,
    settings::SettingsState,
    utils::modules::{
        error::{self, AppError},
        logger,
    },
    wfm_client::client::WFMClient,
};

use super::modules::{on_conversation::OnConversationEvent, on_trading::OnTradeEvent};

#[derive(Clone, Debug)]
pub struct LogParser {
    log_file: PathBuf,
    is_running: Arc<AtomicBool>,
    pub component: String,
    last_file_size: Arc<Mutex<u64>>,
    cold_start: Arc<AtomicBool>,
    on_trade_event: Arc<RwLock<Option<OnTradeEvent>>>,
    on_conversation_event: Arc<RwLock<Option<OnConversationEvent>>>,
    pub settings: Arc<Mutex<SettingsState>>,
    pub wfm: Arc<Mutex<WFMClient>>,
    pub auth: Arc<Mutex<AuthState>>,
    pub cache: Arc<Mutex<CacheClient>>,
    pub notify: Arc<Mutex<NotifyClient>>,
    pub app: Arc<Mutex<AppState>>,
    pub qf: Arc<Mutex<QFClient>>,
}

impl LogParser {
    pub fn new(
        app: Arc<Mutex<AppState>>,
        settings: Arc<Mutex<SettingsState>>,
        wfm: Arc<Mutex<WFMClient>>,
        auth: Arc<Mutex<AuthState>>,
        cache: Arc<Mutex<CacheClient>>,
        notify: Arc<Mutex<NotifyClient>>,
        qf: Arc<Mutex<QFClient>>,
    ) -> Self {
        LogParser {
            app,
            settings,
            wfm,
            auth,
            cache,
            notify,
            qf,
            log_file: helper::get_local_data_path()
                .join("Warframe")
                .join("EE.log"),
            is_running: Arc::new(AtomicBool::new(false)),
            component: "LogParser".to_string(),
            last_file_size: Arc::new(Mutex::new(0)),
            cold_start: Arc::new(AtomicBool::new(true)),
            on_trade_event: Arc::new(RwLock::new(None)),
            on_conversation_event: Arc::new(RwLock::new(None)),
        }
    }
    // pub fn stop_loop(&self) {
    //     self.is_running.store(false, Ordering::SeqCst);
    // }

    // pub fn is_running(&self) -> bool {
    //     self.is_running.load(Ordering::SeqCst)
    // }
    pub fn start_loop(self) -> Result<(), AppError> {
        // Return if it's already running
        if self.is_running.load(Ordering::SeqCst) {
            logger::info_con(&self.component, "Log parser is already running");
            return Ok(());
        }

        self.is_running.store(true, Ordering::SeqCst);
        let is_running = Arc::clone(&self.is_running);

        let forced_stop = Arc::clone(&self.is_running);
        let scraper = self.clone();
        logger::info_con(&scraper.component, "Starting the log parser");
        tauri::async_runtime::spawn(async move {
            while is_running.load(Ordering::SeqCst) && forced_stop.load(Ordering::SeqCst) {
                match scraper.check_for_new_logs(self.cold_start.load(Ordering::SeqCst)) {
                    Ok(_) => (),
                    Err(e) => {
                        if e.cause().to_string().contains("Log file does not exist") {
                            logger::warning_con(
                                &scraper.component,
                                &format!("{} wil try again in 10 seconds", e.cause()),
                            );
                            // Wait 10 seconds before trying again
                            tokio::time::sleep(Duration::from_secs(10)).await;
                            logger::info_con(
                                &scraper.component,
                                "Trying to start the log parser again",
                            );
                        } else {
                            error::create_log_file("log_parser.logs".to_string(), &e);
                        }
                    }
                }
                scraper.cold_start.store(false, Ordering::SeqCst);
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
            logger::info_con(&scraper.component, "Log parser stopped");
        });
        Ok(())
    }
    pub fn check_for_new_logs(&self, is_starting: bool) -> Result<(), AppError> {
        if !self.log_file.exists() {
            return Err(AppError::new(
                &self.component,
                eyre::eyre!("Log file does not exist: {:?}", self.log_file),
            ));
        }

        let mut file = File::open(&self.log_file).map_err(|e| {
            AppError::new(
                &self.component,
                eyre::eyre!("Error opening log file: {}", e),
            )
        })?;

        let metadata = file.metadata().map_err(|e| {
            AppError::new(
                &self.component,
                eyre::eyre!("Error getting file metadata: {}", e),
            )
        })?;
        let current_file_size = metadata.len();

        if is_starting {
            *self.last_file_size.lock().unwrap() = current_file_size;
            return Ok(());
        }

        let mut last_file_size = self.last_file_size.lock().unwrap();

        if *last_file_size > current_file_size || current_file_size < *last_file_size {
            *last_file_size = 0;
        }

        // Now we can call seek on the file because we have Seek in our scope
        file.seek(SeekFrom::Start(*last_file_size)).map_err(|e| {
            AppError::new(
                &self.component,
                eyre::eyre!("Error seeking log file: {}", e),
            )
        })?;

        let reader = BufReader::new(file);

        for (_, line) in reader.lines().enumerate() {
            if let Ok(line) = line {
                if self.trade_event().process_line(&line, *last_file_size)? {
                    continue;
                }
                if self
                    .conversation_event()
                    .process_line(&line, *last_file_size)?
                {
                    continue;
                }
            }
        }

        *last_file_size = current_file_size;
        Ok(())
    }

    pub fn get_logs_between(&self, start: u64, _end: u64) -> Result<Vec<String>, AppError> {
        let mut file = File::open(&self.log_file).map_err(|e| {
            AppError::new(
                &self.component,
                eyre::eyre!("Error opening log file: {}", e),
            )
        })?;

        let mut logs = Vec::new();

        file.seek(SeekFrom::Start(start)).map_err(|e| {
            AppError::new(
                &self.component,
                eyre::eyre!("Error seeking log file: {}", e),
            )
        })?;

        let reader = BufReader::new(file);

        for (_, line) in reader.lines().enumerate() {
            if let Ok(line) = line {
                logs.push(line);
            }
        }

        Ok(logs)
    }

    pub fn trade_event(&self) -> OnTradeEvent {
        // Lazily initialize ItemModule if not already initialized
        if self.on_trade_event.read().unwrap().is_none() {
            *self.on_trade_event.write().unwrap() = Some(OnTradeEvent::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the on_trade_event is initialized
        self.on_trade_event
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone()
    }
    pub fn update_trade_event(&self, module: OnTradeEvent) {
        // Update the stored ItemModule
        *self.on_trade_event.write().unwrap() = Some(module);
    }
    pub fn conversation_event(&self) -> OnConversationEvent {
        // Lazily initialize ItemModule if not already initialized
        if self.on_conversation_event.read().unwrap().is_none() {
            *self.on_conversation_event.write().unwrap() =
                Some(OnConversationEvent::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the on_conversation_event is initialized
        self.on_conversation_event
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone()
    }
}
