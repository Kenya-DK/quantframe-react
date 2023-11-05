use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{
    error::{self, AppError},
    logger,
    settings::SettingsState,
};
use eyre::eyre;
use serde::{Deserialize, Serialize};

enum Events {
    Offering,
    Receive,
    Finished,
}
impl Events {
    fn as_str_list(&self) -> Vec<String> {
        match self {
            Events::Offering => vec![r"You are offering:".to_string()],
            Events::Receive => {
                vec![r"and will receive from (?<name>.+) the following:".to_string()]
            }
            Events::Finished => vec![
                r"The trade was (?<name>.+)".to_string(),
                r"The trade (?<name>.+)".to_string(),
            ],
        }
    }
}

#[derive(Clone, Debug)]
pub struct OnTradingEvent {
    wf_ee_path: PathBuf,
    settings: Arc<Mutex<SettingsState>>,
    // Current trade
    trade: Arc<Mutex<TradingStruct>>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradingStruct {
    // Current trade
    datetime: String,
    trade_mode: String,
    offerings: Vec<String>,
    from: String,
    receives: Vec<String>,
    raw_lines: Vec<String>,
}
impl TradingStruct {
    pub fn new() -> Self {
        Self {
            datetime: String::new(),
            trade_mode: "stopped".to_string(),
            offerings: Vec::new(),
            from: String::new(),
            receives: Vec::new(),
            raw_lines: Vec::new(),
        }
    }
    pub fn reset(&mut self) {
        self.datetime = String::new();
        self.trade_mode = "stopped".to_string();
        self.offerings = Vec::new();
        self.from = String::new();
        self.receives = Vec::new();
        self.raw_lines = Vec::new();
    }
}

impl OnTradingEvent {
    pub fn new(settings: Arc<Mutex<SettingsState>>, wf_ee_path: PathBuf) -> Self {
        Self {
            settings,
            wf_ee_path,
            trade: Arc::new(Mutex::new(TradingStruct::new())),
        }
    }
    pub fn check(&self, index: usize, input: &str) -> Result<(), AppError> {
        let file_path = "tradings.json";
        let settings = self.settings.lock()?.clone().whisper_scraper;

        let arced_mutex = Arc::clone(&self.trade);
        let mut trade = arced_mutex.lock()?;

        if !settings.enable {
            return Ok(());
        }

        match crate::wf_ee_log_parser::events::helper::match_pattern(
            input,
            Events::Receive.as_str_list(),
        ) {
            Ok((found, captures)) => {
                if found {
                    println!("Found receive: {:?}", index);
                    trade.raw_lines = crate::wf_ee_log_parser::events::helper::get_range_of_lines(
                        self.wf_ee_path.to_str().unwrap(),
                        index as i32,
                        25,
                        25,
                    )?;

                    trade.datetime = chrono::Local::now().naive_local().to_string();
                    trade.trade_mode = "receive".to_string();
                    trade.from = captures.get(0).unwrap().clone().unwrap();
                } else if trade.trade_mode == "receive" && input != "" && !input.contains(":") {
                    trade.receives.push(input.to_string().clone());
                }
            }
            Err(_) => {}
        }

        match crate::wf_ee_log_parser::events::helper::match_pattern(
            input,
            Events::Offering.as_str_list(),
        ) {
            Ok((found, _)) => {
                if found {
                    trade.trade_mode = "offering".to_string();
                } else if trade.trade_mode == "offering" && input != "" {
                    trade.offerings.push(input.to_string().clone());
                }
            }
            Err(_) => {}
        }

        match crate::wf_ee_log_parser::events::helper::match_pattern(
            input,
            Events::Finished.as_str_list(),
        ) {
            Ok((found, captures)) => {
                if found {
                    trade.raw_lines.push(input.to_string().clone());
                    let state = captures.get(0).unwrap().clone().unwrap();
                    if state.contains("cancelled") {
                        logger::debug_con("OnTradingEvent", "Trade cancelled");
                    } else if state.contains("successful") {
                        logger::debug_con("OnTradingEvent", "Trade successful");
                        logger::debug_con(
                            "OnTradingEvent",
                            &format!("Offering: {:?}", trade.offerings),
                        );
                        logger::debug_con(
                            "OnTradingEvent",
                            &format!("Receiving: {:?}", trade.receives),
                        );
                        logger::debug_con("OnTradingEvent", &format!("From: {:?}", trade.from));
                        logger::debug_con(
                            "OnTradingEvent",
                            &format!("Date: {:?}", chrono::Local::now().naive_local().to_string()),
                        );
                    }
                    match self.read_json_file(file_path) {
                        Ok(data) => {
                            // Modify the data
                            let mut modified_data = data;
                            modified_data.push(trade.clone());

                            // Write the modified data back to the JSON file
                            if let Err(err) = self.write_json_file(file_path, &modified_data) {
                                error::create_log_file("read_json_file.log".to_string(), &err);
                            }
                        }
                        Err(err) => {
                            error::create_log_file("read_json_file.log".to_string(), &err);
                        }
                    }
                    trade.reset();
                }
            }
            Err(_) => {}
        }

        Ok(())
    }
    fn read_json_file(&self, file_path: &str) -> Result<Vec<TradingStruct>, AppError> {
        let path = logger::get_log_forlder().join(file_path);
        match std::fs::File::open(path) {
            Ok(file) => {
                let reader = std::io::BufReader::new(file);
                let data: Vec<TradingStruct> = serde_json::from_reader(reader)
                    .map_err(|e| AppError::new("read_json_file", eyre!(e.to_string())))?;
                Ok(data)
            }
            Err(_) => {
                // Create a new file if it doesn't exist
                let new_data: Vec<TradingStruct> = vec![];
                self.write_json_file(file_path, &new_data)?;
                Ok(new_data)
            }
        }
    }

    fn write_json_file(&self, file_path: &str, data: &Vec<TradingStruct>) -> Result<(), AppError> {
        let path = logger::get_log_forlder().join(file_path);
        let file = std::fs::File::create(path)
            .map_err(|e| AppError::new("read_json_file", eyre!(e.to_string())))?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, data)
            .map_err(|e| AppError::new("read_json_file", eyre!(e.to_string())))?;
        Ok(())
    }
}
