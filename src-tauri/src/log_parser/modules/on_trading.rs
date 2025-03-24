use std::collections::HashMap;

use entity::{enums::stock_type::StockType, sub_type::SubType};
use eyre::eyre;

use serde_json::{json, Value};
use service::StockRivenQuery;

use crate::{
    cache::{client::CacheClient, types::cache_item_component::CacheItemComponent},
    helper,
    log_parser::{
        client::LogParser,
        enums::{trade_classification::TradeClassification, trade_item_type::TradeItemType},
        types::{
            create_stock_entity::CreateStockEntity,
            trade::PlayerTrade,
            trade_detection::{DetectionStatus, TradeDetection, DETECTIONS},
            trade_item::TradeItem,
        },
    },
    notification::client::NotifyClient,
    qf_client::client::QFClient,
    utils::{
        enums::{log_level::LogLevel, ui_events::UIEvent},
        modules::{
            error::{self, AppError},
            logger::{self, LoggerOptions},
            states,
            trading_helper::{
                notify, process_stock_item, process_stock_riven, process_wish_list, trace,
                trace_centered_message,
            },
        },
    },
    wfm_client::{client::WFMClient, enums::order_type::OrderType},
    DATABASE,
};

#[derive(Clone, Debug)]
pub struct OnTradeEvent {
    pub client: LogParser,
    component: String,
    detection: TradeDetection,
    logs: Vec<String>,
    getting_trade_message_multiline: bool,
    waiting_confirmation: bool,
    current_trade: PlayerTrade,
}

