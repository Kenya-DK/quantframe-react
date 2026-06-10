use std::sync::{LazyLock, Mutex};

use crate::{
    add_metric,
    enums::TradeItemType,
    handlers::{handle_item, handle_riven_by_name, handle_transaction, handle_wish_list},
    helper::get_or_create_window,
    log_parser::*,
    notify_gui, send_event,
    types::*,
    utils::{modules::states, SubTypeExt},
};
use entity::enums::TransactionType;
use serde_json::json;
use tauri::{Emitter, Listener};
use utils::*;
use wf_market::enums::OrderType;

pub static LOGGER: Mutex<Option<ZipLogger>> = Mutex::new(None);
pub static COMPONENT: Mutex<String> = Mutex::new(String::new());
pub static ENABLED_LOGGING: Mutex<bool> = Mutex::new(true);
static BASE_LOG_OPTIONS: LazyLock<LoggerOptions> = LazyLock::new(|| {
    LoggerOptions::default()
        .set_file("trade.log")
        .set_console(false)
        .set_show_elapsed_time(false)
        .set_show_component(false)
        .set_show_level(false)
});

pub fn log(content: impl Into<String>, options: Option<&LoggerOptions>) {
    if !*ENABLED_LOGGING.lock().unwrap() {
        return;
    }
    let content = content.into();
    let options = if let Some(opts) = options {
        opts
    } else {
        &BASE_LOG_OPTIONS
    };
    trace("OnTradeEvent", &content, options);
}
pub fn enable_logging(state: bool) {
    *ENABLED_LOGGING.lock().unwrap() = state;
}
fn get_component(component: &str) -> String {
    format!("{}:{}", COMPONENT.lock().unwrap().as_str(), component)
}
pub struct OnTradeEvent {
    detection: TradeDetection,
    logs: Vec<String>,
    getting_trade_message_multiline: bool,
    waiting_confirmation: bool,
    current_trade: PlayerTrade,
}

