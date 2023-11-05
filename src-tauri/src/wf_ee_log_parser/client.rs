use crate::error::AppError;
use crate::settings::SettingsState;
use crate::{helper, logger};
use serde_json::json;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom}; // Add Seek here
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use super::events::on_new_conversation::OnNewConversationEvent;
use super::events::on_new_trading::OnTradingEvent;

#[derive(Clone, Debug)]
pub struct EELogParser {
    is_running: Arc<AtomicBool>,
    wf_ee_path: PathBuf,
    last_file_size: Arc<Mutex<u64>>,
    last_line_index: Arc<Mutex<usize>>,
    handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    cold_start: Arc<AtomicBool>,
    // Events
    event_conversation: Arc<Mutex<OnNewConversationEvent>>,
    event_trading: Arc<Mutex<OnTradingEvent>>,
}

impl EELogParser {
    pub fn new(settings: Arc<Mutex<SettingsState>>) -> Self {
        let wf_ee_path = helper::get_app_local_path().join("Warframe").join("EE.log");
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            wf_ee_path: wf_ee_path.clone(),
            last_file_size: Arc::new(Mutex::new(0)),
            last_line_index: Arc::new(Mutex::new(0)),
            handle: Arc::new(Mutex::new(None)),
            cold_start: Arc::new(AtomicBool::new(true)),
            event_conversation: Arc::new(Mutex::new(OnNewConversationEvent::new(
                Arc::clone(&settings),
                wf_ee_path.clone(),
            ))),
            event_trading: Arc::new(Mutex::new(OnTradingEvent::new(
                Arc::clone(&settings),
                wf_ee_path.clone(),
            )))
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

        // Events to check
        let event_conversation = self.event_conversation.lock()?.clone();
        let event_trading = self.event_trading.lock()?.clone();

        match new_lines_result {
            Ok(new_lines) => {
                for line in new_lines {
                    event_conversation.check(line.0, &line.1)?;
                    event_trading.check(line.0, &line.1)?;
                }
            }
            Err(err) => {
                helper::send_message_to_window("EELogParser", Some(json!({ "error": "err" })));
                Err(AppError::new("EELogParser", eyre::eyre!(err.to_string())))?
            }
        }
        Ok(())
    }

    fn read_new_lines(&self, is_starting: bool) -> io::Result<Vec<(usize, String)>> {
        let mut new_lines: Vec<(usize, String)> = Vec::new();
        let mut file = File::open(&self.wf_ee_path)?;

        let metadata = file.metadata()?;
        let current_file_size = metadata.len();

        if is_starting {
            *self.last_file_size.lock().unwrap() = current_file_size;
            return Ok(new_lines);
        }

        let mut last_file_size = self.last_file_size.lock().unwrap();
        let mut last_line_index = self.last_line_index.lock().unwrap();
        if *last_file_size > current_file_size || current_file_size< *last_file_size {
            *last_file_size = 0;
            *last_line_index = 0;
        }

        // Now we can call seek on the file because we have Seek in our scope
        file.seek(SeekFrom::Start(*last_file_size))?;

        let reader = BufReader::new(file);

        for (_, line) in reader.lines().enumerate() {
            *last_line_index += 1;
            if let Ok(line) = line {
                new_lines.push((last_line_index.clone(), line)); // Adding line index
            }
        }

        *last_file_size = current_file_size;
        Ok(new_lines)
    }
}
