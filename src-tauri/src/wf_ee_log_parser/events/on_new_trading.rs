use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{
    cache::client::CacheClient,
    database::client::DBClient,
    error::{self, AppError},
    handler::MonitorHandler,
    helper, logger,
    settings::SettingsState,
    structs::TradeClassification,
    structs::WarframeLanguage,
};
use eyre::eyre;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug)]
struct TradeLogMessages {
    detect_line: &'static str,
    detect_trade_confirmation_line: &'static str,
    detect_trade_failed_line: &'static str,
    will_receive_line_first_part: &'static str,
    will_receive_line_second_part: &'static str,
    platinum_name: &'static str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerTradeStruct {
    crated_at: String,
    user_name: String,
    trade_type: TradeClassification,
    total_platinum: i32,
    offerings: Vec<TradeItemStruct>,
    receiving: Vec<TradeItemStruct>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeItemStruct {
    name: String,
    wfm_id: Option<String>,
    wfm_url_name: Option<String>,
    display_name: String,
    quantity: i32,
    rank: i32,
}

#[derive(Debug)]
pub struct OnTradingEvent {
    wf_ee_path: PathBuf,
    settings: Arc<Mutex<SettingsState>>,
    chche: Arc<Mutex<CacheClient>>,
    helper: Arc<Mutex<MonitorHandler>>,
    // Current trade
    trade_log_messages_by_language: HashMap<WarframeLanguage, TradeLogMessages>,
    current_trade_logs: Vec<String>,
    getting_trade_message_multiline: bool,
    waiting_for_trade_message_confirmation: bool,
    current_trade: Arc<Mutex<PlayerTradeStruct>>,
}

impl OnTradingEvent {
    pub fn new(
        settings: Arc<Mutex<SettingsState>>,
        helper: Arc<Mutex<MonitorHandler>>,
        chche: Arc<Mutex<CacheClient>>,
        wf_ee_path: PathBuf,
    ) -> Self {
        Self {
            settings,
            helper,
            chche,
            wf_ee_path,
            trade_log_messages_by_language: HashMap::from([(
                WarframeLanguage::English,
                TradeLogMessages {
                    detect_line:
                        "description=Are you sure you want to accept this trade? You are offering",
                    detect_trade_confirmation_line:
                        "description=The trade was successful!, leftItem=/Menu/Confirm_Item_Ok",
                    detect_trade_failed_line:
                        "description=The trade failed., leftItem=/Menu/Confirm_Item_Ok",
                    will_receive_line_first_part: "and will receive from ",
                    will_receive_line_second_part: " the following:",
                    platinum_name: "Platinum",
                },
            )]),
            current_trade_logs: Vec::new(),
            getting_trade_message_multiline: false,
            waiting_for_trade_message_confirmation: false,
            current_trade: Arc::new(Mutex::new(PlayerTradeStruct {
                crated_at: chrono::Local::now().to_string(),
                total_platinum: -1,
                user_name: "".to_string(),
                trade_type: TradeClassification::Unknown,
                offerings: Vec::new(),
                receiving: Vec::new(),
            })),
        }
    }
    pub fn check(&mut self, _index: usize, input: &str) -> Result<bool, AppError> {
        // let file_path = "tradings.json";
        // let settings = self.settings.lock()?.clone().whisper_scraper;

        while self.getting_trade_message_multiline {
            if input.contains("[Info]") || input.contains("[Error]") || input.contains("[Warning]")
            {
                self.getting_trade_message_multiline = false;
                self.trade_logs_finished()?;
                self.waiting_for_trade_message_confirmation = true;
            } else {
                self.received_trade_log_message(input);
                return Ok(true);
            }
        }

        // Start of a Trade
        if input.contains("[Info]: Dialog.lua: Dialog::CreateOkCancel(description=")
            && self.is_beginninig_of_tradelog(input)?
        {
            self.start_trade_log(input);
            if input
                .contains(", leftItem=/Menu/Confirm_Item_Ok, rightItem=/Menu/Confirm_Item_Cancel)")
            {
                self.waiting_for_trade_message_confirmation = true;
            } else {
                self.getting_trade_message_multiline = true;
            }
            return Ok(true);
        }
        // Waiting for trade confirmation / trade failed
        else if self.waiting_for_trade_message_confirmation
            && input.contains("[Info]: Dialog.lua: Dialog::CreateOk(description=")
        {
            if self.is_trade_confirmation(input)? {
                self.trade_accepted()?;
            } else if self.is_trade_failed(input)? {
                self.trade_failed();
            }
            return Ok(true);
        }
        Ok(false)
    }