impl OnTradeEvent {
    pub fn new(base_component: &str) -> Self {
        delete_log("trade.log").ok();

        let detections = DETECTIONS.get().unwrap();
        *COMPONENT.lock().unwrap() = format!("{}:OnTradeEvent", base_component);
        OnTradeEvent {
            detection: detections.get("en").unwrap().clone(),
            logs: Vec::new(),
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
        log("State reset".to_string(), None);
    }
    pub fn trade_started(&mut self) {
        log(
            "Started",
            Some(&BASE_LOG_OPTIONS.set_width(180).set_centered(true)),
        );
        add_metric!("on_trade_event", "trade_started");
    }
    pub fn start_line_processing(&mut self) {
        let mut is_offering = true;
        let lines = self.logs.clone();
        self.current_trade.logs = lines.clone();

        let add_item = |item: TradeItem| {
            if is_offering {
                log(format!("Added offered item: {}", item), None);
                self.current_trade.offered_items.push(item);
            } else {
                log(format!("Added received item: {}", item), None);
                self.current_trade.received_items.push(item);
            }
        };

        log(
            format!("Processing {} trade message lines", lines.len()),
            None,
        );
        for line in &lines {
            log(format!("Trade message line: '{}'", line), None);
        }
        log("Beginning line-by-line processing of trade message", None);
        let mut ignore_combined = false;
        let mut prev_line = String::new();
        for i in 0..lines.len() {
            let line = lines[i].to_owned();

            println!("Processing Line {} | Prev Line: '{}'", line, prev_line);

            // First check if the line is a item,
            let (status, item) =
                TradeItem::from_string(&line, &prev_line, &self.detection, ignore_combined);

            log(
                format!(
                    "TradeItem | Line: '{}' | Prev Line: '{}'| Item Type: '{:?}' | Quantity: {} | Detected Type: {:?}",
                    line, prev_line, item.item_type, item.quantity, status
                ),
                None,
            );

            // let (full_line, is_offer_line) =
            //     self.detection
            //         .is_offer_line(&line, &prev_line, ignore_combined);

            // if matches!(
            //     is_offer_line,
            //     DetectionStatus::Line | DetectionStatus::Combined
            // ) {
            //     is_offering = false;
            //     let player_name = full_line
            //         .strip_prefix(&self.detection.receive_line_first_part)
            //         .and_then(|s| s.strip_suffix(&self.detection.receive_line_second_part))
            //         .unwrap_or("Unknown");
            //     self.current_trade.player_name = remove_special_characters(&player_name);
            //     log(
            //         format!(
            //         "Player identified: '{}' | switching to received items | Detection Type: {:?}",
            //         self.current_trade.player_name, is_offer_line
            //     ),
            //         None,
            //     );
            //     prev_line = line.clone();
            //     ignore_combined = matches!(is_offer_line, DetectionStatus::Combined);
            //     continue;
            // }
            // let (status, item) =
            //     TradeItem::from_string(&line, &prev_line, &self.detection, ignore_combined);

            // log(
            //         format!(
            //             "TradeItem | Line: '{}' | Prev Line: '{}'| Item Type: '{:?}' | Detected Type: {:?}",
            //             line, prev_line, item.item_type, status
            //         ),
            //         None,
            //     );
            // if status.is_combined() {
            //     log(
            //         format!("Combined line consumed | status: {:?}", status),
            //         None,
            //     );
            // }

            // if !item.is_valid() {
            //     log(
            //         format!("Skipped invalid parsed item | status: {:?}", status),
            //         None,
            //     );
            //     prev_line = line.clone();
            //     ignore_combined = status.is_combined();
            //     continue;
            // }

            // log(
            //     format!(
            //         "Parsed item: {} | status: {:?} | side: {}",
            //         item, status, is_offering
            //     ),
            //     None,
            // );

            // let mut items = if is_offering {
            //     self.current_trade.offered_items.iter_mut()
            // } else {
            //     self.current_trade.received_items.iter_mut()
            // };

            // if let Some(trade) = items.find(|p| {
            //     p.unique_name == item.unique_name
            //         && !item.unique_name.is_empty()
            //         && p.sub_type == item.sub_type
            // }) {
            //     trade.quantity += 1;
            //     log(
            //         format!(
            //             "Incremented item quantity: {} -> {}",
            //             trade.unique_name, trade.quantity
            //         ),
            //         None,
            //     );
            // } else if is_offering {
            //     log(format!("Added offered item: {}", item), None);
            //     self.current_trade.offered_items.push(item);
            // } else {
            //     log(format!("Added received item: {}", item), None);
            //     self.current_trade.received_items.push(item);
            // }
            prev_line = line.clone();
            ignore_combined = false;
        }

        self.current_trade.trade_time = chrono::Local::now().with_timezone(&chrono::Utc);
        log(
            format!("Trade time set: {}", self.current_trade.trade_time),
            None,
        );

        self.current_trade.calculate();
        log(
            format!(
                "Trade calculation complete | type: {:?}",
                self.current_trade.trade_type
            ),
            None,
        );
    }

    pub fn trade_accepted(&mut self) -> Result<(), Error> {
        log("Trade accepted".to_string(), None);

        let mut trade = self.current_trade.clone();
        self.reset();
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
                        Err(e) => {
                            e.log("creating_trade_json.log");
                        }
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
            log(
                "Simple trade detected; skipping auto-processing".to_string(),
                None,
            );
            return Ok(());
        }
        tauri::async_runtime::spawn({
            async move {
                trade.calculate_items();
                let items = trade.get_valid_items(&trade_type, vec![]);
                let mut operations = OperationSet::new();
                let item = items.first();
                if settings.live_scraper.auto_trade {
                    operations.add("AutoTrade");
                }
                log(format!("Valid trade items found: {}", items.len()), None);
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
                let item = item.unwrap().clone();
                if items.len() > 1 || item.properties.get_property_value("requireSubType", false) {
                    operations.add("MultipleItems");
                    match process_mutable_items(&trade, trade_type, order_type).await {
                        Ok(op) => operations.merge(&op),
                        Err(mut e) => {
                            e = e.with_location(get_location!());
                            log(e.to_string(), None);
                        }
                    }
                } else {
                    operations.add("Found");
                }

                if operations.any(&["Found", "SetFound"]) && operations.has("AutoTrade") {
                    match process_trade_item(item, trade.platinum, &trade.player_name, order_type)
                        .await
                    {
                        Ok(op) => operations.merge(&op),
                        Err(mut e) => {
                            e = e.with_location(get_location!());
                            log(e.to_string(), None);
                            return;
                        }
                    }
                }
                let msg = format!(
                    "Trade processed | operations: {}",
                    operations.operations.join(", ")
                );
                log(msg, None);
                process_operations(&trade, operations);
            }
        });
        add_metric!("on_trade_event", "trade_accepted");
        Ok(())
    }
    pub fn add_trade_message(&mut self, line: &str) {
        let line = line.replace('\n', "").replace('\r', "");
        if line.is_empty() {
            return;
        }
        self.logs.push(line);
    }
}

