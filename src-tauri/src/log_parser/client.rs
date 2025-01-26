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
    commands::log,
    helper,
    utils::modules::{
        error::{self, AppError},
        logger, states,
    },
};

use super::modules::{on_conversation::OnConversationEvent, on_trading::OnTradeEvent};

#[derive(Clone, Debug)]
pub struct LogParser {
    log_file: PathBuf,
    previous_log_file: PathBuf,
    pub component: String,
    last_file_size: Arc<Mutex<u64>>,
    on_trade_event: Arc<RwLock<Option<OnTradeEvent>>>,
    on_conversation_event: Arc<RwLock<Option<OnConversationEvent>>>,
}

impl LogParser {
    pub fn new() -> Self {
        LogParser {
            log_file: helper::get_local_data_path()
                .join("Warframe")
                .join("EE.log"),
            previous_log_file: PathBuf::new(),
            component: "LogParser".to_string(),
            last_file_size: Arc::new(Mutex::new(0)),
            on_trade_event: Arc::new(RwLock::new(None)),
            on_conversation_event: Arc::new(RwLock::new(None)),
        }
    }
    pub fn init(&self) -> Result<(), AppError> {
        let mut scraper = self.clone();
        logger::info_con(&scraper.component, "Starting the log parser");
        tauri::async_runtime::spawn(async move {
            loop {
                match scraper.check_for_new_logs() {
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
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
        });
        Ok(())
    }
    pub fn check_for_new_logs(&mut self) -> Result<(), AppError> {
        let settings = states::settings().unwrap().clone();
        let mut reset = false;

        let log_file =
            if !settings.wf_log_path.is_empty() && PathBuf::from(&settings.wf_log_path).exists() {
                PathBuf::from(&settings.wf_log_path)
            } else {
                self.log_file.clone()
            };

        //Validate log file
        if !log_file.exists() {
            return Err(AppError::new(
                &self.component,
                eyre::eyre!("Log file does not exist: {:?}", log_file),
            ));
        }

        // Read the log file and process the lines
        let mut file = File::open(&log_file).map_err(|e| {
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

        if log_file != self.previous_log_file {
            logger::info_con(&self.component, "Log file changed");
            *self.last_file_size.lock().unwrap() = current_file_size;
            self.previous_log_file = log_file.clone();
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
                println!("{}", line);
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