    fn start_trade_log(&mut self, msg: &str) {
        self.reset_trade();
        self.received_trade_log_message(msg);
    }
    fn received_trade_log_message(&mut self, msg: &str) {
        self.current_trade_logs.push(msg.to_string());
    }

    fn trade_logs_finished(&mut self) -> Result<(), AppError> {
        let trade_struct_mutex = Arc::clone(&self.current_trade);
        let mut trade_struct = trade_struct_mutex.lock()?;

        let lang = helper::get_warframe_language();

        let trade_log_messages = self.trade_log_messages_by_language.get(&lang).unwrap();

        let mut logs = self.current_trade_logs.clone();

        let first_line = logs.get(0).unwrap().clone();
        let str_array: Vec<&str> = first_line.split('\n').collect();
        logs.remove(0);
        for (index, item) in str_array.into_iter().enumerate() {
            logs.insert(index, item.to_string());
        }
        let mut flag = true;
        // Loop through the trade logs
        for (_index, log) in logs.iter().enumerate() {
            if log == "\n" || log == "" || log.contains(trade_log_messages.detect_line) {
                continue;
            }
            // Find the user name
            if log.contains(trade_log_messages.will_receive_line_first_part)
                && log.contains(trade_log_messages.will_receive_line_second_part)
            {
                trade_struct.user_name = log
                    .replace(trade_log_messages.will_receive_line_first_part, "")
                    .replace(trade_log_messages.will_receive_line_second_part, "")
                    .replace("\u{e000}", "")
                    .trim()
                    .to_string();
                flag = false;
            } else {
                let mut str2 = log.clone();
                if log.contains(", leftItem=/") {
                    str2.truncate(log.find(", leftItem=/").unwrap());
                }

                let str3 = str2.replace("\r", "").replace("\n", "");
                let mut item_name;
                let mut num = 1;

                if str3.contains(" x ") {
                    let parts: Vec<&str> = str3.split(" x ").collect();
                    item_name = parts[0].to_string();
                    num = parts[1].parse().unwrap_or(1);
                } else {
                    item_name = str3;
                }

                item_name = item_name.trim().to_string();

                if item_name == trade_log_messages.platinum_name {
                    item_name = "plat".to_string();
                }
                // Check if item is empty
                if item_name == "" {
                    continue;
                }
                if flag {
                    if let Some(traded_object) = trade_struct
                        .offerings
                        .iter_mut()
                        .find(|p| p.name == item_name)
                    {
                        traded_object.quantity += 1;
                    } else {
                        trade_struct.offerings.push(TradeItemStruct {
                            wfm_id: None,
                            wfm_url_name: None,
                            name: item_name.clone(),
                            quantity: num,
                            display_name: item_name.clone(),
                            rank: 0,
                        });
                    }
                } else if let Some(traded_object) = trade_struct
                    .receiving
                    .iter_mut()
                    .find(|p| p.name == item_name)
                {
                    traded_object.quantity += 1;
                } else {
                    trade_struct.receiving.push(TradeItemStruct {
                        wfm_id: None,
                        wfm_url_name: None,
                        name: item_name.clone(),
                        quantity: num,
                        display_name: item_name.clone(),
                        rank: 0,
                    });
                }
            }
        }

        // Clean up the trade struct
        for item in trade_struct.offerings.iter_mut() {
            if !self.convert_itemname_to_id(item)? {
                item.display_name = item.name.clone();
            }
        }

        // Get the total platinum amount
        let mut all_trade_items = trade_struct.offerings.clone();
        all_trade_items.append(&mut trade_struct.receiving.clone());
        trade_struct.total_platinum = all_trade_items
            .iter()
            .filter(|p| p.name == "plat")
            .map(|p| p.quantity)
            .sum::<i32>();

        for item in trade_struct.receiving.iter_mut() {
            if !self.convert_itemname_to_id(item)? {
                item.display_name = item.name.clone();
            }
        }
        let num3 = trade_struct
            .receiving
            .iter()
            .any(|p| p.name == "plat")
            .then(|| 1)
            .unwrap_or(0);

        let receiving_plat = trade_struct.offerings.iter().any(|p| p.name == "plat");

        if num3 == 0 || trade_struct.offerings.len() != 1 {
            if !receiving_plat || trade_struct.receiving.len() != 1 {
                trade_struct.trade_type = TradeClassification::Trade;
            } else {
                trade_struct.trade_type = TradeClassification::Purchase;
            }
        } else {
            trade_struct.trade_type = TradeClassification::Sale;
        }
        Ok(())
    }

