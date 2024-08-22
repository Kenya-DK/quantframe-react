use std::collections::HashMap;

use entity::{
    enums::stock_type::StockType, sub_type::SubType, transaction::transaction::TransactionType,
};
use eyre::eyre;

use migration::value;
use serde_json::{json, Value};
use service::{StockItemMutation, StockRivenMutation, StockRivenQuery, TransactionMutation};

use crate::{
    cache::{client::CacheClient, types::cache_item_component::CacheItemComponent},
    helper,
    log_parser::{
        client::LogParser,
        enums::trade_classification::TradeClassification,
        types::{
            create_stock_entity::CreateStockEntity,
            trade::{PlayerTrade, TradeItem},
            trade_detection::TradeDetection,
        },
    },
    utils::{
        enums::{
            log_level::LogLevel,
            ui_events::{UIEvent, UIOperationEvent},
        },
        modules::{
            error::{self, AppError},
            logger,
        },
    },
    wfm_client::enums::order_type::OrderType,
};

#[derive(Clone, Debug)]
pub struct OnTradeEvent {
    pub client: LogParser,
    component: String,
    pub trade_detections: HashMap<String, TradeDetection>,
    logs: Vec<String>,
    getting_trade_message_multiline: bool,
    waiting_confirmation: bool,
    current_trade: PlayerTrade,
    start_pos: u64,
    end_pos: u64,
}

