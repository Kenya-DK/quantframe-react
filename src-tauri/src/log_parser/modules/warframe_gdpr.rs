use chrono::{DateTime, Datelike, TimeZone, Utc};
use entity::{
    dto::{FinancialGraphMap, FinancialReport, PaginatedResult, SubType},
    enums::FieldChange,
};
use regex::Regex;
use serde_json::json;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Instant,
};
use utils::{
    fill_missing_date_keys, filters_by, get_start_end_of, group_by, group_by_date, info, Error,
    GroupByDate, LoggerOptions,
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
    pub was_initialized: Mutex<bool>,
    pub trades_years: Mutex<Vec<String>>,
    pub trades: Mutex<Vec<PlayerTrade>>,
    pub logins: Mutex<Vec<Login>>,
    pub purchases: Mutex<Vec<Purchase>>,
    pub transactions: Mutex<Vec<Transaction>>,
}
impl WarframeGDPRModule {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            was_initialized: Mutex::new(false),
            trades_years: Mutex::new(Vec::new()),
            trades: Mutex::new(Vec::new()),
            logins: Mutex::new(Vec::new()),
            purchases: Mutex::new(Vec::new()),
            transactions: Mutex::new(Vec::new()),
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
        let mut current_transaction: Option<Transaction> = None;
        let mut log_section: Option<LogSection> = None;
        let mut section: Option<&str> = None;
        let mut awaiting_purchase_shop_id = false;

        let trades_re = Regex::new(r"^TRADES\s*:\s*(\d+)").unwrap();
        let logins_re = Regex::new(r"^LOGINS\s*:\s*(\d+)").unwrap();
        let purchases_re = Regex::new(r"^PURCHASES\s*:\s*(\d+)").unwrap();
        let date_re = Regex::new(r"^\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2}\s+UTC$").unwrap();
        let item_re = Regex::new(r"^(.+?)(?:\s*:\s*(-?\d+))?$").unwrap();
        let platinum_re = Regex::new(r"^PLATINUM\s*:\s*(\d+)").unwrap();
        let transaction_index_re = Regex::new(r"^(\d+)\s*:\s*$").unwrap();

        let mut trades = self.trades.lock().unwrap();
        let mut years = self.trades_years.lock().unwrap();
        let mut logins = self.logins.lock().unwrap();
        let mut purchases = self.purchases.lock().unwrap();
        let mut transactions = self.transactions.lock().unwrap();
        let mut was_initialized = self.was_initialized.lock().unwrap();
        *was_initialized = true;
        // Start Time
        let start = Instant::now();
        let mut previous_line = String::new();
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
                if previous_line.eq("Stats") {
                    continue;
                }
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