    fn convert_itemname_to_id(&self, item: &mut TradeItemStruct) -> Result<bool, AppError> {
        item.rank = -1;
        let item_cache = self
            .chche
            .lock()?
            .items()
            .get_types()
            .expect("No item cache found");
        // Find the item

        if item.name.contains("(") && item.name.ends_with(")") {
            let item_details: String = item.name[item.name.rfind("(").unwrap()..].to_string();
            let name_part = item.name[..item.name.rfind("(").unwrap() - 1].to_string();
            if item_details.len() > 3 {
                // Get the rank of the item
                let item_rank = item_details.replace("(", "").replace(")", "");
                let ch_array: &[char] = &[' '];
                for s in item_rank.split(ch_array) {
                    if let Ok(result) = s.parse::<i32>() {
                        item.rank = result;
                        break;
                    }
                }
                // Check if the item is a riven mod
                if item_details.contains("(RIVEN RANK ") {
                    if item_details.contains(" Riven Mod") {
                        item.display_name = name_part + " (Veiled)";
                    } else {
                        let str3 = name_part[..name_part.rfind(" ").unwrap()].to_string();
                        let str4 = name_part[name_part.rfind(" ").unwrap() + 1..].to_string();
                        item.name = format!("/AF_Special/Riven/{}/{}", str3, str4);
                    }

                    return Ok(true);
                }
                let ch_item = item_cache.iter().find(|p| p.item_name == name_part);
                if ch_item.is_some() {
                    let ch_item = ch_item.unwrap();
                    item.wfm_id = Some(ch_item.id.clone());
                    item.wfm_url_name = Some(ch_item.url_name.clone());
                    item.display_name = ch_item.item_name.clone();
                    return Ok(true);
                } else {
                    item.display_name = name_part;
                }
                return Ok(true);
            }
        }
        if item.name.chars().count() != item.name.len() {
            let arcane_name_part = item.name[..item.name.rfind(' ').unwrap_or(0)].to_string();
            item.display_name = arcane_name_part;
            return Ok(true);
        }
        Ok(false)
    }

