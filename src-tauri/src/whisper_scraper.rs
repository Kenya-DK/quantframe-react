use crate::settings::SettingsState;
use crate::{helper, logger};
use regex::Regex;
use serde_json::json;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom}; // Add Seek here
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

#[derive(Clone, serde::Serialize)]
struct Payload {
    name: String,
}

#[derive(Clone)]
pub struct WhisperScraper {
    is_running: Arc<AtomicBool>,
    log_path: PathBuf,
    last_file_size: Arc<Mutex<u64>>,
    handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    settings: Arc<Mutex<SettingsState>>,
}

impl WhisperScraper {
    pub fn new(settings: Arc<Mutex<SettingsState>>) -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            log_path: helper::get_app_local_path().join("Warframe").join("EE.log"),
            last_file_size: Arc::new(Mutex::new(0)),
            handle: Arc::new(Mutex::new(None)),
            settings,
        }
    }

    pub fn start_loop(&mut self) {
        let is_running = Arc::clone(&self.is_running);
        let settings = Arc::clone(&self.settings).lock().unwrap().clone().whisper_scraper;
        let scraper = self.clone();

        self.is_running.store(true, Ordering::SeqCst);

        let handle = thread::spawn(move || {
            let mut is_starting = true;
            while is_running.load(Ordering::SeqCst) {
                let new_lines_result = scraper.read_new_lines(is_starting);
                match new_lines_result {
                    Ok(new_lines) => {
                        for line in new_lines {
                            match WhisperScraper::match_pattern(&line) {
                                Ok((matched, group1)) => {
                                    if matched {
                                        helper::send_message_to_window(
                                            "whisper_scraper_mesage_from_player",
                                            Some(json!({"name": group1.clone().unwrap()})),
                                        );
                                        if settings.webhook != "" {
                                            helper::send_message_to_discord(
                                                settings.webhook.clone(),
                                                format!(
                                                    "You have whisper(s) from {}",
                                                    group1.unwrap().as_str()
                                                ),
                                                settings.ping_on_notif,
                                            );
                                        }
                                    }
                                }
                                Err(err) => {
                                    helper::send_message_to_window(
                                        "whisper_scraper_error",
                                        Some(json!({ "error": "err" })),
                                    );
                                    logger::error(
                                        "WhisperScraper",
                                        format!("{:?}", err).as_str(),
                                        true,
                                        None,
                                    );
                                }
                            }
                        }
                    }
                    Err(err) => eprintln!("Error: {:?}", err),
                }

                is_starting = false;

                thread::sleep(Duration::from_secs(1));
            }
        });

        *self.handle.lock().unwrap() = Some(handle);
    }

    pub fn stop_loop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        // Return the current value of is_running
        self.is_running.load(Ordering::SeqCst)
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
    fn match_pattern(input: &str) -> Result<(bool, Option<String>), regex::Error> {
        let pattern = r"Script \[Info\]: ChatRedux\.lua: ChatRedux::AddTab: Adding tab with channel name: F(?<name>.+) to index.+";
        let re = Regex::new(pattern)?;

        if let Some(captures) = re.captures(input) {
            let group1 = captures.get(1).map(|m| m.as_str().to_string());
            let result: Option<String> =
                group1.map(|s| s.chars().filter(|c| c.is_ascii()).collect());
            return Ok((true, result));
        }

        Ok((false, None))
    }
}