impl LineHandler for OnTradeEvent {
    fn process_line(&mut self, entry: &LineEntry) -> Result<(bool, bool), Error> {
        // Handle multiline trade message collection
        if self.getting_trade_message_multiline {
            // Detect start of trade
            let trade_end = self.detection.is_end_of_trade(
                &entry.line,
                &entry.prev_line,
                entry.ignore_combined,
            );

            if trade_end.is_found() {
                self.getting_trade_message_multiline = false;
                self.waiting_confirmation = true;
                self.start_line_processing();
                log(
                    "Waiting For Confirmation/Trade Failed/Trade Cancelled",
                    None,
                );
                return Ok((false, !trade_end.is_combined()));
            } else if !is_start_of_log(&entry.line) {
                self.add_trade_message(&entry.line);
                return Ok((false, false));
            } else {
                return Ok((false, false));
            }
        }

        // Detect start of trade
        let trade_start = self.detection.is_beginning_of_trade(
            &entry.line,
            &entry.prev_line,
            entry.ignore_combined,
        );

        if trade_start.is_found() {
            self.getting_trade_message_multiline = true;
            self.trade_started();
            return Ok((true, !trade_start.is_combined()));
        }
        // Nothing else to do unless we're waiting for confirmation
        if !self.waiting_confirmation || entry.ignore_combined {
            return Ok((false, false));
        }

        match self
            .detection
            .get_trade_result(&entry.line, &entry.prev_line, entry.ignore_combined)
        {
            (status, TradeResult::Success) => {
                self.trade_accepted()?;
                self.waiting_confirmation = false;
                return Ok((
                    true,
                    matches!(
                        status,
                        DetectionStatus::Line | DetectionStatus::PreviousLine
                    ),
                ));
            }
            (status, TradeResult::Failed) => {
                log("Failed".to_string(), None);
                self.reset();
                add_metric!("on_trade_event", "trade_failed");
                self.waiting_confirmation = false;
                return Ok((true, status.is_combined()));
            }
            (status, TradeResult::Cancelled) => {
                log("Cancelled".to_string(), None);
                self.reset();
                add_metric!("on_trade_event", "trade_cancelled");
                self.waiting_confirmation = false;
                return Ok((true, status.is_combined()));
            }
            _ => {}
        }
        Ok((false, false))
    }
}

