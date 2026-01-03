use chrono::{DateTime, Utc};
use entity::dto::{FinancialGraph, FinancialGraphMap, FinancialReport, PaginatedResult, SubType};
use regex::Regex;
use serde_json::json;
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Instant,
};
use utils::{
    get_location, group_by, group_by_date, info, read_json_file_optional, Error, GroupByDate,
    LoggerOptions,
};

use crate::{
    enums::LogSection,
    helper::paginate,
    log_parser::{enable_logging, types::*, TradeClassification, TradeItemType},
    utils::modules::states,
};
fn to_date(text: &str) -> DateTime<Utc> {
    match text.parse::<DateTime<Utc>>() {
        Ok(dt) => dt,
        Err(e) => {
            println!("Failed to parse date line '{}': {}", text, e);
            Utc::now()
        }
    }
}
static COMPONENT: &str = "WarframeGDPRModule";
#[derive(Debug)]
pub struct WarframeGDPRModule {
    pub trades: Mutex<Vec<PlayerTrade>>,
    pub logins: Mutex<Vec<Login>>,
    pub purchases: Mutex<Vec<Purchase>>,
}
impl WarframeGDPRModule {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            trades: Mutex::new(Vec::new()),
            logins: Mutex::new(Vec::new()),
            purchases: Mutex::new(Vec::new()),
        })
    }

    pub fn load(&self, file_path: &str) -> Result<(), Error> {
        enable_logging(false);
        let cache = states::cache_client()?;
        // Read the file content
        let content = std::fs::read_to_string(&file_path)?;
        let lines: Vec<String> = content.lines().map(|l| l.trim().to_string()).collect();
        info(
            format!("{}:Load", COMPONENT),
            format!(
                "Starting to load Warframe GDPR data from: {} lines",
                lines.len()
            ),
            &LoggerOptions::default(),
        );

        let detections = DETECTIONS.get().unwrap();
        let detection = detections.get("en").unwrap();
        let mut current_trade: Option<PlayerTrade> = None;
        let mut current_login: Option<Login> = None;
        let mut current_purchase: Option<Purchase> = None;
        let mut log_section: Option<LogSection> = None;
        let mut section: Option<&str> = None;
        let mut awaiting_purchase_shop_id = false;

        let trades_re = Regex::new(r"^TRADES\s*:\s*(\d+)").unwrap();
        let logins_re = Regex::new(r"^LOGINS\s*:\s*(\d+)").unwrap();
        let purchases_re = Regex::new(r"^PURCHASES\s*:\s*(\d+)").unwrap();
        let date_re = Regex::new(r"^\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2}\s+UTC$").unwrap();
        let item_re = Regex::new(r"^(.+?)(?:\s*:\s*(-?\d+))?$").unwrap();
        let platinum_re = Regex::new(r"^PLATINUM\s*:\s*(\d+)").unwrap();

        let mut trades = self.trades.lock().unwrap();
        let mut logins = self.logins.lock().unwrap();
        let mut purchases = self.purchases.lock().unwrap();

        // Start Time
        let start = Instant::now();
        for line in lines {
            /* =======================
               METADATA
            ======================= */
            if let Some(_) = trades_re.captures(&line) {
                // result.metadata.trades = caps[1].parse().unwrap_or(0);
                section = Some("trades");
                trades.clear();
                continue;
            }

            if let Some(_) = logins_re.captures(&line) {
                // result.metadata.logins = caps[1].parse().unwrap_or(0);
                section = Some("logins");
                logins.clear();
                continue;
            }

            if let Some(_) = purchases_re.captures(&line) {
                // result.metadata.purchases = caps[1].parse().unwrap_or(0);
                section = Some("purchases");
                purchases.clear();
                continue;
            }

            /* =======================
               DATE LINE
            ======================= */
            if date_re.is_match(&line) {
                match section {
                    Some("trades") => {
                        if let Some(mut trade) = current_trade.take() {
                            trade.calculate();
                            trade.calculate_items();
                            trades.push(trade);
                        }
                        current_trade = Some(PlayerTrade::default().set_time(to_date(&line)));
                    }

                    Some("logins") => {
                        if let Some(login) = current_login.take() {
                            logins.push(login);
                        }

                        current_login = Some(Login {
                            date: to_date(&line),
                            ip: None,
                            client_type: None,
                        });
                    }

                    Some("purchases") => {
                        if let Some(purchase) = current_purchase.take() {
                            purchases.push(purchase);
                        }

                        current_purchase = Some(Purchase {
                            date: to_date(&line),
                            shop_id: String::new(),
                            price: 0,
                            items_received: vec![],
                        });

                        awaiting_purchase_shop_id = true;
                        log_section = None;
                    }
                    _ => {}
                }
                continue;
            }

            /* =======================
               TRADE DETAILS
            ======================= */
            if section == Some("trades") {
                match line.as_str() {
                    "TRADED ITEMS GIVEN :" => {
                        log_section = Some(LogSection::Offered);
                        continue;
                    }
                    "TRADED ITEMS RECIEVED :" => {
                        log_section = Some(LogSection::Received);
                        continue;
                    }
                    _ if line.is_empty() && matches!(log_section, Some(LogSection::Received)) => {
                        log_section = None;
                        continue;
                    }
                    _ => {}
                }

                let (trade, sec) = match (current_trade.as_mut(), &log_section) {
                    (Some(t), Some(s)) => (t, s),
                    _ => continue,
                };

                let caps = match item_re.captures(&line) {
                    Some(c) => c,
                    None => continue,
                };

                let mut raw = caps.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
                let quantity = caps
                    .get(2)
                    .and_then(|m| m.as_str().parse::<i64>().ok())
                    .unwrap_or(1)
                    .abs();

                /* ---------- Normalize Raw Name ---------- */

                match raw.as_str() {
                    "LEGENDARY CORE" => {
                        raw = "Legendary Core (LEGENDARY RANK 0)".to_string();
                    }
                    _ if raw.contains("RIVEN MOD") => {
                        raw.push_str(" (RIVEN RANK 0)");
                    }
                    _ if raw.ends_with("PLATINUM") => {
                        raw = "Platinum".to_string();
                        trade.platinum = quantity;
                    }
                    _ if raw.ends_with("CREDITS") => {
                        raw = "Credits".to_string();
                        trade.credits = quantity;
                    }
                    _ => {}
                }

                if quantity > 1 {
                    raw = format!("{raw} x {quantity}");
                }

                /* ---------- Create + Validate Item ---------- */

                let (_, mut item) = TradeItem::from_string(&raw, "", &detection);

                if item.item_type == TradeItemType::Unknown {
                    let validations = [
                        item.raw.clone(),
                        format!("{} Blueprint", item.raw),
                        item.raw.replace(' ', "_").to_lowercase(),
                    ];

                    for attempt in validations {
                        item.raw = attempt;
                        match item.validate("") {
                            Ok(status) => {
                                if status.is_found() {
                                    break;
                                }
                            }
                            Err(e) => {
                                println!("Validation error: {}", e);
                            }
                        }
                    }

                    if item.item_type == TradeItemType::Unknown {
                        println!("Item not found in cache: {}", item.raw);
                    }
                }

                /* ---------- Fix known mod-rank error ---------- */

                if let Some((err, _)) = &item.error {
                    if err.contains("Mod Rank not found") {
                        item.sub_type = Some(SubType::rank(0));
                        item.error = None;
                    }
                }

                /* ---------- Push results ---------- */
                match cache.tradable_item().get_by(&item.unique_name) {
                    Ok(cached_item) => {
                        item.set_property_value("item_name", cached_item.name.clone());
                    }
                    Err(_) => {}
                }
                /* ---------- Push results ---------- */

                match sec {
                    LogSection::Offered => trade.offered_items.push(item.clone()),
                    LogSection::Received => trade.received_items.push(item.clone()),
                    _ => {}
                }
            }

            /* =======================
               LOGIN DETAILS
            ======================= */
            if section == Some("logins") {
                if let Some(login) = current_login.as_mut() {
                    if line.starts_with("IP :") {
                        login.ip = Some(line.replace("IP :", "").trim().to_string());
                        continue;
                    }

                    if line.starts_with("CLIENT TYPE :") {
                        login.client_type =
                            Some(line.replace("CLIENT TYPE :", "").trim().to_string());
                        continue;
                    }
                }
            }
            /* =======================
               PURCHASE DETAILS
            ======================= */
            if section == Some("purchases") {
                if let Some(purchase) = current_purchase.as_mut() {
                    // First non-empty line after date → shop_id
                    if awaiting_purchase_shop_id && !line.is_empty() {
                        purchase.shop_id = line.clone();
                        awaiting_purchase_shop_id = false;
                        continue;
                    }

                    // Platinum spent
                    if let Some(caps) = platinum_re.captures(&line) {
                        purchase.price = caps[1].parse().unwrap_or(0);
                        continue;
                    }

                    // Items received section
                    if line == "ITEMS RECIEVED :" {
                        log_section = Some(LogSection::Items);
                        continue;
                    }

                    // Parse item lines
                    if matches!(log_section, Some(LogSection::Items)) {
                        if line.is_empty() {
                            log_section = None;
                            continue;
                        }

                        if let Some(caps) = item_re.captures(&line) {
                            let name = caps
                                .get(1)
                                .map(|m| m.as_str().trim().to_string())
                                .unwrap_or_default();

                            let qty = caps
                                .get(2)
                                .and_then(|m| m.as_str().parse::<i64>().ok())
                                .unwrap_or(1);

                            purchase.items_received.push((name, qty));
                        }
                    }
                }
            }
        }
        if let Some(mut trade) = current_trade {
            trade.calculate();
            trade.calculate_items();
            trades.push(trade);
        }

        if let Some(login) = current_login {
            logins.push(login);
        }

        if let Some(purchase) = current_purchase {
            purchases.push(purchase);
        }

        info(
            format!("{}:Load", COMPONENT),
            format!(
                "Finished loading Warframe GDPR data in: {:.2?} | Trades: {} | Logins: {} | Purchases: {}",
                start.elapsed(),
                trades.len(),
                logins.len(),
                purchases.len()
            ),
            &LoggerOptions::default(),
        );
        enable_logging(true);
        Ok(())
    }

    pub fn trades(&self, query: TradePaginationQueryDto) -> PaginatedResult<PlayerTrade> {
        let trades = self.trades.lock().unwrap().clone();
        let paginate = paginate(&trades, query.pagination.page, query.pagination.limit);
        paginate
    }
    pub fn trade_financial_report(&self, mut query: TradePaginationQueryDto) -> FinancialReport {
        query.pagination.limit = -1; // get all trades
        let trades = self.trades(query).results;

        let total_transactions = trades.len();

        let purchases: Vec<&PlayerTrade> = trades
            .iter()
            .filter(|t| t.trade_type == TradeClassification::Purchase)
            .collect();
        let purchase_items = purchases
            .iter()
            .flat_map(|t| t.received_items.iter())
            .collect::<Vec<&TradeItem>>();
        let expenses: i64 = purchases.iter().map(|t| t.platinum).sum();
        let highest_expense = purchases.iter().map(|t| t.platinum).max().unwrap_or(0) as f64;
        let lowest_expense = purchases.iter().map(|t| t.platinum).min().unwrap_or(0) as f64;
        let mut purchase_quantities_by_item = group_by(&purchase_items, |item| {
            item.get_property_value("item_name".to_string(), item.raw.clone())
                .clone()
        })
        .iter()
        .map(|(name, items)| (name.clone(), items.iter().map(|i| i.quantity).sum()))
        .collect::<Vec<(String, i64)>>();
        purchase_quantities_by_item.sort_by(|a, b| b.1.cmp(&a.1));

        let sales: Vec<&PlayerTrade> = trades
            .iter()
            .filter(|t| t.trade_type == TradeClassification::Sale)
            .collect();
        let sale_items = sales
            .iter()
            .flat_map(|t| t.offered_items.iter())
            .collect::<Vec<&TradeItem>>();
        let mut sale_quantities_by_item = group_by(&sale_items, |item| {
            item.get_property_value("item_name".to_string(), item.raw.clone())
                .clone()
        })
        .iter()
        .map(|(name, items)| (name.clone(), items.iter().map(|i| i.quantity).sum()))
        .collect::<Vec<(String, i64)>>();

        let trade_list: Vec<&PlayerTrade> = trades
            .iter()
            .filter(|t| t.trade_type == TradeClassification::Trade)
            .collect();
        sale_quantities_by_item.sort_by(|a, b| b.1.cmp(&a.1));

        let revenue: i64 = sales.iter().map(|t| t.platinum).sum();
        let highest_revenue = sales.iter().map(|t| t.platinum).max().unwrap_or(0) as f64;
        let lowest_revenue = sales.iter().map(|t| t.platinum).min().unwrap_or(0) as f64;

        let total_credits: i64 = trades.iter().map(|t| t.credits).sum();

        let mut fi = FinancialReport::new(
            total_transactions,
            sales.len(),
            highest_revenue,
            lowest_revenue,
            revenue,
            purchases.len(),
            highest_expense,
            lowest_expense,
            expenses,
        );

        let sdf = group_by_date(&trades, |t| t.trade_time, &[GroupByDate::Year]);
        let graph_payload = FinancialGraphMap::<i64>::from(&sdf, |group| {
            HashMap::from([
                ("total", group.len() as i64),
                (
                    "total_sales",
                    group
                        .iter()
                        .filter(|t| t.trade_type == TradeClassification::Sale)
                        .collect::<Vec<_>>()
                        .len() as i64,
                ),
                (
                    "total_purchases",
                    group
                        .iter()
                        .filter(|t| t.trade_type == TradeClassification::Purchase)
                        .collect::<Vec<_>>()
                        .len() as i64,
                ),
                (
                    "total_trades",
                    group
                        .iter()
                        .filter(|t| t.trade_type == TradeClassification::Trade)
                        .collect::<Vec<_>>()
                        .len() as i64,
                ),
            ])
        });
        fi.properties = Some(json!({
           "total_credits": total_credits,
           "total_trades": trades
            .iter()
            .filter(|t| t.trade_type == TradeClassification::Trade)
            .collect::<Vec<_>>().len(),
           "most_purchased_items": purchase_quantities_by_item.into_iter().take(5).collect::<Vec<(String, i64)>>(),
           "most_sold_items": sale_quantities_by_item.into_iter().take(5).collect::<Vec<(String, i64)>>(),
           "financial_graph": graph_payload,
        }));

        fi
    }
    pub fn logins(&self, query: LoginPaginationQueryDto) -> PaginatedResult<Login> {
        let logins = self.logins.lock().unwrap().clone();
        let paginate = paginate(&logins, query.pagination.page, query.pagination.limit);
        paginate
    }
    pub fn purchases(&self, query: PurchasePaginationQueryDto) -> PaginatedResult<Purchase> {
        let purchases = self.purchases.lock().unwrap().clone();
        let paginate = paginate(&purchases, query.pagination.page, query.pagination.limit);
        paginate
    }
}
