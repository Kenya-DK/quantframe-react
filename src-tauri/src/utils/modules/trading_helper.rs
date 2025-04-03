use serde_json::json;
use service::StockRivenQuery;

use crate::{
    helper,
    log_parser::{
        enums::{trade_classification::TradeClassification, trade_item_type::TradeItemType},
        types::{
            create_stock_entity::CreateStockEntity, trade::PlayerTrade,
            trade_detection::DetectionStatus,
        },
    },
    utils::enums::ui_events::UIEvent,
    wfm_client::enums::order_type::OrderType,
    DATABASE,
};

use super::{
    error::AppError,
    logger::{self, LoggerOptions},
    states,
};

fn contains_any_match(line: &str, match_patterns: &[&str], is_exact_match: bool) -> bool {
    match_patterns
        .iter()
        .all(|&pattern| contains_match(line, pattern, is_exact_match))
}
fn contains_match(line: &str, match_pattern: &str, is_exact_match: bool) -> bool {
    if is_exact_match {
        line == match_pattern
    } else {
        line.contains(match_pattern)
    }
}

pub fn contains_unicode(
    line: &str,
    next_line: &str,
    use_previous_line: bool,
) -> (String, DetectionStatus) {
    if line.len() != line.chars().count() {
        return (line.to_string(), DetectionStatus::Line);
    }

    let trimmed_combined = if use_previous_line {
        next_line.to_owned() + line
    } else {
        line.to_owned() + next_line
    };

    if trimmed_combined.len() != trimmed_combined.chars().count() {
        return (trimmed_combined, DetectionStatus::Combined);
    }

    if !use_previous_line {
        return (line.to_string(), DetectionStatus::None);
    } else {
        return (next_line.to_string(), DetectionStatus::None);
    }
}

pub fn combine_and_detect_match(
    line: &str,
    next_line: &str,
    match_pattern: &str,
    use_previous_line: bool,
    is_exact_match: bool,
) -> (String, DetectionStatus) {
    if !use_previous_line && next_line == "" {
        return (line.to_string(), DetectionStatus::None);
    } else if use_previous_line && line == "" {
        return (next_line.to_string(), DetectionStatus::None);
    }

    let trimmed_combined = if use_previous_line {
        next_line.to_owned() + line
    } else {
        line.to_owned() + next_line
    };

    if contains_match(&trimmed_combined, match_pattern, is_exact_match) {
        return (trimmed_combined, DetectionStatus::Combined);
    }

    if !use_previous_line {
        return (line.to_string(), DetectionStatus::None);
    } else {
        return (next_line.to_string(), DetectionStatus::None);
    }
}
pub fn combine_and_detect_multiple_matches(
    line: &str,
    next_line: &str,
    match_patterns: &[&str],
    use_previous_line: bool,
    is_exact_match: bool,
) -> (String, DetectionStatus) {
    if !use_previous_line && next_line.is_empty() {
        return (line.to_string(), DetectionStatus::None);
    } else if use_previous_line && line.is_empty() {
        return (next_line.to_string(), DetectionStatus::None);
    }

    let concatenated_string = if use_previous_line {
        next_line.to_owned() + line
    } else {
        line.to_owned() + next_line
    };

    if contains_any_match(&concatenated_string, match_patterns, is_exact_match) {
        return (concatenated_string, DetectionStatus::Combined);
    }

    if !use_previous_line {
        return (line.to_string(), DetectionStatus::None);
    } else {
        return (next_line.to_string(), DetectionStatus::None);
    }
}

pub fn tags_to_type(tags: Vec<&str>) -> TradeItemType {
    match () {
        _ if tags.contains(&"main_part") => TradeItemType::MainBlueprint,
        _ if tags.contains(&"blueprint") && !tags.contains(&"component") => {
            TradeItemType::MainBlueprint
        }
        _ if tags.contains(&"weapon") => TradeItemType::Weapon,
        _ if tags.contains(&"relic") => TradeItemType::Relic,
        _ if tags.contains(&"component") => TradeItemType::Component,
        _ if tags.contains(&"lens") => TradeItemType::Lens,
        _ if tags.contains(&"arcane_enhancement") => TradeItemType::Arcane,
        _ if tags.contains(&"mod") => TradeItemType::Mod,
        _ if tags.contains(&"fish") => TradeItemType::Fish,
        _ => TradeItemType::Unknown,
    }
}

pub fn contains_at_least(haystack: &str, needles: &str, count: usize, exact: bool) -> bool {
    let found = haystack.chars().filter(|c| needles.contains(*c)).count();
    if exact {
        found == count
    } else {
        found >= count
    }
}

