use std::sync::Mutex;

use crate::{
    handlers::{handle_item, handle_riven_by_name, handle_wish_list},
    log_parser::*,
    notify_gui, send_event,
    types::*,
    utils::modules::states,
};
use serde_json::json;
use utils::*;
use wf_market::enums::OrderType;

pub static LOGGER: Mutex<Option<ZipLogger>> = Mutex::new(None);
pub static COMPONENT: Mutex<String> = Mutex::new(String::new());
pub fn add_to_zip(content: impl Into<String>) {
    let content = content.into();
    // trace(
    //     "OnTradeEvent",
    //     &content,
    //     &LoggerOptions::default().set_file("trade.log"),
    // );
    if let Some(zip) = LOGGER.lock().unwrap().as_ref() {
        zip.add_log(content).ok();
    }
}
fn get_component(component: &str) -> String {
    format!("{}:{}", COMPONENT.lock().unwrap().as_str(), component)
}
pub fn write_zip() {
    let mut log = LOGGER.lock().unwrap();
    let date = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    if let Some(zip) = log.as_ref() {
        match zip.finalize() {
            Ok(_) => println!("Zip archive finalized successfully."),
            Err(e) => {
                e.log(format!("trade_{}.log", date));
            }
        }
        *log = None;
    }
}
pub fn new_zip() {
    let date = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    *LOGGER.lock().unwrap() = ZipLogger::start(format!("trade_{}.zip", date)).ok();
}
pub fn add_error(error: &Error) {
    if let Some(zip) = LOGGER.lock().unwrap().as_ref() {
        match zip.add_error(error) {
            Ok(_) => {}
            Err(e) => {
                e.log("add_error.log");
            }
        }
    }
}
pub struct OnTradeEvent {
    detection: TradeDetection,
    logs: Vec<String>,
    getting_trade_message_multiline: bool,
    waiting_confirmation: bool,
    current_trade: PlayerTrade,
    uuid: String,
}