async fn process_mutable_items(
    trade: &PlayerTrade,
    trade_type: TradeClassification,
    order_type: OrderType,
) -> Result<OperationSet, Error> {
    let mut operations = OperationSet::new();
    log("Processing mutable trade items".to_string(), None);
    let (is_open, window) = get_or_create_window(
        "processing-trades",
        "clean?type=process_trade",
        "Processing Trades",
        Some((800.0, 600.0)),
        true,
    )?;

    let app = states::app_state()?.clone();

    let mut items = match trade_type {
        TradeClassification::Purchase => trade.offered_items.clone(),
        TradeClassification::Sale => trade.received_items.clone(),
        _ => return Ok(operations),
    };
    for item in &mut items {
        if !item.is_valid() {
            log(format!("Skipping invalid item {}", item), None);

            continue;
        }

        let info = match item.get_trade_item_info() {
            Ok(info) => info,
            Err(_) => {
                log(format!("Skipped item (missing trade info): {}", item), None);
                continue;
            }
        };

        item.properties.set_property_value("name", json!(info.name));
        item.properties
            .set_property_value("subTypes", json!(info.sub_type));
        item.properties
            .set_property_value("wfm_url", json!(info.wfm_url));

        let price = app
            .wfm_client
            .order()
            .cache_orders()
            .find_order(
                &info.wfm_id,
                &SubTypeExt::from_entity(item.sub_type.clone()),
                order_type,
            )
            .map(|order| order.platinum)
            .unwrap_or(0);
        log(
            format!("Resolved price | item: {} | platinum: {}", info.name, price),
            None,
        );
        item.properties.set_property_value("price", json!(price));
    }

    let mut payload = json!(trade);
    if let Some(obj) = payload.as_object_mut() {
        obj.remove("offeredItems");
        obj.remove("receivedItems");
        obj.remove("logs");
    }
    // Remove invalid item from the list
    items.retain(|i| i.is_valid());
    payload["items"] = json!(items);

    let window_clone = window.clone();
    let payload_clone = payload.clone();

    window.once("initialize", move |_| {
        let _ = window_clone.emit("add_trade", payload_clone);
        log("processing-trades window initialized", None);
    });

    if is_open {
        let _ = window.emit("add_trade", payload);
        log("Dispatched add_trade to processing-trades window", None);
    }
    operations.add(format!("Items:{}", items.len()));
    Ok(operations)
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
    operations.add(format!("Quantity: {}", item.quantity));
    // Handle Imprints
    if item.item_type == TradeItemType::Imprint {
        let model = handle_transaction(
            entity::transaction::Model::new(
                item.properties
                    .get_property_value("wfmUrl", "Unknown Imprint".to_string()),
                item.properties
                    .get_property_value("wfmUrl", "Unknown Imprint".to_string()),
                item.properties.get_property_value("name", "".to_string()),
                entity::enums::TransactionItemType::Item,
                item.unique_name.clone(),
                item.sub_type.clone(),
                item.properties.get_property_value("tags", vec![]),
                if order_type == OrderType::Buy {
                    TransactionType::Purchase
                } else {
                    TransactionType::Sale
                },
                item.quantity,
                player_name,
                platinum,
                2000 * item.quantity,
                Some(json!({
                    "petName": item.sub_type.unwrap().variant.unwrap_or("Unknown".to_string())
                })),
            ),
            &operations,
        )
        .await
        .map_err(|e| e.with_location(get_location!()))?;
        operations.add(format!("Name: {}", model.item_name));
        return Ok(operations);
    }

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
                operations.add(format!("Name: {} {}", model.weapon_name, model.mod_name));
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
        &OperationSet::from(vec!["ReturnOn:NotFound", ""]),
    )
    .await
    .map_err(|e| e.with_location(get_location!()))?;
    operations.merge(&op);
    if !op.has("WishListItemBought_NotFound") {
        operations.add(format!("Name: {}", model.item_name));
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
        OperationSet::from(vec!["SkipWFMCheck:ItemSell_NotFound"]), // Will skip WFM check if the item is not found in Stock when selling.
    )
    .await
    .map_err(|e| e.with_location(get_location!()))?;
    operations.merge(&op);
    if !op.ends_with("_NotFound") {
        operations.add(format!("Name: {}", model.item_name));
    }
    Ok(operations)
}