    fn trade_accepted(&mut self) -> Result<(), AppError> {
        let file_path = "tradings.json";
        let mh = self.helper.lock()?.clone();
        let trade = self.current_trade.lock()?.clone();

        // Send a notification to the user
        mh.show_notification(
            "Trade Accepted",
            &format!("Trade accepted from {}", trade.user_name),
            Some("assets/icons/icon.png"),
            Some("Default"),
        );

        // Send the trade to the main window
        helper::send_message_to_window("Client:Trade:Received", json!(trade.clone()))?;

        match self.read_json_file(file_path) {
            Ok(data) => {
                // Modify the data
                let mut modified_data = data;

                let mut json_data = json!(trade.clone());

                json_data["current_trade_logs"] = json!(self.current_trade_logs.clone());
                modified_data.push(json_data);

                // Write the modified data back to the JSON file
                if let Err(err) = self.write_json_file(file_path, &modified_data) {
                    error::create_log_file("read_json_file.log".to_string(), &err);
                }
            }
            Err(err) => {
                error::create_log_file("read_json_file.log".to_string(), &err);
            }
        }

        logger::info_con(
            "OnTradingEvent",
            format!("Trade accepted from {}", trade.user_name).as_str(),
        );
        self.reset_trade();
        Ok(())
    }

    fn trade_failed(&mut self) {
        logger::info_con("OnTradingEvent", "Trade failed");
        self.reset_trade();
    }

    fn reset_trade(&mut self) {
        let mut trade_struct = self.current_trade.lock().unwrap();
        trade_struct.trade_type = TradeClassification::Unknown;
        trade_struct.offerings.clear();
        trade_struct.receiving.clear();
        trade_struct.user_name = "".to_string();
        trade_struct.total_platinum = 0;
        self.current_trade_logs = Vec::new();
        self.getting_trade_message_multiline = false;
        self.waiting_for_trade_message_confirmation = false;
    }

    fn is_beginninig_of_tradelog(&self, msg: &str) -> Result<bool, AppError> {
        let lang = helper::get_warframe_language();
        // Find trade log messages
        let trade_log_messages = self.trade_log_messages_by_language.get(&lang).unwrap();

        // Check if the message is the beginning of a trade log
        if msg.contains(trade_log_messages.detect_line) {
            return Ok(true);
        }
        Ok(false)
    }

    fn is_trade_confirmation(&self, msg: &str) -> Result<bool, AppError> {
        let lang = helper::get_warframe_language();
        // Find trade log messages
        let trade_log_messages = self.trade_log_messages_by_language.get(&lang).unwrap();

        // Check if the message is the beginning of a trade log
        if msg.contains(trade_log_messages.detect_trade_confirmation_line) {
            return Ok(true);
        }
        Ok(false)
    }

    fn is_trade_failed(&self, msg: &str) -> Result<bool, AppError> {
        let lang = helper::get_warframe_language();
        // Find trade log messages
        let trade_log_messages = self.trade_log_messages_by_language.get(&lang).unwrap();

        // Check if the message is the beginning of a trade log
        if msg.contains(trade_log_messages.detect_trade_failed_line) {
            return Ok(true);
        }
        Ok(false)
    }

    fn read_json_file(&self, file_path: &str) -> Result<Vec<Value>, AppError> {
        let path = logger::get_log_forlder().join(file_path);
        match std::fs::File::open(path) {
            Ok(file) => {
                let reader = std::io::BufReader::new(file);
                let data: Vec<Value> = serde_json::from_reader(reader)
                    .map_err(|e| AppError::new("read_json_file", eyre!(e.to_string())))?;
                Ok(data)
            }
            Err(_) => {
                // Create a new file if it doesn't exist
                let new_data: Vec<Value> = vec![];
                self.write_json_file(file_path, &new_data)?;
                Ok(new_data)
            }
        }
    }

    fn write_json_file(&self, file_path: &str, data: &Vec<Value>) -> Result<(), AppError> {
        let path = logger::get_log_forlder().join(file_path);
        let file = std::fs::File::create(path)
            .map_err(|e| AppError::new("read_json_file", eyre!(e.to_string())))?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, data)
            .map_err(|e| AppError::new("read_json_file", eyre!(e.to_string())))?;
        Ok(())
    }
}