impl OnTradeEvent {
    pub fn new(base_component: &str) -> Self {
        let detections = DETECTIONS.get().unwrap();
        *COMPONENT.lock().unwrap() = format!("{}:OnTradeEvent", base_component);
        OnTradeEvent {
            detection: detections.get("en").unwrap().clone(),
            logs: Vec::new(),
            uuid: String::new(),
            getting_trade_message_multiline: false,
            waiting_confirmation: false,
            current_trade: PlayerTrade::default(),
        }
    }
    pub fn reset(&mut self) {
        self.logs = Vec::new();
        self.getting_trade_message_multiline = false;
        self.waiting_confirmation = false;
        self.current_trade = PlayerTrade::default();
    }
    pub fn trade_started(&mut self, line: &str, last_line: &str) {
        self.reset();
        self.add_trade_message(last_line);
        self.add_trade_message(line);
        add_to_zip("Started");
    }
    pub fn start_line_processing(&mut self) {
        let mut is_offering = true;
        let lines = self.logs.clone();
        self.current_trade.logs = lines.clone();
        let mut i = 0;

        add_to_zip(format!("Processing {} Lines", lines.len()));

        while i < lines.len() {
            let line = lines[i].to_owned().replace("\r", "").replace("\n", "");
            let next_line = if i < lines.len() - 1 {
                lines[i + 1].to_owned().replace("\r", "").replace("\n", "")
            } else {
                "N/A".to_string()
            };

            add_to_zip(format!("Line [{}]: '{}', Next: '{}'", i, line, next_line));

            let (is_irrelevant, status) =
                self.detection.is_irrelevant_trade_line(&line, &next_line);

            if !is_irrelevant {
                add_to_zip(format!(
                    "Skipping Irrelevant: '{}' (Status: {:?})",
                    line, status
                ));
                i += if status.is_combined() { 2 } else { 1 };
                continue;
            }

            let (full_line, is_offer_line) = self.detection.is_offer_line(&line, &next_line);

            if is_offer_line.is_found() {
                i += if is_offer_line.is_combined() { 2 } else { 1 };

                add_to_zip(format!(
                    "Detected Offer Line | Full: '{}' | Combined: {}",
                    full_line,
                    is_offer_line.is_combined()
                ));

                let player_name = full_line
                    .replace(&self.detection.receive_line_first_part, "")
                    .replace(&self.detection.receive_line_second_part, "")
                    .replace("\u{e000}", "")
                    .trim()
                    .to_string();
                self.current_trade.player_name = remove_special_characters(&player_name);

                add_to_zip(format!(
                    "Player Identified: '{}', Switching to Receiving Items",
                    self.current_trade.player_name
                ));

                is_offering = false;
                continue;
            } else {
                let (status, item) = TradeItem::from_string(&line, &next_line, &self.detection);

                if status.is_combined() {
                    add_to_zip(format!(
                        "Combined Line Detected (Status: {:?}) | Advancing Index",
                        status
                    ));
                    i += 1;
                }

                if !item.is_valid() {
                    add_to_zip(format!(
                        "Invalid Item | Line: '{}', Next: '{}', Status: {:?}",
                        line, next_line, status
                    ));
                    i += 1;
                    continue;
                }

                add_to_zip(format!(
                    "Valid Item Parsed: {} | Status: {:?} | Offering? {}",
                    item, status, is_offering
                ));

                let mut items = if is_offering {
                    self.current_trade.offered_items.iter_mut()
                } else {
                    self.current_trade.received_items.iter_mut()
                };

                if let Some(trade) = items.find(|p| {
                    p.unique_name == item.unique_name
                        && !item.unique_name.is_empty()
                        && p.sub_type == item.sub_type
                }) {
                    trade.quantity += 1;
                    add_to_zip(format!(
                        "Incremented Quantity for Item: {} | New Qty: {}",
                        trade.unique_name, trade.quantity
                    ));
                } else if is_offering {
                    add_to_zip(format!("Adding New Offered Item: {}", item));
                    self.current_trade.offered_items.push(item);
                } else {
                    add_to_zip(format!("Adding New Received Item: {}", item));
                    self.current_trade.received_items.push(item);
                }
            }
            i += 1;
        }

        self.current_trade.trade_time =
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        add_to_zip(format!("Trade Time Set: {}", self.current_trade.trade_time));

        self.current_trade.calculate();
        add_to_zip("Trade Calculation Complete".to_string());
    }