impl OnTradeEvent {
    pub fn new(client: LogParser) -> Self {
        let mut trade_detections: HashMap<String, TradeDetection> = HashMap::new();
        trade_detections.insert(
            "en".to_string(),
            TradeDetection::new(
                "description=Are you sure you want to accept this trade? You are offering"
                    .to_string(),
                "description=The trade was successful!, leftItem=/Menu/Confirm_Item_Ok".to_string(),
                "description=The trade failed., leftItem=/Menu/Confirm_Item_Ok".to_string(),
                "description=The trade was cancelled".to_string(),
                "and will receive from ".to_string(),
                " the following:".to_string(),
                "Platinum".to_string(),
            ),
        );
        OnTradeEvent {
            client,
            component: "OnTradeEvent".to_string(),
            trade_detections,
            logs: vec![],
            getting_trade_message_multiline: false,
            waiting_confirmation: false,
            current_trade: PlayerTrade::default(),
            start_pos: 0,
            end_pos: 0,
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }
    fn update_state(&self) {
        self.client.update_trade_event(self.clone());
    }
    fn debug(&self, message: &str) {
        logger::debug_file(
            &self.get_component("Debug"),
            message,
            Some("on_trade_event.log"),
        );
    }
    pub fn process_line(&mut self, line: &str, pos: u64) -> Result<bool, AppError> {
        while self.getting_trade_message_multiline {
            // Check if the line is the end of the trade
            if line.contains("[Info]") || line.contains("[Error]") || line.contains("[Warning]") {
                self.getting_trade_message_multiline = false;
                self.trade_finished();
                self.waiting_confirmation = true;
                self.debug(format!("Trade finished: {}", line).as_str());
                self.update_state();
            } else {
                self.debug(format!("Getting trade message: {}", line).as_str());
                self.add_trade_message(line);
                return Ok(true);
            }
        }
        // Start of a Trade
        if line.contains("[Info]: Dialog.lua: Dialog::CreateOkCancel(description=")
            && self.is_beginning_of_trade(line)
        {
            self.trade_started(line);
            self.debug(format!("Trade started: {}", line).as_str());
            self.start_pos = pos;
            if line
                .contains(", leftItem=/Menu/Confirm_Item_Ok, rightItem=/Menu/Confirm_Item_Cancel)")
            {
                self.waiting_confirmation = true;
            } else {
                self.getting_trade_message_multiline = true;
            }
            self.update_state();
            return Ok(true);
        }
        // Waiting for trade confirmation / trade failed
        else if self.waiting_confirmation
            && line.contains("[Info]: Dialog.lua: Dialog::CreateOk(description=")
        {
            self.end_pos = pos;
            if self.was_trade_successful(line) {
                self.debug(format!("Trade successful: {}", line).as_str());
                match self.trade_accepted() {
                    Ok(_) => {}
                    Err(e) => {
                        error::create_log_file("trade_accepted.log".to_string(), &e);
                    }
                }
            } else if self.was_trade_failed(line) {
                self.debug(format!("Trade failed: {}", line).as_str());
                self.trade_failed();
            } else if self.was_trade_cancelled(line) {
                self.debug(format!("Trade cancelled: {}", line).as_str());
                self.trade_cancelled();
            }
            self.reset();
            return Ok(true);
        }
        Ok(false)
    }
    pub fn reset(&mut self) {
        self.current_trade = PlayerTrade::default();
        self.logs = Vec::new();
        self.getting_trade_message_multiline = false;
        self.waiting_confirmation = false;
        self.update_state();
    }
    pub fn trade_started(&mut self, line: &str) {
        self.reset();
        self.add_trade_message(line);
    }
    pub fn trade_finished(&mut self) {
        let detection = self.trade_detections.get("en").unwrap();
        let cache = self.client.cache.lock().unwrap();
        self.current_trade.file_logs = self
            .client
            .get_logs_between(self.start_pos, self.end_pos)
            .expect("Failed to get logs");
        self.current_trade.logs = self.logs.clone();
        let lines: Vec<String> = self.logs.clone();

        // // Validate the logs from the EE.log
        // let mut is_trade = false;
        // for log in self.current_trade.file_logs.iter() {
        //     if log.contains("[Info]: Dialog.lua: Dialog::CreateOkCancel(description=")
        //         && self.is_beginning_of_trade(&log)
        //     {
        //         is_trade = true;
        //     }
        //     if is_trade {
        //         lines.push(log.clone());
        //     }
        //     if log
        //         .contains(", leftItem=/Menu/Confirm_Item_Ok, rightItem=/Menu/Confirm_Item_Cancel)")
        //     {
        //         break;
        //     }
        // }

        let mut is_offering = true;
        for (_, line) in lines.iter().enumerate() {
            if line == "\n"
                || line == ""
                || line.contains(&detection.start)
                || line.contains(&detection.confirmation_line)
                || line.contains(&detection.failed_line)
            {
                continue;
            }

            // Find the player name
            if line.contains(&detection.receive_line_first_part)
                && line.contains(&detection.receive_line_second_part)
            {
                let player_name = line
                    .replace(&detection.receive_line_first_part, "")
                    .replace(&detection.receive_line_second_part, "")
                    .replace("\u{e000}", "")
                    .trim()
                    .to_string();
                self.current_trade.player_name = helper::remove_special_characters(&player_name);
                is_offering = false;
            } else {
                let mut line_clone = line.clone();
                if line.contains(", leftItem=/") {
                    line_clone.truncate(line.find(", leftItem=/").unwrap());
                }

                // Get Item Name and Quantity
                let raw_name = line_clone.replace("\r", "").replace("\n", "");
                let mut item_name;
                let mut quantity = 1;

                if raw_name.contains(" x ") {
                    let parts: Vec<&str> = raw_name.split(" x ").collect();
                    item_name = parts[0].to_string();
                    quantity = parts[1].parse().unwrap_or(1);
                } else {
                    item_name = raw_name;
                }
                item_name = item_name.trim().to_string();

                // Check if the item is platinum
                if item_name == detection.platinum_name {
                    item_name = "plat".to_string();
                }

                // Check if the item is empty
                if item_name == "" {
                    continue;
                }

                let trade_item = TradeItem::new(
                    item_name.clone(),
                    quantity,
                    item_name.clone(),
                    None,
                    None,
                    None,
                    "Unknown".to_string(),
                    None,
                );

                let mut items = if is_offering {
                    self.current_trade.offered_items.iter_mut()
                } else {
                    self.current_trade.received_items.iter_mut()
                };

                if let Some(trade) = items.find(|p| p.name == item_name) {
                    trade.quantity += 1;
                } else if is_offering {
                    self.current_trade.offered_items.push(trade_item);
                } else {
                    self.current_trade.received_items.push(trade_item);
                }
            }
        }

        // Parse the items
        for item in self.current_trade.offered_items.iter_mut() {
            match parse_item(&cache, item) {
                Ok(success) => {
                    if !success {
                        logger::warning_file(
                            "TradeFinished",
                            &format!("Failed to parse item {} in offered", item.name),
                            Some("trade_progress.log"),
                        );
                    } else {
                        // Get trade item from cache
                        let trade_item = cache
                            .tradable_items()
                            .get_by(
                                item.unique_name.as_str(),
                                "--item_by unique_name --item_lang en",
                            )
                            .expect("Failed to find item");
                        if trade_item.is_some() && item.name != "Platinum" {
                            let trade_item = trade_item.unwrap();
                            item.wfm_id = Some(trade_item.wfm_id.clone());
                            item.wfm_url = Some(trade_item.wfm_url_name.clone());
                            if trade_item.tags.contains(&"arcane_enhancement".to_string()) {
                                item.sub_type = Some(SubType::rank(
                                    trade_item.sub_type.unwrap().max_rank.unwrap_or(0),
                                ));
                            }
                        }
                    }
                }
                Err(e) => {
                    error::create_log_file("trade_finished.log".to_string(), &e);
                }
            }
        }
        for item in self.current_trade.received_items.iter_mut() {
            match parse_item(&cache, item) {
                Ok(success) => {
                    if !success {
                        logger::warning_file(
                            "TradeFinished",
                            &format!("Failed to parse item {} in received items", item.name),
                            Some("trade_progress.log"),
                        );
                    } else {
                        // Get trade item from cache
                        let trade_item = cache
                            .tradable_items()
                            .get_by(
                                item.unique_name.as_str(),
                                "--item_by unique_name --item_lang en",
                            )
                            .expect("Failed to find item");
                        if trade_item.is_some() && item.name != "Platinum" {
                            let trade_item = trade_item.unwrap();
                            item.wfm_id = Some(trade_item.wfm_id.clone());
                            item.wfm_url = Some(trade_item.wfm_url_name.clone());
                            if trade_item.tags.contains(&"arcane_enhancement".to_string()) {
                                item.sub_type = Some(SubType::rank(
                                    trade_item.sub_type.unwrap().max_rank.unwrap_or(0),
                                ));
                            }
                        }
                    }
                }
                Err(e) => {
                    error::create_log_file("trade_finished.log".to_string(), &e);
                }
            }
        }

        self.current_trade.trade_time = chrono::Local::now().to_string();
        let offer_plat = self.current_trade.get_offered_plat();
        let receive_plat = self.current_trade.get_received_plat();
        if offer_plat > 0 {
            self.current_trade.platinum = offer_plat;
        }
        if receive_plat > 0 {
            self.current_trade.platinum = receive_plat;
        }

        if offer_plat > 1 && self.current_trade.offered_items.len() == 1 {
            self.current_trade.trade_type = TradeClassification::Purchase;
        } else if receive_plat > 1 && self.current_trade.received_items.len() == 1 {
            self.current_trade.trade_type = TradeClassification::Sale;
        } else {
            self.current_trade.trade_type = TradeClassification::Trade;
        }
        self.update_state();
    }
    pub fn trade_cancelled(&self) {
        logger::info_con(&self.get_component("TradeCancelled"), "Trade cancelled");
    }
    pub fn trade_failed(&self) {
        logger::info_con(&self.get_component("TradeFailed"), "Trade failed");
    }
    pub fn trade_accepted(&self) -> Result<(), AppError> {
        let file_path = "tradings.json";
        let gui_id = "on_trade_event";
        let settings = self.client.settings.lock().unwrap().clone();
        let notify_user = settings.notifications.on_new_trade;
        let auto_trade = settings.live_scraper.stock_item.auto_trade;
        let notify = self.client.notify.lock().unwrap().clone();
        let trade = self.current_trade.clone();
        let cache = self.client.cache.lock()?.clone();
        let wfm = self.client.wfm.lock()?.clone();
        let app = self.client.app.lock()?.clone();
        let qf = self.client.qf.lock()?.clone();

        // If the trade is not a sale or purchase, return
        if trade.trade_type == TradeClassification::Trade {
            logger::info_con(&self.get_component("TradeAccepted"), &trade.display());
            return Ok(());
        }

        // Get Trade Items
        let items = match trade.trade_type {
            TradeClassification::Sale => trade.offered_items.clone(),
            TradeClassification::Purchase => trade.received_items.clone(),
            _ => vec![],
        };

        // Set Notification's Data
        let mut notify_type = "success";
        let mut notify_payload = json!({
            "i18n_key_title": "",
            "i18n_key_message": "",
        });
        let mut notify_value = json!({
            "player_name": trade.player_name,
            "trade_type": trade.trade_type,
            "platinum": trade.platinum,
            "order": "❔",
            "auction": "❔",
            "stock": "❔",
            "order": "❔",
        });

        // Append the trade to the file
        match self.append_to_file(file_path, trade.clone()) {
            Ok(_) => {}
            Err(err) => {
                error::create_log_file("append_to_file.log".to_string(), &err);
            }
        }

        // Validate the items
        let created_stock = match self.get_stock_item(&cache, items, trade.platinum) {
            Ok(item) => item,
            Err(err) => {
                notify_payload["i18n_key_title"] =
                    format!("{}.warning.created_stock.title", gui_id).into();
                notify_payload["i18n_key_message"] =
                    format!("{}.warning.created_stock.message", gui_id).into();
                notify
                    .gui()
                    .send_event(UIEvent::OnNotificationWarning, Some(notify_payload));
                helper::add_metric("EE_NewTrade", "not_found");
                return Err(err);
            }
        };
        helper::add_metric("EE_NewTrade", "found");

        // Set Item Name
        notify_value["item_name"] = json!(created_stock.get_name()?);
        notify_value["quantity"] = json!(created_stock.quantity);
        let notify_entity = match created_stock.entity_type {
            StockType::Item => "item",
            StockType::Riven => "riven",
            _ => "",
        };

        notify_payload["i18n_key_title"] =
            format!("{}.{}.{}.title", gui_id, notify_type, notify_entity).into();
        notify_payload["i18n_key_message"] =
            format!("{}.{}.{}.message", gui_id, notify_type, notify_entity).into();

        let content = notify_user
            .content
            .replace("<PLAYER_NAME>", trade.player_name.as_str())
            .replace("<OF_COUNT>", &trade.offered_items.len().to_string())
            .replace("<RE_COUNT>", &trade.received_items.len().to_string());

        logger::info_con(&self.get_component("TradeAccepted"), &trade.display());

        let client = self.clone();
        tokio::task::spawn(async move {
            if auto_trade {
                // Handle Riven Mods Sale
                if created_stock.entity_type == StockType::Riven
                    && trade.trade_type == TradeClassification::Sale
                {
                    // Find Stock
                    let stock = match StockRivenQuery::get_by_riven_name(
                        &app.conn,
                        &created_stock.wfm_url,
                        &created_stock.mod_name,
                        created_stock.sub_type.clone().unwrap(),
                    )
                    .await
                    {
                        Ok(stock_riven) => stock_riven,
                        Err(e) => {
                            notify.gui().send_event(
                                UIEvent::OnNotificationWarning,
                                Some(notify_payload.clone()),
                            );
                            error::create_log_file(
                                "trade_accepted.log".to_string(),
                                &AppError::new_db(&client.get_component("AutoTrade"), e),
                            );
                            None
                        }
                    };
                    if stock.is_none() {
                        notify_value["stock"] = json!("⚠️");
                        notify_payload["values"] = notify_value;
                        notify
                            .gui()
                            .send_event(UIEvent::OnNotificationWarning, Some(json!(created_stock)));
                        return;
                    }
                    let stock = stock.unwrap();

                    match helper::progress_stock_riven(
                        &mut stock.to_create(trade.platinum),
                        "--weapon_by url_name --weapon_lang en --attribute_by url_name",
                        &trade.player_name,
                        OrderType::Sell,
                        "auto",
                        app,
                        cache,
                        notify.clone(),
                        wfm,
                        qf,
                    )
                    .await
                    {
                        Ok((_, rep)) => {
                            if rep.contains(&"StockRivenDeleted".to_string()) {
                                notify_value["stock"] = json!("✅");
                            } else {
                                notify_type = "warning";
                                notify_value["stock"] = json!("⚠️");
                            }

                            if rep.contains(&"WFMRivenDeleted".to_string()) {
                                notify_value["auction"] = json!("✅");
                            } else {
                                notify_type = "warning";
                                notify_value["auction"] = json!("⚠️");
                            }
                        }
                        Err(e) => {
                            notify_value["stock"] = json!("❌");
                            error::create_log_file("trade_accepted.log".to_string(), &e);
                        }
                    }
                }
                // Handle Riven Mods Purchase
                else if created_stock.entity_type == StockType::Riven
                    && trade.trade_type == TradeClassification::Purchase
                {
                    logger::info_con(&client.get_component("AutoTrade"), "Riven Mod Purchase");
                }
                // Handle Item Sale & Purchase
                else if created_stock.entity_type == StockType::Item
                    && (trade.trade_type == TradeClassification::Sale
                        || trade.trade_type == TradeClassification::Purchase)
                {
                    // Create Stock Item
                    let mut stock_item = created_stock.to_stock_item();

                    let order_type = if trade.trade_type == TradeClassification::Sale {
                        OrderType::Sell
                    } else {
                        OrderType::Buy
                    };
                    match helper::progress_stock_item(
                        &mut stock_item,
                        "--item_by url_name --item_lang en",
                        &trade.player_name,
                        order_type,
                        vec![
                            "StockContinueOnError".to_string(),
                            "WFMContinueOnError".to_string(),
                        ],
                        "auto",
                        app,
                        cache,
                        notify.clone(),
                        wfm,
                        qf,
                    )
                    .await
                    {
                        Ok((_, rep)) => {
                            if !rep.contains(&"StockItemNotFound".to_string()) {
                                notify_value["stock"] = json!("✅");
                            } else {
                                notify_value["stock"] = json!("⚠️");
                            }

                            if rep.contains(&"WFMOrderDeleted".to_string())
                                || rep.contains(&"WFMOrderUpdated".to_string())
                            {
                                notify_value["order"] = json!("✅");
                            } else {
                                notify_value["order"] = json!("⚠️");
                            }

                            if rep.contains(&"TransactionCreated".to_string()) {
                                notify_value["transaction"] = json!("✅");
                            } else {
                                notify_value["order"] = json!("⚠️");
                            }
                        }
                        Err(_) => {
                            notify_type = "warning";
                            notify_value["stock"] = json!("⚠️");
                            notify_value["transaction"] = json!("❌");
                        }
                    }
                } else {
                    logger::warning_con("TradeAccepted", "Unknown entity type");
                    return;
                }
            }

            notify_payload["values"] = notify_value;
            if notify_user.system_notify {
                notify
                    .system()
                    .send_notification(&notify_user.title, &content, None, None);
            }

            if notify_user.discord_notify
                && notify_user.webhook.clone().unwrap_or("".to_string()) != ""
            {
                notify.discord().send_notification(
                    &notify_user.webhook.clone().unwrap_or("".to_string()),
                    &notify_user.title,
                    &content,
                    notify_user.user_ids.clone(),
                );
            }

            if notify_type == "success" {
                notify
                    .gui()
                    .send_event(UIEvent::OnNotificationSuccess, Some(notify_payload));
            } else if notify_type == "warning" {
                notify
                    .gui()
                    .send_event(UIEvent::OnNotificationWarning, Some(notify_payload));
            } else if notify_type == "error" {
                notify
                    .gui()
                    .send_event(UIEvent::OnNotificationError, Some(notify_payload));
            }
        });
        return Ok(());
    }
    pub fn add_trade_message(&mut self, line: &str) {
        self.logs.push(line.to_string());
        self.update_state();
    }
    pub fn is_beginning_of_trade(&self, line: &str) -> bool {
        let detection = self.trade_detections.get("en").unwrap();
        // Check if the message is the beginning of a trade log
        if line.contains(&detection.start) {
            return true;
        }
        false
    }
    pub fn was_trade_successful(&self, line: &str) -> bool {
        let detection = self.trade_detections.get("en").unwrap();
        // Check if the message is the beginning of a trade log
        if line.contains(&detection.confirmation_line) {
            return true;
        }
        false
    }
    pub fn was_trade_failed(&self, line: &str) -> bool {
        let detection = self.trade_detections.get("en").unwrap();
        // Check if the message is the beginning of a trade log
        if line.contains(&detection.failed_line) {
            return true;
        }
        false
    }
    pub fn was_trade_cancelled(&self, line: &str) -> bool {
        let detection = self.trade_detections.get("en").unwrap();
        // Check if the message is the beginning of a trade log
        if line.contains(&detection.cancelled_line) {
            return true;
        }
        false
    }

    fn get_stock_item(
        &self,
        cache: &CacheClient,
        items: Vec<TradeItem>,
        plat: i64,
    ) -> Result<CreateStockEntity, AppError> {
        let all_items_parts = cache.parts().get_parts("All");
        let mut stock_item = CreateStockEntity::new("", plat);

        let mut error_json = json!({
            "items": items,
            "plat": plat,
        });

        if items.len() > 1 {
            let mut source: Vec<(Option<&CacheItemComponent>, i64)> = vec![];
            for item in items {
                let com = all_items_parts
                    .iter()
                    .find(|x| x.unique_name == item.unique_name);
                source.push((com.clone(), item.quantity));
            }
            error_json["source"] = json!(source);
            // Get first item in the list
            let first_item = source.first();

            // Check first item is not none
            if first_item.is_none()
                || first_item.unwrap().0.is_none()
                || first_item.unwrap().0.unwrap().part_of.is_none()
            {
                return Err(AppError::new_with_level(
                    &self.get_component("GetWfmUrl"),
                    eyre!(format!(
                        "Failed to get item from trade [J]{}[J]",
                        error_json.to_string()
                    )),
                    LogLevel::Warning,
                ));
            }
            let main_part = first_item.unwrap().0.unwrap().part_of.as_ref().unwrap();

            if first_item.is_some()
                && source.iter().all(|x| {
                    let com = x.0;

                    if com.is_none() || com.unwrap().part_of.is_none() {
                        return false;
                    }
                    let com = com.unwrap();
                    let part_of = com.part_of.as_ref().unwrap();
                    return part_of.unique_name == main_part.unique_name;
                })
            {
                let mut num = source
                    .iter()
                    .map(|(_, count)| count)
                    .min()
                    .cloned()
                    .unwrap_or(0);
                if main_part.name.to_lowercase().contains("dual decurion") {
                    num /= 2
                }
                stock_item.raw = format!("{} Set", main_part.name);
                stock_item.quantity = num;
                stock_item.entity_type = StockType::Item;
            } else {
                return Err(AppError::new_with_level(
                    &self.get_component("GetWfmUrl"),
                    eyre!(format!(
                        "Mismatched parts in trade [J]{}[J]",
                        error_json.to_string()
                    )),
                    LogLevel::Warning,
                ));
            }
        } else if items.len() == 1 {
            let item = items.first().unwrap();
            stock_item.raw = item.name.clone();
            stock_item.quantity = item.quantity;
            stock_item.sub_type = item.sub_type.clone();
            stock_item.unique_name = item.unique_name.clone();
            stock_item.entity_type = StockType::Item;
            if item.item_type == "Riven Mod" {
                stock_item.entity_type = StockType::Riven;
                let properties = item.properties.clone();
                if properties.is_none() {
                    return Err(AppError::new_with_level(
                        &self.get_component("GetWfmUrl"),
                        eyre!(format!(
                            "Failed to get item properties [J]{}[J]",
                            error_json.to_string()
                        )),
                        LogLevel::Warning,
                    ));
                }
                let properties = properties.unwrap();
                stock_item.mod_name = properties
                    .get("mod_name")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string();
                stock_item.raw = properties
                    .get("weapon")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string();
            }
        } else {
            return Err(AppError::new_with_level(
                &self.get_component("GetWfmUrl"),
                eyre!(format!(
                    "Failed to get item from trade [J]{}[J]",
                    error_json.to_string()
                )),
                LogLevel::Warning,
            ));
        }

        if stock_item.raw == "" {
            return Err(AppError::new_with_level(
                &self.get_component("GetWfmUrl"),
                eyre!(format!(
                    "Failed to get wfm_url from trade [J]{}[J]",
                    error_json.to_string()
                )),
                LogLevel::Warning,
            ));
        }

        // Validate the stock entity
        stock_item.validate_entity(
            &cache,
            "--item_by name --item_lang en --weapon_by name --weapon_lang en --ignore_attributes",
        )?;

        return Ok(stock_item);
    }

    fn append_to_file(&self, file_path: &str, trade: PlayerTrade) -> Result<(), AppError> {
        match self.read_trade_log(file_path) {
            Ok(data) => {
                // Modify the data
                let mut modified_data = data;

                let mut json_data = json!(trade.clone());
                modified_data.push(json_data);

                // Write the modified data back to the JSON file
                if let Err(err) = self.write_trade_log(file_path, &modified_data) {
                    error::create_log_file("read_json_file.log".to_string(), &err);
                }
                return Ok(());
            }
            Err(err) => {
                error::create_log_file("read_json_file.log".to_string(), &err);
                return Err(err);
            }
        }
    }
    fn read_trade_log(&self, file_path: &str) -> Result<Vec<Value>, AppError> {
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
                self.write_trade_log(file_path, &new_data)?;
                Ok(new_data)
            }
        }
    }
    fn write_trade_log(&self, file_path: &str, data: &Vec<Value>) -> Result<(), AppError> {
        let path = logger::get_log_folder().join(file_path);
        let file = std::fs::File::create(path)
            .map_err(|e| AppError::new("read_json_file", eyre!(e.to_string())))?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, data)
            .map_err(|e| AppError::new("read_json_file", eyre!(e.to_string())))?;
        Ok(())
    }
}

