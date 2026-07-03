use std::sync::{LazyLock, Mutex};

use crate::{
    add_metric,
    app::Settings,
    enums::TradeItemType,
    handlers::{handle_item, handle_riven_by_name, handle_transaction, handle_wish_list},
    helper::get_or_create_window,
    log_parser::*,
    notify_gui, send_event,
    types::*,
    utils::{modules::states, SubTypeExt},
    APP,
};
use entity::enums::TransactionType;
use serde_json::json;
use tauri::{Emitter, Listener, Manager};
use utils::*;
use wf_market::enums::OrderType;
//----------------------------
//         CORE STATE
//----------------------------

pub static COMPONENT: Mutex<String> = Mutex::new(String::new());
pub static ENABLED_LOGGING: Mutex<bool> = Mutex::new(true);

pub fn enable_logging(state: bool) {
    *ENABLED_LOGGING.lock().unwrap() = state;
}

fn get_component(component: &str) -> String {
    format!("{}:{}", COMPONENT.lock().unwrap().as_str(), component)
}

//----------------------------
//         TRADE EVENT STRUCT
//----------------------------

pub struct OnTradeEvent {
    detection: TradeDetection,
    logs: Vec<LineEntry>,
    current_trade: PlayerTrade,
    operations: OperationSet,
    watcher: FileWatcher,
    logger: ZipLogger,
}

//----------------------------
//         INITIALIZATION
//----------------------------

impl OnTradeEvent {
    pub fn new(base_component: &str, watcher: FileWatcher) -> Self {
        let detections = DETECTIONS.get().unwrap();
        delete_log("trade.log").ok();

        *COMPONENT.lock().unwrap() = format!("{}:OnTradeEvent", base_component);

        Self {
            detection: detections.get("en").unwrap().clone(),
            logs: Vec::new(),
            current_trade: PlayerTrade::default(),
            operations: OperationSet::new(),
            watcher,
            logger: ZipLogger::new(),
        }
    }

    pub fn reset(&mut self) {
        self.logs.clear();
        self.current_trade = PlayerTrade::default();
        self.operations = OperationSet::new();
    }
}

//----------------------------
//         LOGGING HELPERS
//----------------------------

impl OnTradeEvent {
    fn log_trade_summary(&self) {
        self.logger
            .add_log(format!("Type: {}", self.current_trade.trade_type.display()));
        self.logger
            .add_log(format!("Platinum: {}", self.current_trade.platinum));
        self.logger
            .add_log(format!("Credits: {}", self.current_trade.credits));
    }

    fn log_trade_items(&self) {
        self.logger.add_log("Offered Items:");
        for item in &self.current_trade.offered_items {
            self.logger.add_log(format!(" - {}", item));
        }

        self.logger.add_log("Received Items:");
        for item in &self.current_trade.received_items {
            self.logger.add_log(format!(" - {}", item));
        }
    }
    fn create_log_file(&self) -> Result<(), Error> {
        let timestamp = chrono::Local::now()
            .with_timezone(&chrono::Utc)
            .format("%Y_%m_%d_%H_%M_%S")
            .to_string();
        let log_start = self.logs.first().cloned().unwrap_or_default();
        let log_end = self.logs.last().cloned().unwrap_or_default();
        let raw_logs = self
            .watcher
            .get_cached_lines_between(log_start.index.saturating_sub(5), log_end.index + 5);
        self.logger.create_file(
            "RawEELogs.txt",
            format!("{}", json!(raw_logs).to_string()).as_bytes(),
        );
        self.logger.finalize(format!("{}_TRADE.zip", timestamp))?;
        Ok(())
    }
}

//----------------------------
//         TRADE LIFECYCLE
//----------------------------

impl OnTradeEvent {
    pub fn trade_accepted(&mut self) -> Result<(), Error> {
        self.logger.add_log("Finalize Trade");

        self.current_trade.finalize_trade();
        self.log_trade_summary();

        self.current_trade.finalize_items();
        self.log_trade_items();

        let settings = states::get_settings()?.clone();

        let (trade_type, order_type) = match self.current_trade.trade_type {
            TradeClassification::Sale => (TradeClassification::Purchase, OrderType::Sell),
            TradeClassification::Purchase => (TradeClassification::Sale, OrderType::Buy),
            _ => return Ok(()),
        };

        self.logger.add_log("Switching to mode: Processing Trade");

        self.spawn_trade_processor(settings, trade_type, order_type);

        add_metric!("on_trade_event", "trade_accepted");
        Ok(())
    }
}

//----------------------------
//         ITEM HANDLING
//----------------------------

