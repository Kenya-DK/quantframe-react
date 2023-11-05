use crate::error::AppError;
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
    cold_start: Arc<AtomicBool>,
    settings: Arc<Mutex<SettingsState>>,
}

impl WhisperScraper {
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
        logger::info_con("WhisperScraper", "Starting Whisper Listener");
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
        logger::info_con("WhisperScraper", "Stopping Whisper Listener");
        self.is_running.store(false, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        // Return the current value of is_running
        self.is_running.load(Ordering::SeqCst)
    }

    fn check(&self) -> Result<(), AppError> {
        let new_lines_result = self.read_new_lines(self.cold_start.load(Ordering::SeqCst));
        let settings = self.settings.lock()?.clone().whisper_scraper;

        // offering, receive, stopped
        let mut trade_mode = "stopped";

        let mut offerings: Vec<String> = Vec::new();
        let mut from: String = "".to_string();
        let mut receives: Vec<String> = Vec::new();

        match new_lines_result {
            Ok(new_lines) => {
                for line in new_lines {
                    // Check if for trading dialog

                    match WhisperScraper::match_trade_receive(&line) {
                        Ok((matched, group1)) => {
                            if matched {
                                trade_mode = "receive";
                                from = group1.clone().unwrap();
                                println!("Trade from: {:?}", from);
                            } else if trade_mode == "receive" && line != "" && !line.contains(":") {
                                receives.push(line.clone().replace(", leftItem=/Menu/Confirm_Item_Ok, rightItem=/Menu/Confirm_Item_Cancel)", ""));
                            }
                        }
                        Err(_) => {}
                    }

                    match WhisperScraper::match_trade_offering(&line) {
                        Ok((matched, _)) => {
                            if matched {
                                trade_mode = "offering";
                            } else if trade_mode == "offering" && line != "" {
                                offerings.push(line.clone());
                            }
                        }
                        Err(_) => {}
                    }

                    match WhisperScraper::match_trade_finished(&line) {
                        Ok((matched, group1)) => {
                            if matched {
                                let trade_result = group1.clone().unwrap();
                                if trade_result == "cancelled" {
                                    println!("Trade cancelled");
                                }
                                if trade_result == "successful" {
                                    println!("Trade successful");
                                    println!("Offering: {:?}", offerings);
                                    println!("Receiving: {:?}", receives);
                                    println!("From: {:?}", from);
                                }
                                trade_mode = "stopped";
                            }
                        }
                        Err(_) => {}
                    }

                    // if trade_started {
                    //     items.push(line.clone());
                    //     // Check if trade ended
                    //     match WhisperScraper::match_trade_from(&line) {
                    //         Ok((matched, group1)) => {
                    //             if matched {
                    //                 let username = group1.clone().unwrap();
                    //                 println!("Trade from: {:?}", username);
                    //                 println!("Trade from: {}", username);
                    //             }
                    //         }
                    //         Err(_) => {}
                    //     }
                    // }

                    // Check if trade started

                    match WhisperScraper::match_pattern(&line) {
                        Ok((matched, group1)) => {
                            if matched {
                                let username = group1.clone().unwrap();
                                helper::send_message_to_window(
                                    "WhisperScraper:ReceivedMessage",
                                    Some(json!({ "name": username })),
                                );
                                if settings.webhook != "" {
                                    helper::send_message_to_discord(
                                        settings.webhook.clone(),
                                        format!("You have whisper(s) from {}", username.as_str()),
                                        settings.ping_on_notif,
                                    );
                                }
                            }
                        }
                        Err(err) => {
                            helper::send_message_to_window(
                                "WhisperScraper:Error",
                                Some(json!({ "error": "err" })),
                            );
                            logger::error_con("WhisperScraper", format!("{:?}", err).as_str());
                            Err(AppError::new(
                                "WhisperScraper",
                                eyre::eyre!(err.to_string()),
                            ))?
                        }
                    }
                }
            }
            Err(err) => {
                helper::send_message_to_window(
                    "WhisperScraper:Error",
                    Some(json!({ "error": "err" })),
                );
                Err(AppError::new(
                    "WhisperScraper",
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

    fn match_trade_offering(input: &str) -> Result<(bool, Option<String>), regex::Error> {
        // Dialog.lua: Dialog::CreateOkCancel(
        let pattern = r"You are offering:";
        let re = Regex::new(pattern)?;

        if let Some(captures) = re.captures(input) {
            return Ok((true, None));
        }

        Ok((false, None))
    }

    fn match_trade_receive(input: &str) -> Result<(bool, Option<String>), regex::Error> {
        //
        let pattern = r"and will receive from (?<name>.+) the following:";
        let re = Regex::new(pattern)?;

        if let Some(captures) = re.captures(input) {
            let group1 = captures.get(1).map(|m| m.as_str().to_string());
            let result: Option<String> =
                group1.map(|s| s.chars().filter(|c| c.is_ascii()).collect());
            return Ok((true, result));
        }

        Ok((false, None))
    }

    fn match_trade_finished(input: &str) -> Result<(bool, Option<String>), regex::Error> {
        // Dialog.lua: Dialog::CreateOkCancel(
        let trade_cancelled = r"The trade was cancelled";
        let trade_successful = r"The trade was successful";

        if let Some(_) = Regex::new(trade_cancelled)?.captures(input) {
            return Ok((true, Some("cancelled".to_string())));
        }
        if let Some(_) = Regex::new(trade_successful)?.captures(input) {
            return Ok((true, Some("successful".to_string())));
        }

        Ok((false, None))
    }

    fn match_pattern(input: &str) -> Result<(bool, Option<String>), regex::Error> {
        //
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
