use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    cache::client::CacheClient,
    enums::TradeClassification,
    error::{self, AppError},
    handler::MonitorHandler,
    helper, logger,
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
impl PlayerTradeStruct {
    pub fn as_string(&self) -> String {
        format!(
            "Trade: Created at: {}, Offerings: {}, Receiving: {}, User: {}",
            self.crated_at,
            self.offerings.len(),
            self.receiving.len(),
            self.user_name,
        )
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeItemStruct {
    unique_name: String,
    wfm_id: Option<String>,
    wfm_url_name: Option<String>,
    display_name: String,
    raw_name: String,
    quantity: i32,
    rank: i32,
}

#[derive(Debug)]
pub struct OnTradingEvent {
    cache: Arc<Mutex<CacheClient>>,
    helper: Arc<Mutex<MonitorHandler>>,
    // Current trade
    trade_log_messages_by_language: HashMap<WarframeLanguage, TradeLogMessages>,
    current_trade_logs: Vec<String>,
    getting_trade_message_multiline: bool,
    waiting_for_trade_message_confirmation: bool,
    current_trade: Arc<Mutex<PlayerTradeStruct>>,
}

impl OnTradingEvent {
    pub fn new(helper: Arc<Mutex<MonitorHandler>>, cache: Arc<Mutex<CacheClient>>) -> Self {
        Self {
            helper,
            cache,
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
        while self.getting_trade_message_multiline {
            if input.contains("[Info]") || input.contains("[Error]") || input.contains("[Warning]")
            {
                self.getting_trade_message_multiline = false;
                logger::info_con("OnTradingEvent", "Trade log finished");
                self.trade_logs_finished()?;
                self.waiting_for_trade_message_confirmation = true;
            } else {
                self.received_trade_log_message(input);
                return Ok(true);
            }
        }

        // Start of a Trade
        if input.contains("[Info]: Dialog.lua: Dialog::CreateOkCancel(description=")
            && self.is_trade_log_beginning(input)?
        {
            logger::info_con("OnTradingEvent", "Trade Started");
            self.start_trade_log(input);
            if input
                .contains(", leftItem=/Menu/Confirm_Item_Ok, rightItem=/Menu/Confirm_Item_Cancel)")
            {
                logger::info_con("OnTradingEvent", "Waiting for trade confirmation");
                self.waiting_for_trade_message_confirmation = true;
            } else {
                logger::info_con("OnTradingEvent", "Getting trade message multiline");
                self.getting_trade_message_multiline = true;
            }
            return Ok(true);
        }
        if self.waiting_for_trade_message_confirmation
            && input.contains("[Info]: Dialog.lua: Dialog::CreateOk(description=")
        {
            if self.is_trade_confirmation(input)? {
                logger::info_con("OnTradingEvent", "Trade Was Accepted");
                self.trade_accepted()?;
            } else if self.is_trade_failed(input)? {
                logger::info_con("OnTradingEvent", "Trade Failed");
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
        logger::info_con(
            "OnTradingEvent",
            format!("Processing trade logs: {:?}", self.current_trade_logs).as_str(),
        );
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

                let str3 = str2.clone().replace("\r", "").replace("\n", "");
                let mut item_name;
                let mut quantity = 1;

                if str3.contains(" x ") {
                    let parts: Vec<&str> = str3.split(" x ").collect();
                    item_name = parts[0].to_string();
                    quantity = parts[1].parse().unwrap_or(1);
                } else {
                    item_name = str3.clone();
                }

                item_name = item_name.trim().to_string();

                if item_name == trade_log_messages.platinum_name {
                    item_name = "plat".to_string();
                    trade_struct.total_platinum += quantity;
                }
                // Check if item is empty
                if item_name == "" {
                    continue;
                }
                if flag {
                    if let Some(traded_object) = trade_struct
                        .offerings
                        .iter_mut()
                        .find(|p| p.unique_name == item_name)
                    {
                        traded_object.quantity += 1;
                    } else {
                        trade_struct.offerings.push(TradeItemStruct {
                            wfm_id: None,
                            wfm_url_name: None,
                            raw_name: str3.clone(),
                            unique_name: item_name.clone(),
                            quantity,
                            display_name: item_name.clone(),
                            rank: 0,
                        });
                    }
                } else if let Some(traded_object) = trade_struct
                    .receiving
                    .iter_mut()
                    .find(|p| p.unique_name == item_name)
                {
                    traded_object.quantity += 1;
                } else {
                    trade_struct.receiving.push(TradeItemStruct {
                        wfm_id: None,
                        wfm_url_name: None,
                        raw_name: str3.clone(),
                        unique_name: item_name.clone(),
                        quantity,
                        display_name: item_name.clone(),
                        rank: 0,
                    });
                }
            }
        }

        // Clean up the trade receiving
        for item in trade_struct.receiving.iter_mut() {
            if !self.map_item_name_to_id(item)? {
                logger::warning(
                    "OnTradingEvent",
                    format!("Item not found: {} in receiving", item.unique_name).as_str(),
                    true,
                    Some("trade.log"),
                );
            }
        }
        // Clean up the trade offerings
        for item in trade_struct.offerings.iter_mut() {
            if !self.map_item_name_to_id(item)? {
                logger::warning(
                    "OnTradingEvent",
                    format!("Item not found: {} in offerings", item.unique_name).as_str(),
                    true,
                    Some("trade.log"),
                );
            }
        }

        let offerings_plat = trade_struct
            .offerings
            .iter()
            .any(|p| p.unique_name == "/QF_Special/Platinum")
            .then(|| 1)
            .unwrap_or(0);

        let receiving_plat = trade_struct
            .receiving
            .iter()
            .any(|p| p.unique_name == "/QF_Special/Platinum")
            .then(|| 1)
            .unwrap_or(0);

        if receiving_plat == 0 || trade_struct.receiving.len() != 1 {
            if offerings_plat == 0 || trade_struct.offerings.len() != 1 {
                trade_struct.trade_type = TradeClassification::Trade;
            } else {
                trade_struct.trade_type = TradeClassification::Purchase;
            }
        } else {
            trade_struct.trade_type = TradeClassification::Sale;
        }
        logger::info_con(
            "OnTradingEvent",
            format!("Trade log processed: {}", trade_struct.as_string()).as_str(),
        );
        Ok(())
    }

    fn map_item_name_to_id(&self, item: &mut TradeItemStruct) -> Result<bool, AppError> {
        let cache = self.cache.lock()?.clone();
        item.rank = -1;
        if item.unique_name == "plat" {
            item.unique_name = "/QF_Special/Platinum".to_string();
            return Ok(true);
        }
        if item.unique_name.starts_with("Imprint of") {
            item.unique_name = format!(
                "/QF_Special/Imprint/{}",
                item.unique_name.replace("Imprint of ", "")
            );
            return Ok(true);
        }
        if item.unique_name.starts_with("Legendary Core") {
            item.unique_name = "/QF_Special/Legendary Fusion Core".to_string();
            return Ok(true);
        }
        if item.unique_name.starts_with("Ancient Core") {
            item.unique_name = "/QF_Special/Legendary Ancient Core".to_string();
            return Ok(true);
        }

        // Check if the item is a misc item
        let misc = cache.misc().get_by_name(&item.unique_name, true);
        if misc.is_some() {
            let misc = misc.unwrap();
            item.unique_name = misc.unique_name.clone();
            return Ok(true);
        }


        if item.unique_name.contains("(") && item.unique_name.ends_with(")") {
            let item_details: String =
                item.unique_name[item.unique_name.rfind("(").unwrap()..].to_string();
            let mut name_part =
                item.unique_name[..item.unique_name.rfind("(").unwrap() - 1].to_string();
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
                        item.unique_name = format!("/QF_Special/Riven/{}/{}", str3, str4);
                    }
                    // TODO: Add riven mod to the cache
                    return Ok(false);
                }
                name_part = name_part.replace(" defiled", "");
                let mods = cache.mods().get_by_name(&name_part, true);
                if mods.is_some() {
                    let mods = mods.unwrap();
                    item.unique_name = mods.unique_name.clone();
                    return Ok(true);
                }
                return Ok(false);
            }
            let fish = cache.fish().get_by_name(&name_part, true);
            let fish_size: String = item_details.replace("(", "").replace(")", "");
            if fish_size.len() == 1 {
                let char_array: Vec<char> = fish_size.chars().collect();
                if let Some(first_char) = char_array.get(0) {
                    let rank = *first_char as i32;
                    item.rank = rank;
                }
            }
            if fish.is_some() {
                let fish = fish.unwrap();
                item.unique_name = fish.unique_name.clone();
                return Ok(true);
            }
            return Ok(false);
        }

        if item.unique_name.chars().count() != item.unique_name.len() {
            let arcane_name_part =
                item.unique_name[..item.unique_name.rfind(' ').unwrap_or(0)].to_string();
            // Check if Item is a Arcane
            let arcane = cache.arcane().get_by_name(&arcane_name_part, true);
            if arcane.is_some() {
                let arcane = arcane.unwrap();
                item.unique_name = arcane.unique_name.clone();
                return Ok(true);
            }
        }
        // Check if the item is a warframe part
        let mut warframe_part =
            cache
                .parts()
                .get_part_by_name("Warframe", cache.clone(), &item.unique_name, true);
        if warframe_part.is_none() {
            warframe_part = cache.parts().get_part_by_name(
                "Warframe",
                cache.clone(),
                &item.unique_name.replace(" Blueprint", ""),
                true,
            );
        }
        if warframe_part.is_some() {
            let warframe_part = warframe_part.unwrap();
            item.unique_name = warframe_part.unique_name.clone();
            return Ok(true);
        }

        // Check if the item is a weapon part
        let mut weapon_part =
            cache
                .parts()
                .get_part_by_name("Weapon", cache.clone(), &item.unique_name, true);
        if weapon_part.is_none() {
            weapon_part = cache.parts().get_part_by_name(
                "Weapon",
                cache.clone(),
                &item.unique_name.replace(" Blueprint", ""),
                true,
            );
        }
        if weapon_part.is_some() {
            let weapon_part = weapon_part.unwrap();
            item.unique_name = weapon_part.unique_name.clone();
            return Ok(true);
        }


        // Check if the item is a primary weapon
        let primary = cache.primary().get_by_name(&item.unique_name, true);
        if primary.is_some() {
            let primary = primary.unwrap();
            item.unique_name = primary.unique_name.clone();
            return Ok(true);
        }

        // Check if the item is a secondary weapon
        let secondary = cache.secondary().get_by_name(&item.unique_name, true);
        if secondary.is_some() {
            let secondary = secondary.unwrap();
            item.unique_name = secondary.unique_name.clone();
            return Ok(true);
        }

        // Check if the item is a melee weapon
        let melee = cache.melee().get_by_name(&item.unique_name, true);
        if melee.is_some() {
            let melee = melee.unwrap();
            item.unique_name = melee.unique_name.clone();
            return Ok(true);
        }

        // Check if the item is a arc-gun
        let arc_gun = cache.arch_gun().get_by_name(&item.unique_name, true);
        if arc_gun.is_some() {
            let arc_gun = arc_gun.unwrap();
            item.unique_name = arc_gun.unique_name.clone();
            return Ok(true);
        }

        // Check if the item is a arc-melee
        let arc_melee = cache.arch_melee().get_by_name(&item.unique_name, true);
        if arc_melee.is_some() {
            let arc_melee = arc_melee.unwrap();
            item.unique_name = arc_melee.unique_name.clone();
            return Ok(true);
        }

        // Check if the item is a Relic
        if item.unique_name.contains("Relic") {
                        
        }

        // Check if the item is a skin
        let skin = cache.skin().get_by_name(&item.unique_name, true);
        if skin.is_some() {
            let skin = skin.unwrap();
            item.unique_name = skin.unique_name.clone();
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
            &format!("Trade Accepted: {}", trade.user_name),
            &format!(
                "{}\nReceiving: {} item(s) \nOffering: {} item(s)",
                trade.trade_type.display(),
                trade.offerings.clone().len(),
                trade.receiving.clone().len()
            ),
            Some("assets/icons/icon.png"),
            Some("Default"),
        );

        // Send the trade to the main window
        helper::send_message_to_window("Client:Trade:Received", Some(json!(trade.clone())));

        match self.read_json_file(file_path) {
            Ok(data) => {
                // Modify the data
                let mut modified_data = data.clone();

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

        self.reset_trade();
        Ok(())
    }

    fn trade_failed(&mut self) {
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

    fn is_trade_log_beginning(&self, msg: &str) -> Result<bool, AppError> {
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
        let path = logger::get_log_folder().join(file_path);
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
        let path = logger::get_log_folder().join(file_path);
        let file = std::fs::File::create(path)
            .map_err(|e| AppError::new("read_json_file", eyre!(e.to_string())))?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, data)
            .map_err(|e| AppError::new("read_json_file", eyre!(e.to_string())))?;
        Ok(())
    }
}