    pub fn trade_cancelled(&mut self) {
        add_to_zip("Cancelled");
        self.reset();
    }
    pub fn trade_failed(&mut self) {
        add_to_zip("Failed");
        self.reset();
    }
    pub fn trade_accepted(&mut self) -> Result<(), Error> {
        add_to_zip("Trade Was Successful");
        let trade = self.current_trade.clone();
        let settings = states::get_settings()?.clone();
        let order_type = match trade.trade_type {
            TradeClassification::Sale => OrderType::Sell,
            TradeClassification::Purchase => OrderType::Buy,
            _ => {
                return Ok(());
            }
        };
        let trade_type = match trade.trade_type {
            TradeClassification::Sale => TradeClassification::Purchase,
            TradeClassification::Purchase => TradeClassification::Sale,
            _ => TradeClassification::Trade,
        };
        // Log the trade to a file
        match log_json_formatted(json!(trade), "trade.json", true) {
            Ok(_) => {
                if let Some(zip) = LOGGER.lock().unwrap().as_ref() {
                    match zip.add_log_file("trade.json", "trade.json") {
                        Ok(_) => {}
                        Err(e) => add_error(&e.with_location(get_location!())),
                    }
                }
            }
            Err(_) => {}
        }
        // If the trade is not a sale or purchase, return
        if trade_type == TradeClassification::Trade {
            info(
                get_component("TradeAccepted"),
                &trade.to_string(),
                &LoggerOptions::default(),
            );
            return Ok(());
        }
        tauri::async_runtime::spawn({
            async move {
                let items = trade.get_valid_items(&trade_type);
                let mut operations = OperationSet::new();
                let item = items.first();
                if settings.live_scraper.auto_trade {
                    operations.add("AutoTrade");
                }
                add_to_zip(format!("Found {} valid items", items.len()));
                if item.is_none() {
                    warning(
                        "OnTradeEvent",
                        "No valid items found in trade",
                        &LoggerOptions::default(),
                    );
                    notify_gui!(
                        "on_trade_event",
                        "yellow",
                        "no_valid_items",
                        json!({
                            "player_name": trade.player_name
                        })
                    );
                    return;
                }
                let mut item = item.unwrap().clone();
                // Check if the trade is a set
                let (is_set, set_name) = match trade.is_set() {
                    Ok((is_set, set_name)) => (is_set, set_name),
                    Err(mut e) => {
                        e = e.with_location(get_location!());
                        e.log("");
                        add_error(&e);
                        return;
                    }
                };

                if is_set {
                    add_to_zip(format!("Trade is a set: {}", set_name));
                    item.unique_name = set_name;
                    item.sub_type = None;
                    item.quantity = 1;
                    operations.add("Found");
                } else if items.len() > 1 {
                    operations.add("MultipleItems");
                } else if !set_name.is_empty() {
                    operations.add("SetNotValid");
                } else {
                    operations.add("Found");
                }

                if operations.has("Found") && operations.has("AutoTrade") {
                    match process_trade_item(item, trade.platinum, &trade.player_name, order_type)
                        .await
                    {
                        Ok(op) => operations.merge(&op),
                        Err(mut e) => {
                            e = e.with_location(get_location!());
                            e.log("");
                            return add_error(&e);
                        }
                    }
                }
                let msg = format!("Trade Processed: {}", operations.operations.join(", "));
                add_to_zip(&msg);
                process_operations(&trade, operations);
                write_zip();
            }
        });
        self.reset();
        Ok(())
    }
    pub fn add_trade_message(&mut self, line: &str) {
        self.logs.push(line.to_string());
    }
}

impl LineHandler for OnTradeEvent {
    fn process_line(
        &mut self,
        line: &str,
        prev_line: &str,
        ignore_combined: bool,
    ) -> Result<(bool, bool), Error> {
        while self.getting_trade_message_multiline {
            if self
                .detection
                .is_end_of_trade(line, prev_line, true, ignore_combined)
                .is_found()
            {
                self.getting_trade_message_multiline = false;
                self.start_line_processing();
                self.waiting_confirmation = true;
                add_to_zip("Waiting For Confirmation/Trade Failed/Trade Cancelled");
            } else if !is_start_of_log(line) {
                self.add_trade_message(line);
                return Ok((false, false));
            } else {
                return Ok((false, false));
            }
        }

        if line.contains("UUID:") {
            self.uuid = line.split("UUID:").nth(1).unwrap_or("").trim().to_string();
        }

        // Start of a Trade
        if self
            .detection
            .is_beginning_of_trade(line, prev_line, true, ignore_combined)
            .is_found()
        {
            write_zip();
            new_zip();
            self.trade_started(line, prev_line);
            self.getting_trade_message_multiline = true;
            return Ok((false, true));
        }
        // Waiting for trade confirmation / trade failed
        else if self.waiting_confirmation
            && self
                .detection
                .is_trade_finished(line, prev_line, true)
                .is_found()
        {
            if self
                .detection
                .was_trade_successful(line, prev_line, true, ignore_combined)
                .is_found()
            {
                match self.trade_accepted() {
                    Ok(_) => {}
                    Err(_) => {}
                }
            } else if self
                .detection
                .was_trade_failed(line, prev_line, true, ignore_combined)
                .is_found()
            {
                self.trade_failed();
                write_zip();
            } else if self
                .detection
                .was_trade_cancelled(line, prev_line, true, ignore_combined)
                .is_found()
            {
                self.trade_cancelled();
                write_zip();
            }
            self.reset();
        }
        Ok((false, false))
    }
}

