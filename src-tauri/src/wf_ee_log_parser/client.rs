use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

use eyre::eyre;
use polars::{
    prelude::{DataFrame, NamedFrom},
    series::Series,
};
use reqwest::{header::HeaderMap, Client, Method, Url};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    auth::AuthState,
    error::AppError,
    helper,
    logger::{self, LogLevel},
    rate_limiter::RateLimiter,
    structs::{Item, RivenAttributeInfo, RivenTypeInfo},
    wfm_client::client::WFMClient,
};

use super::modules::{item::ItemModule, riven::RivenModule};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct CacheDataStruct {
    pub last_refresh: Option<String>,
    pub item: CacheDataItemStruct,
    pub riven: CacheDataRivenStruct,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct CacheDataItemStruct {
    pub items: Vec<Item>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CacheDataRivenStruct {
    pub items: Vec<RivenTypeInfo>,
    pub attributes: Vec<RivenAttributeInfo>,
}

#[derive(Clone, Debug)]
pub struct EELogParser {
    is_running: Arc<AtomicBool>,
    log_path: PathBuf,
    last_file_size: Arc<Mutex<u64>>,
    handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    cold_start: Arc<AtomicBool>,
    settings: Arc<Mutex<SettingsState>>,
}

impl EELogParser {
    pub fn new(settings: Arc<Mutex<SettingsState>>) -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            log_path: helper::get_app_local_path().join("Warframe").join("EE.log"),
            last_file_size: Arc::new(Mutex::new(0)),
            handle: Arc::new(Mutex::new(None)),
            cold_start: Arc::new(AtomicBool::new(true)),
            settings,
        }
    }

    pub fn start_loop(&mut self) {
        logger::info_con("EELogParser", "Starting Whisper Listener");
        let is_running = Arc::clone(&self.is_running);

        let scraper = self.clone();
        self.is_running.store(true, Ordering::SeqCst);

        let handle = thread::spawn(move || {
            while is_running.load(Ordering::SeqCst) {
                match scraper.check() {
                    Ok(_) => {
                        scraper.cold_start.store(false, Ordering::SeqCst);
                    }
                    Err(_) => {}
                }

                thread::sleep(Duration::from_secs(1));
            }
        });

        *self.handle.lock().unwrap() = Some(handle);
    }

    pub fn stop_loop(&self) {
        logger::info_con("EELogParser", "Stopping Whisper Listener");
        self.is_running.store(false, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        // Return the current value of is_running
        self.is_running.load(Ordering::SeqCst)
    }

    fn check(&self) -> Result<(), AppError> {
        let new_lines_result = self.read_new_lines(self.cold_start.load(Ordering::SeqCst));
        let settings = self.settings.lock()?.clone().whisper_scraper;
        match new_lines_result {
            Ok(new_lines) => {
                for line in new_lines {
                
                }
            }
            Err(err) => {
                helper::send_message_to_window(
                    "EELogParser",
                    Some(json!({ "error": "err" })),
                );
                Err(AppError::new(
                    "EELogParser",
                    eyre::eyre!(err.to_string()),
                ))?
            }
        }
        Ok(())
    }

    fn read_new_lines(&self, is_starting: bool) -> io::Result<Vec<String>> {
        let mut new_lines = Vec::new();
        let mut file = File::open(&self.log_path)?;

        let metadata = file.metadata()?;
        let current_file_size = metadata.len();

        if is_starting {
            *self.last_file_size.lock().unwrap() = current_file_size;
            return Ok(new_lines);
        }

        let mut last_file_size = self.last_file_size.lock().unwrap();

        if *last_file_size > current_file_size {
            *last_file_size = 0;
        }

        // Now we can call seek on the file because we have Seek in our scope
        file.seek(SeekFrom::Start(*last_file_size))?;

        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                new_lines.push(line);
            }
        }

        *last_file_size = current_file_size;
        Ok(new_lines)
    }
}