            if line.eq("Transactions") {
                section = Some("transactions");
                transactions.clear();
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
                        let date = to_date(&line);
                        let year_str = date.year().to_string();
                        if !years.contains(&year_str) {
                            years.push(year_str);
                        }
                        current_trade = Some(PlayerTrade::default().set_time(date));
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

                    Some("transactions") => {
                        if let Some(transaction) = current_transaction.take() {
                            transactions.push(transaction);
                        }

                        current_transaction = Some(Transaction {
                            date: to_date(&line),
                            sku: String::new(),
                            price: 0.0,
                            currency: String::new(),
                            vendor: String::new(),
                            account: String::new(),
                        });
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
                        item.set_property_value("tags", cached_item.tags.clone());
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
            /* =======================
               TRANSACTION DETAILS
            ======================= */
            if section == Some("transactions") {
                // Check for transaction index (e.g., "0 :", "1 :")
                if transaction_index_re.is_match(&line) {
                    if let Some(transaction) = current_transaction.take() {
                        transactions.push(transaction);
                    }
                    current_transaction = Some(Transaction {
                        sku: String::new(),
                        price: 0.0,
                        currency: String::new(),
                        vendor: String::new(),
                        date: Utc::now(),
                        account: String::new(),
                    });
                    continue;
                }

                if let Some(transaction) = current_transaction.as_mut() {
                    if line.starts_with("SKU :") {
                        transaction.sku = line.replace("SKU :", "").trim().to_string();
                        continue;
                    }

                    if line.starts_with("PRICE :") {
                        transaction.price =
                            line.replace("PRICE :", "").trim().parse().unwrap_or(0.0);
                        continue;
                    }

                    if line.starts_with("CURRENCY :") {
                        transaction.currency = line.replace("CURRENCY :", "").trim().to_string();
                        continue;
                    }

                    if line.starts_with("VENDOR :") {
                        transaction.vendor = line.replace("VENDOR :", "").trim().to_string();
                        continue;
                    }

                    if line.starts_with("DATE :") {
                        let date_str = line.replace("DATE :", "").trim().to_string();
                        transaction.date = to_date(&format!("{} UTC", date_str));
                        continue;
                    }

                    if line.starts_with("ACCOUNT :") {
                        transaction.account = line.replace("ACCOUNT :", "").trim().to_string();
                        continue;
                    }
                }
            }
            previous_line = line.clone();
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

        if let Some(transaction) = current_transaction {
            transactions.push(transaction);
        }

        info(
            format!("{}:Load", COMPONENT),
            format!(
                "Finished loading Warframe GDPR data in: {:.2?} | Trades: {} | Logins: {} | Purchases: {} | Transactions: {}",
                start.elapsed(),
                trades.len(),
                logins.len(),
                purchases.len(),
                transactions.len()
            ),
            &LoggerOptions::default(),
        );
        notify_gui!(
            "warframe_gdpr_data_loaded",
            "green",
            "success",
            json!({
                "trades": trades.len(),
                "logins": logins.len(),
                "purchases": purchases.len(),
                "transactions": transactions.len(),
            })
        );
        enable_logging(true);
        Ok(())
    }
    /* =======================
        HELPER METHODS
    ======================= */
    fn generate_trade_financial_report(&self, trades: &Vec<PlayerTrade>) -> FinancialReport {
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
            .filter(|item| item.item_type != TradeItemType::Credits)
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
        let report = FinancialReport::new(
            total_transactions,
            sales.len(),
            highest_revenue,
            lowest_revenue,
            revenue,
            purchases.len(),
            highest_expense,
            lowest_expense,
            expenses,
        ).with_properties(json!({
            "total_credits": total_credits,
            "total_trades": trade_list.len(),
            "most_purchased_items": purchase_quantities_by_item.into_iter().take(7).collect::<Vec<(String, i64)>>(),
            "most_sold_items": sale_quantities_by_item.into_iter().take(7).collect::<Vec<(String, i64)>>(),
        }));
        report
    }
    fn generate_trade_financial_graph(
        &self,
        trades: &Vec<PlayerTrade>,
        date: DateTime<Utc>,
        group_by1: GroupByDate,
        group_by2: &[GroupByDate],
    ) -> (FinancialReport, FinancialGraphMap<i64>) {
        let (start, end) = get_start_end_of(date, group_by1);
        let trades = filters_by(trades, |t| t.trade_time >= start && t.trade_time <= end);

        let mut grouped = group_by_date(&trades, |t| t.trade_time, group_by2);

        fill_missing_date_keys(&mut grouped, start, end, group_by2);
        let graph: FinancialGraphMap<i64> = FinancialGraphMap::<i64>::from(&grouped, |group| {
            HashMap::from([
                (
                    "total_purchase",
                    group
                        .iter()
                        .filter(|t| t.trade_type == TradeClassification::Purchase)
                        .count() as i64,
                ),
                (
                    "total_sales",
                    group
                        .iter()
                        .filter(|t| t.trade_type == TradeClassification::Sale)
                        .count() as i64,
                ),
                (
                    "total_trades",
                    group
                        .iter()
                        .filter(|t| t.trade_type == TradeClassification::Trade)
                        .count() as i64,
                ),
            ])
        });
        (self.generate_trade_financial_report(&trades), graph)
    }

    pub fn was_initialized(&self) -> bool {
        let was_initialized = self.was_initialized.lock().unwrap();
        *was_initialized
    }
    pub fn get_trade_years(&self) -> Vec<String> {
        let trades = self.trades.lock().unwrap();
        let mut years = trades
            .iter()
            .map(|t| t.trade_time.year().to_string())
            .collect::<Vec<String>>();
        years.sort();
        years.dedup();
        years
    }
    pub fn trades(&self, query: TradePaginationQueryDto) -> PaginatedResult<PlayerTrade> {
        let trades = self.trades.lock().unwrap().clone();

        let filtered_auctions = query.apply_query(&trades);

        let paginate = paginate(
            &filtered_auctions,
            query.pagination.page,
            query.pagination.limit,
        );
        paginate
    }
    pub fn trade_financial_report(&self, mut query: TradePaginationQueryDto) -> FinancialReport {
        let settings = states::app_state().unwrap().settings;
        query.pagination.limit = -1; // get all trades
        let trades = self.trades(query.clone()).results;

        let mut report = self.generate_trade_financial_report(&trades);

        let year = match query.year {
            FieldChange::Value(y) => y,
            _ => Utc::now().year(),
        };
        let (year_report, year_graph) = self.generate_trade_financial_graph(
            &trades,
            Utc.ymd(year.to_string().parse().unwrap(), 1, 1)
                .and_hms(0, 0, 0),
            GroupByDate::Year,
            &[GroupByDate::Year, GroupByDate::Month],
        );

        let mut items = vec![];
        for category in settings.summary_settings.categories {
            let tags = &category.tags;
            let types = &category.types;
            let filtered_transactions = filters_by(&trades, |t| {
                let items2 = t
                    .offered_items
                    .iter()
                    .chain(t.received_items.iter())
                    .collect::<Vec<_>>();

                let tag_matches = items2
                    .iter()
                    .map(|item| item.get_property_value::<Vec<String>>("tags".to_string(), vec![]))
                    .flatten()
                    .any(|tag| tags.contains(&tag.trim().to_string()));

                let type_matches = items2
                    .iter()
                    .map(|item| item.item_type.to_string())
                    .any(|tag| types.contains(&tag.trim().to_string()));

                tag_matches || type_matches
            });
            let re = self
                .generate_trade_financial_report(&filtered_transactions)
                .with_properties(json!({
                    "icon": category.icon,
                    "name": category.name,
                }));
            items.push(re);
        }

        if let Some(ref mut properties) = report.properties {
            properties["graph"] = json!(year_graph);
            properties["categories"] = json!(items);
            properties["year"] = json!(year_report);
        }

        report
    }
    pub fn logins(&self, query: LoginPaginationQueryDto) -> PaginatedResult<Login> {
        let logins = self.logins.lock().unwrap().clone();
        let filtered_paginate = query.apply_query(&logins);
        let paginate = paginate(
            &filtered_paginate,
            query.pagination.page,
            query.pagination.limit,
        );
        paginate
    }
    pub fn purchases(&self, query: PurchasePaginationQueryDto) -> PaginatedResult<Purchase> {
        let purchases = self.purchases.lock().unwrap().clone();
        let filtered_paginate = query.apply_query(&purchases);
        let paginate = paginate(
            &filtered_paginate,
            query.pagination.page,
            query.pagination.limit,
        );
        paginate
    }
    pub fn transactions(
        &self,
        query: TransactionPaginationQueryDto,
    ) -> PaginatedResult<Transaction> {
        let transactions = self.transactions.lock().unwrap().clone();
        let filtered_paginate = query.apply_query(&transactions);
        let paginate = paginate(
            &filtered_paginate,
            query.pagination.page,
            query.pagination.limit,
        );
        paginate
    }
}