pub fn parse_quantity(raw: &str) -> (String, i64) {
    if let Some((name, qty)) = raw.split_once(" x ") {
        let quantity = qty.trim().parse().unwrap_or(1);
        (name.to_string(), quantity)
    } else {
        (raw.to_string(), 1)
    }
}

pub fn trace_centered_message(message: &str) {
    let total_width = 180;
    let message_len = message.len();

    if message_len >= total_width {
        trace(message);
        return;
    }

    let padding = total_width - message_len;
    let left_padding = padding / 2;
    let right_padding = padding - left_padding;

    let line = format!(
        "{}{}{}",
        "-".repeat(left_padding),
        message,
        "-".repeat(right_padding)
    );

    trace(&line);
}
pub fn trace(msg: &str) {
    logger::trace(
        "TradeHelper",
        msg,
        LoggerOptions::default()
            .set_file("trade_trace.log")
            .set_show_component(false)
            .set_show_level(false)
            .set_console(false)
            .set_show_time(false),
    );
}

pub async fn process_stock_riven(
    created_stock: &CreateStockEntity,
    trade: &PlayerTrade,
) -> Result<Vec<String>, AppError> {
    let con = DATABASE.get().unwrap();
    let mut operations: Vec<String> = vec![];
    if trade.trade_type == TradeClassification::Purchase {
        operations.push("StockRiven_Skipped".to_string());
        return Ok(operations);
    }
    // Find Stock
    let stock = match StockRivenQuery::get_by_riven_name(
        &con,
        &created_stock.wfm_url,
        &created_stock.mod_name,
        created_stock.sub_type.clone().unwrap(),
    )
    .await
    {
        Ok(stock_riven) => stock_riven,
        Err(e) => {
            return Err(AppError::new_db("StockRiven_NotFound", e));
        }
    };
    if stock.is_none() {
        operations.push("StockRiven_NotFound".to_string());
        return Ok(operations);
    }
    let stock = stock.unwrap();

    match helper::progress_stock_riven(
        &mut stock.to_create(trade.platinum),
        "--weapon_by url_name --weapon_lang en --attribute_by url_name",
        &trade.player_name,
        OrderType::Sell,
        "auto",
    )
    .await
    {
        Ok((_, mut rep)) => {
            operations.append(&mut rep);
        }
        Err(e) => {
            return Err(e);
        }
    }
    return Ok(operations);
}

pub async fn process_stock_item(
    created_stock: &CreateStockEntity,
    trade: &PlayerTrade,
) -> Result<Vec<String>, AppError> {
    let mut operations: Vec<String> = vec![];

    match helper::progress_stock_item(
        &mut created_stock.to_stock_item(),
        "--item_by url_name --item_lang en",
        &trade.player_name,
        trade.trade_type.to_order_type(),
        vec![
            "StockContinueOnError".to_string(),
            "WFMContinueOnError".to_string(),
        ],
        "auto",
    )
    .await
    {
        Ok((_, mut rep)) => {
            operations.append(&mut rep);
        }
        Err(_) => {}
    }

    return Ok(operations);
}

pub async fn process_wish_list(
    created_stock: &CreateStockEntity,
    trade: &PlayerTrade,
) -> Result<Vec<String>, AppError> {
    let mut operations: Vec<String> = vec![];

    match helper::progress_wish_item(
        &mut created_stock.to_wish_item(),
        "--item_by url_name --item_lang en",
        &trade.player_name,
        trade.trade_type.to_order_type(),
        vec![
            "StockContinueOnError".to_string(),
            "WFMContinueOnError".to_string(),
        ],
        "auto",
    )
    .await
    {
        Ok((_, mut rep)) => {
            operations.append(&mut rep);
        }
        Err(_) => {}
    }
    return Ok(operations);
}

fn contains_warning(operations: &[String], keywords: &[&str]) -> bool {
    operations
        .iter()
        .any(|op| keywords.iter().any(|keyword| op.contains(keyword)))
}