impl OnTradeEvent {
    pub fn add_item(&mut self, item: TradeItem) {
        let (items, id) = if self.operations.has("Receiving") {
            (&mut self.current_trade.received_items, "Received")
        } else {
            (&mut self.current_trade.offered_items, "Offered")
        };

        if let Some(existing) = items
            .iter_mut()
            .find(|i| i.unique_name == item.unique_name && i.sub_type == item.sub_type)
        {
            existing.quantity += item.quantity;
            self.logger
                .add_log(format!("Updating item in {} items", id));
            self.logger
                .add_log(format!("       Quantity: {} -> {}", existing, item));
            return;
        }

        self.logger.add_log(format!("Adding item to {} items", id));
        self.logger.add_log(format!("       {item}"));
        items.push(item);
    }
}

//----------------------------
//         LOG PARSING ENGINE
//----------------------------

impl OnTradeEvent {
    pub fn start_process_logs(&mut self) {
        let lines = self.logs.clone();
        self.logger.add_log("");

        let advance = |status: DetectionStatus| {
            if status.is_combined() {
                2
            } else {
                1
            }
        };

        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].line.clone();
            let next_line = lines.get(i + 1).map(|l| l.line.clone()).unwrap_or_default();

            let log_line = format!("Line: '{}' | Next Line: '{}'", line, next_line);

            let (player_name, status) = self.detection.is_offer_line(&line, &next_line, &[]);

            if status.is_found() && status != DetectionStatus::PreviousLine {
                self.operations.add("Receiving");
                self.current_trade.player_name = player_name;
                i += advance(status);
                continue;
            }

            let (status, item) = TradeItem::from_string(&line, &next_line, &self.detection, &[]);

            if status.is_found() {
                self.add_item(item);
            }

            i += advance(status);
        }
    }
}

//----------------------------
//         TRADE PROCESSING PIPELINE
//----------------------------

impl OnTradeEvent {
    fn spawn_trade_processor(
        &self,
        settings: Settings,
        trade_type: TradeClassification,
        order_type: OrderType,
    ) {
        let trade = self.current_trade.clone();
        let logger = self.logger.clone();

        tauri::async_runtime::spawn(async move {
            logger.add_log("Trade processor started");

            let items = trade.get_valid_items(&trade_type, vec![]);
            let mut operations = OperationSet::new();

            logger.add_log(format!(
                "Valid items found: {} | Player: {}",
                items.len(),
                trade.player_name
            ));

            if settings.live_scraper.auto_trade {
                operations.add("AutoTrade");
                logger.add_log("AutoTrade enabled");
            }

            let Some(item) = items.first().cloned() else {
                handle_missing_items(&trade, &logger);
                return;
            };
            logger.add_log(format!("Primary item: {}", item));
            if needs_multi_processing(&items, &item) {
                handle_multi_items(&trade, trade_type, order_type, &mut operations, &logger).await;
            } else {
                operations.add("Found");
                logger.add_log("Single item found, skipping multi-item processing");
            }

            execute_auto_trade_if_needed(&trade, order_type, item, &mut operations, &logger).await;

            logger.add_log(format!(
                "Trade completed with operations: {}",
                operations.operations.join(", ")
            ));

            process_operations(&trade, operations);
        });
    }
}

//----------------------------
//         MULTI-ITEM PROCESSING
//----------------------------

fn needs_multi_processing(items: &[TradeItem], item: &TradeItem) -> bool {
    items.len() > 1 || item.properties.get_property_value("requireSubType", false)
}

fn handle_missing_items(trade: &PlayerTrade, logger: &ZipLogger) {
    logger.add_log("No valid items found");

    warning(
        "OnTradeEvent",
        "No valid items found in trade",
        &LoggerOptions::default(),
    );

    notify_gui!(
        "on_trade_event",
        "yellow",
        "no_valid_items",
        json!({ "player_name": trade.player_name })
    );
}

//----------------------------
//         MULTI ITEM HANDLER
//----------------------------

async fn handle_multi_items(
    trade: &PlayerTrade,
    trade_type: TradeClassification,
    order_type: OrderType,
    operations: &mut OperationSet,
    logger: &ZipLogger,
) {
    operations.add("MultipleItems");
    logger.add_log("Multiple items or subtype requirement detected");

    match process_mutable_items(trade, trade_type, order_type).await {
        Ok(op) => {
            logger.add_log(&format!(
                "Mutable item processing completed | Operations: {:?}",
                op.operations
            ));
            operations.merge(&op);
        }
        Err(mut e) => {
            e = e.with_location(get_location!());
            logger.add_log(&format!("Error in process_mutable_items | Error: {}", e));
        }
    }
}

//----------------------------
//         AUTO TRADE LOGIC
//----------------------------