pub fn parse_item(cache: &CacheClient, item: &mut TradeItem) -> Result<bool, AppError> {
    let default_by = "--item_by name --item_lang en --ignore_case";

    if item.name == "plat" {
        item.name = "Platinum".to_string();
        item.unique_name = "/QF_Special/Platinum".to_string();
        item.item_type = "Plat".to_string();
        return Ok(true);
    }

    if item.name.starts_with("Imprint of") {
        item.unique_name = format!(
            "/QF_Special/Imprint/{}",
            item.name.replace("Imprint of ", "")
        );
        item.item_type = "Imprint".to_string();
        return Ok(true);
    }

    if item.name.starts_with("Legendary Core") {
        item.unique_name = "/QF_Special/Legendary Fusion Core".to_string();
        item.item_type = "Fusion Core".to_string();
        return Ok(true);
    }

    if item.name.starts_with("Ancient Core") {
        item.unique_name = "/QF_Special/Legendary Ancient Core".to_string();
        item.item_type = "Fusion Core".to_string();
        return Ok(true);
    }

    // Check Misc items
    let misc_item = cache.misc().get_by(&item.name, default_by)?;
    if let Some(misc_item) = misc_item {
        item.unique_name = misc_item.unique_name;
        item.item_type = misc_item.category;
        return Ok(true);
    }

    // Check Mods, Rivens, Fish
    if item.name.contains("(") && item.name.contains(")") {
        let index = item.name.find("(").unwrap() as usize;
        let rank_part = &item.clone().name[index..];
        let name_part = &item.clone().name[..index - 1];

        // Check if the item is a mod/fish true if mod else it is a fish
        if rank_part.len() > 3 {
            let rank_part = rank_part.replace("(", "").replace(")", "");
            // Get The Rank of the mod
            for s in rank_part.split(' ') {
                if let Ok(result) = s.parse::<i64>() {
                    item.sub_type = Some(SubType::rank(result));
                    break;
                }
            }
            // Check if the item is a riven
            if item.name.contains("(RIVEN RANK ") {
                // Check if the item is a veiled riven
                if item.name.contains(" Riven Mod") {
                    let all_mods = cache.mods().get_items();
                    let mut filters = all_mods
                        .iter()
                        .filter(|x| x.name.contains(name_part))
                        .collect::<Vec<_>>();
                    filters.sort_by_key(|key| !key.unique_name.contains("/Raw"));

                    item.name = format!("{} (Veiled)", name_part);
                    item.item_type = "Riven Mod (Veiled)".to_string();

                    let cache_mod = filters.first();
                    if let Some(cache_mod) = cache_mod {
                        item.unique_name = cache_mod.unique_name.clone();
                    } else {
                        return Ok(false);
                    }
                } else {
                    let last_space_index = name_part.find(" ").unwrap() as usize;
                    let weapon = &name_part[..last_space_index];
                    let att = &name_part[last_space_index + 1..];
                    item.name = format!("{} {}", weapon, att);
                    item.unique_name = format!("/QF_Special/Riven/{}/{}", weapon, att);
                    item.item_type = "Riven Mod".to_string();
                    item.properties = Some(json!({ "mod_name": att, "weapon": weapon}));
                    return Ok(true);
                }
            } else {
                let cache_mod = cache.mods().get_by(&name_part, default_by)?;
                if let Some(cache_mod) = cache_mod {
                    item.name = cache_mod.name;
                    item.unique_name = cache_mod.unique_name;
                    item.item_type = cache_mod.category;
                    return Ok(true);
                }
            }
        } else {
            let cache_fish = cache.fish().get_by(&name_part, default_by)?;
            let size = rank_part.replace("(", "").replace(")", "");
            if size.len() == 1 {
                if let Some(c) = size.chars().next() {
                    item.sub_type = Some(SubType::rank(c as i64));
                }
            }
            if let Some(cache_fish) = cache_fish {
                item.unique_name = cache_fish.unique_name.clone();
                item.item_type = cache_fish.category;
                return Ok(true);
            }
        }
    }

    // Check Arcane
    if item.name.len() != item.name.chars().count() || item.name.ends_with("???") {
        let index = item.name.rfind(' ').unwrap_or(0);
        let name_part = &item.name[..index];
        let cache_arcane = cache.arcane().get_by(&name_part, default_by)?;
        if let Some(cache_arcane) = cache_arcane {
            item.name = cache_arcane.name;
            item.unique_name = cache_arcane.unique_name;
            item.item_type = cache_arcane.category;
            return Ok(true);
        } else {
            item.name = name_part.to_string();
            item.item_type = "Unknown Arcane".to_string();
        }
    }
    if item.name == "Enter Nihil's Oubliette".to_string() {
        item.unique_name = "/QF_Special/Other/Nihil's Oubliette (Key)".to_string();
        item.item_type = "Key".to_string();
        return Ok(true);
    }

    // Parts remove the blueprint
    let part = cache
        .parts()
        .get_part_by_name("All", &item.name.replace(" Blueprint", ""), true);
    if let Some(weapon_part) = part {
        item.name = format!("{} {}", weapon_part.part_of.unwrap().name, weapon_part.name);
        item.unique_name = weapon_part.unique_name;
        item.item_type = weapon_part.component_type;
        return Ok(true);
    }

    // Parts with blueprint
    let part = cache.parts().get_part_by_name("All", &item.name, true);
    if let Some(weapon_part) = part {
        item.unique_name = weapon_part.unique_name;
        item.item_type = weapon_part.component_type;
        return Ok(true);
    }

    // Melee Weapons
    let melee = cache.melee().get_by(&item.name, default_by)?;
    if let Some(melee) = melee {
        item.unique_name = melee.unique_name;
        item.item_type = melee.category;
        return Ok(true);
    }

    // Primary Weapons
    let primary = cache.primary().get_by(&item.name, default_by)?;
    if let Some(primary) = primary {
        item.unique_name = primary.unique_name;
        item.item_type = primary.category;
        return Ok(true);
    }

    // Secondary Weapons
    let secondary = cache.secondary().get_by(&item.name, default_by)?;
    if let Some(secondary) = secondary {
        item.unique_name = secondary.unique_name;
        item.item_type = secondary.category;
        return Ok(true);
    }

    // Archwing
    let archwing = cache.archwing().get_by(&item.name, default_by)?;
    if let Some(archwing) = archwing {
        item.unique_name = archwing.unique_name;
        item.item_type = archwing.category;
        return Ok(true);
    }

    // Archwing Guns
    let arch_gun = cache.arch_gun().get_by(&item.name, default_by)?;
    if let Some(arch_gun) = arch_gun {
        item.unique_name = arch_gun.unique_name.clone();
        item.item_type = arch_gun.category;
        return Ok(true);
    }

    // Archwing Melee
    let arch_melee = cache.arch_melee().get_by(&item.name, default_by)?;
    if let Some(arch_melee) = arch_melee {
        item.unique_name = arch_melee.unique_name.clone();
        item.item_type = arch_melee.category;
        return Ok(true);
    }

    // Relics
    if item.name.contains("Relic") {
        let mut str = item.name.replace(" Relic", "");
        if str.split(' ').count() == 2 {
            str += " [INTACT]";
        }
        let compare_name = str.replace("[", "").replace("]", "");
        let relic = cache.relics().get_by(&compare_name, default_by)?;
        if let Some(relic) = relic {
            item.name = relic.name.clone();
            item.unique_name = relic.unique_name.clone();
            item.item_type = relic.category;
            return Ok(true);
        }
    }
    // Skins
    let skin = cache.skin().get_by(&item.name, default_by)?;
    if let Some(skin) = skin {
        item.unique_name = skin.unique_name.clone();
        item.item_type = skin.category;
        return Ok(true);
    }

    // Misc Items
    let m_items = cache.misc().get_items();
    let misc_items = m_items
        .iter()
        .filter(|x| x.name.contains(&item.name))
        .collect::<Vec<_>>();
    if let Some(mi_item) = misc_items.first() {
        let components = mi_item.components.as_ref();
        if let Some(components) = components {
            for component in components {
                let com_name = format!("{} {}", mi_item.name, component.name);
                if com_name == item.name.replace(" Blueprint", "") {
                    item.unique_name = component.unique_name.clone();
                    item.item_type = component.component_type.clone();
                    break;
                }
            }
        }
    }
    // Misc Items
    let misc_item = cache
        .misc()
        .get_by(&item.name.replace(" Blueprint", ""), default_by)?;
    if let Some(misc_item) = misc_item {
        item.name = misc_item.name.clone();
        item.unique_name = misc_item.unique_name.clone();
        item.item_type = misc_item.category;
        return Ok(true);
    }
    // Pets
    let pet = cache
        .pet()
        .get_by(&item.name.replace(" Blueprint", ""), default_by)?;
    if let Some(pet) = pet {
        item.unique_name = pet.unique_name.clone();
        item.item_type = pet.category;
        return Ok(true);
    }
    // Resources
    let resource = cache.resource().get_by(&item.name, default_by)?;
    if let Some(resource) = resource {
        item.unique_name = resource.unique_name.clone();
        item.item_type = resource.category;
        return Ok(true);
    }
    item.unique_name = format!("/QF_Special/Other/{}", item.name);
    item.item_type = "Unknown".to_string();
    Ok(false)
}