pub fn notify(
    trade: &PlayerTrade,
    operations: Vec<String>,
    stock_item: Option<&CreateStockEntity>,
) {
    let notify = states::notify_client().expect("Notify Client not initialized");
    let settings = states::settings().expect("Settings Client not initialized");
    let gui_id = "on_trade_event";
    // Set Notification's Data
    let mut notify_payload = json!({
        "i18n_key_title": "",
        "i18n_key_message": "",
        "autoClose": true,
    });
    let mut notify_value = json!({
        "player_name": trade.player_name,
        "trade_type": trade.trade_type,
        "platinum": trade.platinum,
        "wfm_operation": "None",
        "stock_operation": "None",
        "operations": json!(operations)
    });

    notify_payload["i18n_key_title"] = format!("{}.title", gui_id).into();

    notify_payload["i18n_key_message"] = format!("{}.message", gui_id).into();

    // Set Item Name
    if let Some(stock_item) = stock_item {
        match stock_item.get_name() {
            Ok(name) => {
                notify_value["item_name"] = json!(name);
            }
            Err(_) => {
                notify_value["item_name"] = json!(&format!("{}", stock_item.raw));
            }
        }
        notify_value["quantity"] = json!(stock_item.quantity);
    }

    // Warnings
    if contains_warning(&operations, &["Multiple items found"]) {
        notify_payload["i18n_key_message"] = format!("{}.multiple_items_found", gui_id).into();
    }
    if contains_warning(&operations, &["No valid items found"]) {
        notify_payload["i18n_key_message"] = format!("{}.no_valid_items_found", gui_id).into();
    }
    if contains_warning(&operations, &["Set Not valid"]) {
        notify_payload["i18n_key_message"] = format!("{}.set_not_valid", gui_id).into();
    }

    // Operation
    if contains_warning(
        &operations,
        &[
            "StockRiven_NotFound",
            "StockItem_NotFound",
            "WishItem_NotFound",
        ],
    ) {
        notify_value["stock_operation"] = json!("Not Found");
    }
    if contains_warning(
        &operations,
        &[
            "StockRiven_Deleted",
            "StockItem_Deleted",
            "WishItem_Deleted",
        ],
    ) {
        notify_value["stock_operation"] = json!("Deleted");
    }
    if contains_warning(&operations, &["WFM_RivenDeleted", "WFM_Deleted"]) {
        notify_value["wfm_operation"] = json!("Deleted");
    }
    if contains_warning(&operations, &["WFM_Updated"]) {
        notify_value["wfm_operation"] = json!("Deleted");
    }
    if contains_warning(&operations, &["WFM_NotFound"]) {
        notify_value["wfm_operation"] = json!("Not Found");
    }
    notify_payload["values"] = notify_value.clone();

    let event = if contains_warning(
        &operations,
        &[
            "No valid items found",
            "Multiple items found",
            "Set Not valid",
        ],
    ) {
        UIEvent::OnNotificationWarning
    } else {
        UIEvent::OnNotificationSuccess
    };

    // Send to GUI
    notify.gui().send_event(event, Some(notify_payload));

    let title: String = settings
        .notifications
        .on_new_trade
        .title
        .replace("<TR_TYPE>", trade.trade_type.to_str());
    let content: String = settings
        .notifications
        .on_new_trade
        .content
        .replace("<PLAYER_NAME>", trade.player_name.as_str())
        .replace("<OF_COUNT>", &trade.offered_items.len().to_string())
        .replace("<RE_COUNT>", &trade.received_items.len().to_string())
        .replace("<TOTAL_PLAT>", &trade.platinum.to_string());
    // Notification to system
    if settings.notifications.on_new_trade.system_notify {
        notify
            .system()
            .send_notification(&title, &content, None, None);
    }
    // Discord Webhook
    if settings.notifications.on_new_trade.discord_notify
        && settings.notifications.on_new_trade.webhook.is_some()
    {
        let offered_items = trade
            .get_valid_items(&TradeClassification::Purchase)
            .iter()
            .map(|x| format!("{} X{}", x.item_name(), x.quantity))
            .collect::<Vec<String>>()
            .join("\n");

        let received_items = trade
            .get_valid_items(&TradeClassification::Sale)
            .iter()
            .map(|x| format!("{} X{}", x.item_name(), x.quantity))
            .collect::<Vec<String>>()
            .join("\n");
        notify.discord().send_embed_notification(
            &settings.notifications.on_new_trade.webhook.clone().unwrap_or("".to_string()),
            vec![json!({
                "title": title,
                "color": 5814783,
                "fields": [
                    {
                        "name": "Player",
                        "value": format!("```{}```", trade.player_name),
                        "inline": true
                    },
                    {
                        "name": "Trade Type",
                        "value": format!("```{}```", trade.trade_type.to_str()),
                        "inline": true
                    },
                    {
                        "name": "Platinum",
                        "value": format!("```{}```", trade.platinum),
                        "inline": true
                    },
                    {
                        "name": "Offered",
                        "value": format!("```{}```", offered_items),
                        "inline": true
                    },
                    {
                        "name": "Received",
                        "value": format!("```{}```", received_items),
                        "inline": true
                    },
                    {
                        "name": "Stock",
                        "value": format!("```{}```", notify_value["stock_operation"].as_str().unwrap_or("None")),
                        "inline": true
                    },
                    {
                        "name": "Warframe Market",
                        "value": format!("```{}```", notify_value["wfm_operation"].as_str().unwrap_or("None")),
                        "inline": true
                    }
                ],
            })],
        );
    }
}