fn process_operations(trade: &PlayerTrade, operations: OperationSet) {
    let settings = match states::get_settings() {
        Ok(s) => s.notifications.on_new_trade,
        Err(e) => {
            e.log("OnTradeEvent");
            return;
        }
    };
    // Find Name:Item Name and Quantity:#
    let name = operations
        .get_value_after("Name")
        .unwrap_or_else(|| "Unknown Item".to_string());

    let quantity = operations
        .get_value_after("Quantity")
        .and_then(|n| n.parse::<i64>().ok())
        .unwrap_or(0);

    let (refresh_db, notify_type, notify_color) = if operations.has("AutoTrade") {
        if operations.has("MultipleItems") {
            (None, "multiple_items", "yellow")
        } else if operations.has("StockRiven_Deleted") {
            (Some(UIEvent::RefreshStockRivens), "success", "green.7")
        } else if !operations.has("WishListItemBought_NotFound") {
            (Some(UIEvent::RefreshWishListItems), "success", "green.7")
        } else if operations.any(&[
            "ItemSell_Deleted",
            "ItemSell_Updated",
            "ItemBuy_Created",
            "ItemBuy_Updated",
        ]) {
            (Some(UIEvent::RefreshStockItems), "success", "green.7")
        } else {
            (None, "", "")
        }
    } else {
        (None, "", "")
    };

    if let Some(event) = refresh_db {
        send_event!(event, json!({"source": "OnTradeEvent"}));
    }
    if !notify_type.is_empty() {
        notify_gui!(
            "on_trade_event",
            notify_color,
            notify_type,
            json!({
                "player_name": trade.player_name,
                "trade_type": trade.trade_type,
                "platinum": trade.platinum,
                "quantity": quantity,
                "item_name": name,
                "operations": json!(operations.operations)
            })
        );
    }

    settings.send(&trade.get_notify_variables(), Some(json!(trade)));

    info(
        get_component("TradeAccepted"),
        &trade.to_string(),
        &LoggerOptions::default(),
    );
}

async fn process_trade_item(
    item: TradeItem,
    platinum: i64,
    player_name: &str,
    order_type: OrderType,
) -> Result<OperationSet, Error> {
    let mut operations = OperationSet::new();
    operations.add(format!("Quantity:{}", item.quantity));
    // Handle Rivens
    if item.item_type == TradeItemType::RivenUnVeiled {
        let (op, model) = handle_riven_by_name(
            item.raw,
            &item.unique_name,
            item.sub_type.clone().unwrap_or_default(),
            platinum,
            player_name,
            order_type,
            &[],
        )
        .await
        .map_err(|e| e.with_location(get_location!()))?;
        operations.merge(&op);
        if !operations.has("StockRiven_NotFound") {
            if let Some(model) = model {
                operations.add(format!("Name:{} {}", model.weapon_name, model.mod_name));
            }
        }
        return Ok(operations);
    }

    // Handle Wish List
    let (op, model) = handle_wish_list(
        &item.unique_name,
        &item.sub_type,
        item.quantity,
        platinum,
        player_name,
        OrderType::Buy,
        crate::enums::FindByType::UniqueName,
        &["ReturnOn:NotFound", ""],
    )
    .await
    .map_err(|e| e.with_location(get_location!()))?;
    operations.merge(&op);
    if !op.has("WishListItemBought_NotFound") {
        operations.add(format!("Name:{}", model.item_name));
        return Ok(operations);
    }

    // Handle Stock Items
    let (op, model) = handle_item(
        item.unique_name.clone(),
        item.sub_type.clone(),
        item.quantity,
        platinum,
        player_name,
        order_type,
        crate::enums::FindByType::UniqueName,
        &["ReturnOn:NotFound", ""],
    )
    .await
    .map_err(|e| e.with_location(get_location!()))?;
    operations.merge(&op);
    if !op.ends_with("_NotFound") {
        operations.add(format!("Name:{}", model.item_name));
    }

    Ok(operations)
}