impl OnTradeEvent {
    pub fn new(client: LogParser) -> Self {
        let detections = DETECTIONS.get().unwrap();
        logger::clear_log_file("trade_trace.log").unwrap();
        OnTradeEvent {
            client,
            component: "OnTradeEvent".to_string(),
            detection: detections.get("en").unwrap().clone(),
            logs: vec![],
            getting_trade_message_multiline: false,
            waiting_confirmation: false,
            current_trade: PlayerTrade::default(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }
    fn update_state(&self) {
        self.client.update_trade_event(self.clone());
    }
    pub fn process_line(
        &mut self,
        line: &str,
        prev_line: &str,
        ignore_combined: bool,
    ) -> Result<bool, AppError> {
        while self.getting_trade_message_multiline {
            // Check if the line is the end of the trade
            if self
                .detection
                .is_end_of_trade(line, prev_line, true, ignore_combined)
                .is_found()
            {
                self.getting_trade_message_multiline = false;
                self.start_line_processing();
                self.waiting_confirmation = true;
                self.update_state();
                // self.trace("EndOfTrade", line, prev_line, ignore_combined);
            } else if !self.client.is_start_of_log(line) {
                self.add_trade_message(line);
                // self.trace("TradeMessage", line, prev_line, ignore_combined);
                return Ok(false);
            } else {
                return Ok(false);
            }
        }

        // Start of a Trade
        if self
            .detection
            .is_beginning_of_trade(line, prev_line, true, ignore_combined)
            .is_found()
        {
            trace_centered_message("New Trade");
            trace(
                format!(
                    "By: {} | Previous Line: {} | Ignore Combined: {}",
                    line, prev_line, ignore_combined
                )
                .as_str(),
            );
            self.trade_started(line, prev_line);
            self.getting_trade_message_multiline = true;
            self.update_state();
            return Ok(true);
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
                    Err(e) => {
                        error::create_log_file("trade_accepted.log", &e);
                    }
                }
            } else if self
                .detection
                .was_trade_failed(line, prev_line, true, ignore_combined)
                .is_found()
            {
                self.trade_failed();
            } else if self
                .detection
                .was_trade_cancelled(line, prev_line, true, ignore_combined)
                .is_found()
            {
                self.trade_cancelled();
            }
            self.reset();
            return Ok(false);
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
    pub fn trade_started(&mut self, line: &str, last_line: &str) {
        self.reset();
        self.add_trade_message(last_line);
        self.add_trade_message(line);
    }
    pub fn start_line_processing(&mut self) {
        self.current_trade.logs = self.logs.clone();
        let mut is_offering = true;
        let lines = self.logs.clone();
        // Loop through the trade logs by index
        let mut i = 0;
        trace_centered_message(format!("Processing {} Lines", lines.len()).as_str());
        while i < lines.len() {
            // Get the current line and next line.
            let line = lines[i].to_owned().replace("\r", "").replace("\n", "");
            let next_line = if i < lines.len() - 1 {
                lines[i + 1].to_owned().replace("\r", "").replace("\n", "")
            } else {
                "N/A".to_string()
            };

            let (is_irrelevant, status) =
                self.detection.is_irrelevant_trade_line(&line, &next_line);

            if !is_irrelevant {
                trace(format!("Skipping: Line: '{}', Next Line: '{}'", line, next_line).as_str());
                i += if status.is_combined() { 2 } else { 1 };
                continue;
            }

            let (full_line, is_offer_line) = self.detection.is_offer_line(&line, &next_line);

            if is_offer_line.is_found() {
                i += if is_offer_line.is_combined() { 2 } else { 1 };
                trace(
                    format!(
                        "From Player: {} | Is Offering: {}",
                        full_line,
                        is_offer_line.is_found()
                    )
                    .as_str(),
                );
                let player_name = full_line
                    .replace(&self.detection.receive_line_first_part, "")
                    .replace(&self.detection.receive_line_second_part, "")
                    .replace("\u{e000}", "")
                    .trim()
                    .to_string();
                self.current_trade.player_name = helper::remove_special_characters(&player_name);
                is_offering = false;
                continue;
            } else {
                let (status, item) = TradeItem::from_string(&line, &next_line, &self.detection);

                if status.is_combined() {
                    i += 1;
                }

                if !item.is_valid() {
                    trace(
                        format!(
                            "Item Not Found Line: {}, Next Line: '{}', Status: {:?}",
                            line, next_line, status
                        )
                        .as_str(),
                    );
                    i += 1;
                    continue;
                }
                trace(
                    format!(
                        "Item Valid: {}, Status: {:?}, Line: '{}', Next Line: '{}'",
                        item.display(),
                        status,
                        line,
                        next_line
                    )
                    .as_str(),
                );

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
                } else if is_offering {
                    self.current_trade.offered_items.push(item);
                } else {
                    self.current_trade.received_items.push(item);
                }
            }
            i += 1;
        }
        self.current_trade.trade_time = chrono::Local::now().to_string();
        self.current_trade.calculate();
        trace_centered_message("Waiting For Confirmation/Trade Failed/Trade Cancelled");
        self.update_state();
    }
    pub fn trade_cancelled(&self) {
        trace("Trade Cancelled");
        logger::info(
            &self.get_component("TradeCancelled"),
            "Trade cancelled",
            LoggerOptions::default(),
        );
    }
    pub fn trade_failed(&self) {
        trace("Trade Failed");
        logger::info(
            &self.get_component("TradeFailed"),
            "Trade failed",
            LoggerOptions::default(),
        );
    }
    pub fn trade_accepted(&self) -> Result<(), AppError> {
        trace("Trade Was Successful");
        trace_centered_message("Starting Processing Trade Items");
        let trade = self.current_trade.clone();
        let settings = states::settings()?;
        let trade_type = match trade.trade_type {
            TradeClassification::Sale => TradeClassification::Purchase,
            TradeClassification::Purchase => TradeClassification::Sale,
            _ => TradeClassification::Trade,
        };

        // Log the trade to a file
        match logger::log_json("trade.json", &json!(trade)) {
            Ok(_) => {}
            Err(_) => {}
        }

        // If the trade is not a sale or purchase, return
        if trade_type == TradeClassification::Trade {
            trace("Shipping Trade Type: Trade");
            logger::info(
                &self.get_component("TradeAccepted"),
                &trade.display(),
                LoggerOptions::default(),
            );
            notify(&trade, vec![String::from("Trade_Type: Trade")], None);
            return Ok(());
        }
        let component = self.get_component("TradeAccepted");
        tokio::task::spawn(async move {
            let stock_item = match trade.to_stock().await {
                Ok(stock) => stock,
                Err(e) => {
                    error::create_log_file("trade_accepted.log", &e);
                    notify(&trade, vec![e.cause().clone()], None);
                    return;
                }
            };
            trace(&format!("Stock Item: {:?}", stock_item.display()));
            if settings.live_scraper.stock_item.auto_trade {
                if stock_item.entity_type == StockType::Riven {
                    match process_stock_riven(&stock_item, &trade).await {
                        Ok(e) => {
                            notify(&trade, e, Some(&stock_item));
                        }
                        Err(e) => {
                            error::create_log_file("trade_accepted.log", &e);
                            notify(&trade, vec![e.cause().clone()], Some(&stock_item));
                        }
                    }
                } else if stock_item.entity_type == StockType::Item {
                    match process_stock_item(&stock_item, &trade).await {
                        Ok(e) => {
                            notify(&trade, e, Some(&stock_item));
                        }
                        Err(e) => {
                            error::create_log_file("trade_accepted.log", &e);
                            notify(&trade, vec![e.cause().clone()], Some(&stock_item));
                        }
                    }
                } else if stock_item.entity_type == StockType::WishList
                    && trade.trade_type == TradeClassification::Purchase
                {
                    match process_wish_list(&stock_item, &trade).await {
                        Ok(e) => {
                            notify(&trade, e, Some(&stock_item));
                        }
                        Err(e) => {
                            error::create_log_file("trade_accepted.log", &e);
                            notify(&trade, vec![e.cause().clone()], Some(&stock_item));
                        }
                    }
                } else {
                    logger::warning(
                        "TradeAccepted",
                        &format!(
                            "Unknown entity type: {:?} or trade type: {:?}",
                            stock_item.entity_type, trade.trade_type
                        ),
                        LoggerOptions::default(),
                    );
                    return;
                }
            }
            match logger::log_json("stock_item.json", &json!(stock_item)) {
                Ok(_) => {}
                Err(_) => {}
            }
            logger::info(&component, &trade.display(), LoggerOptions::default());
        });
        return Ok(());
    }
    pub fn add_trade_message(&mut self, line: &str) {
        self.logs.push(line.to_string());
        self.update_state();
    }
}