async fn execute_auto_trade_if_needed(
    trade: &PlayerTrade,
    order_type: OrderType,
    item: TradeItem,
    operations: &mut OperationSet,
    logger: &ZipLogger,
) {
    if !(operations.any(&["Found", "SetFound"]) && operations.has("AutoTrade")) {
        logger.add_log("AutoTrade skipped");
        return;
    }

    match process_trade_item(item, trade.platinum, &trade.player_name, order_type).await {
        Ok(op) => {
            logger.add_log(&format!(
                "AutoTrade processing completed | Operations: {:?}",
                op.operations
            ));
            operations.merge(&op);
        }
        Err(e) => logger.add_log(&format!("Error in AutoTrade | Error: {}", e)),
    }
}

//----------------------------
//         CACHE + MUTATION LOGIC
//----------------------------

async fn process_mutable_items(
    trade: &PlayerTrade,
    trade_type: TradeClassification,
    order_type: OrderType,
) -> Result<OperationSet, Error> {
    let mut operations = OperationSet::new();
    // log("Processing mutable trade items".to_string(), None);
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
            continue;
        }

        let info = match item.get_trade_item_info() {
            Ok(info) => info,
            Err(_) => continue,
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
    });

    if is_open {
        let _ = window.emit("add_trade", payload);
    }
    operations.add(format!("Items:{}", items.len()));
    Ok(operations)
}

//----------------------------
//         OPERATION RESOLUTION
//----------------------------

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

//----------------------------
//         PROCESS TRADE ITEM (DOMAIN ROUTER)
//----------------------------

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

//----------------------------
//         LINE HANDLER IMPLEMENTATION
//----------------------------

impl LineHandler for OnTradeEvent {
    fn process_line(&mut self, entry: &LineEntry) -> Result<(bool, DetectionStatus), Error> {
        // Handle multiline trade message collection
        if self.operations.has("GettingTradeMessage") {
            let trade_end = self.detection.is_end_of_trade(
                &entry.line,
                &entry.prev_line,
                &[DetectionStatus::Combined],
            );
            if trade_end.is_found() {
                self.operations.remove("GettingTradeMessage");
                self.operations.add("WaitingConfirmation");
                self.logger.add_log("Trade Message Collection Ended With:");
                self.logger
                    .add_log(format!("       {entry} | Detection: {:?}", trade_end));
                self.logger
                    .add_log("Switching to mode: Waiting for Trade Confirmation");
                return Ok((false, trade_end));
            } else if !is_start_of_log(&entry.line) {
                let mut lie = entry.clone();
                lie.clear_newlines();
                self.logger.add_log(format!("Add message {entry}"));
                self.logs.push(lie);
            } else {
                return Ok((false, DetectionStatus::None));
            }
        }
        // Detect start of trade
        let trade_start = self.detection.is_beginning_of_trade(
            &entry.line,
            &entry.prev_line,
            &[entry
                .prev_detection
                .replace_if_matches(&[DetectionStatus::Line], DetectionStatus::Combined)],
        );
        if trade_start.is_found() {
            self.logger = ZipLogger::new();
            self.logger.operations.add("DumpLog:trade.log");
            self.logger.add_log("Initialized Trade Logger");
            self.logger.add_log("Trade Started With:");
            self.logger
                .add_log(format!("       {entry} | Detection: {:?}", trade_start));

            self.operations.add("TradeStarted");

            self.current_trade.trade_time = chrono::Local::now().with_timezone(&chrono::Utc);

            self.operations.add("GettingTradeMessage");
            self.logger
                .add_log("Switching to mode: Collecting Trade Message");

            add_metric!("on_trade_event", "trade_started");

            return Ok((true, trade_start));
        }

        // Nothing else to do unless we're waiting for confirmation
        if !self.operations.has("WaitingConfirmation") {
            return Ok((false, DetectionStatus::None));
        }

        let (status, result) = self.detection.get_trade_result(
            &entry.line,
            &entry.prev_line,
            &[entry
                .prev_detection
                .replace_if_matches(&[DetectionStatus::Line], DetectionStatus::Combined)],
        );
        if result == TradeResult::Unknown {
            return Ok((false, status));
        }

        self.logger
            .add_log(format!("Trade {} With:", result.display()));
        self.logger
            .add_log(format!("       {entry} | Detection: {:?}", status));
        add_metric!("on_trade_event", result.metric_name());
        self.logger.add_log(format!(
            "Switching to mode: Processing Trade Logs {} message lines collected",
            self.logs.len()
        ));
        self.start_process_logs();
        match result {
            TradeResult::Failed | TradeResult::Cancelled => {
                self.logger.add_log("Done");
            }
            TradeResult::Success => {
                self.logger
                    .add_log("Switching to mode: Finalize trade and items");
                self.trade_accepted()?;
            }
            _ => {}
        }
        self.create_log_file()?;
        self.reset();
        Ok((false, DetectionStatus::None))
    }
}
