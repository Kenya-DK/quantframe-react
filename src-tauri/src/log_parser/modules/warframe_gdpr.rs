use chrono::{Datelike, NaiveDateTime, TimeZone, Utc};
use entity::{
    dto::{FinancialReport, PaginatedResult, SubType},
    enums::FieldChange,
};
use regex::Regex;
use serde_json::json;
use std::{
    sync::{Arc, Mutex},
    time::Instant,
};
use utils::*;

use crate::{
    enums::{LogSection, TradeItemType},
    helper::paginate,
    log_parser::*,
    notify_gui,
    utils::modules::states,
};

static COMPONENT: &str = "WarframeGDPRModule";
#[derive(Debug)]
pub struct WarframeGDPRModule {
    pub was_initialized: Mutex<bool>,
    pub accounts: Mutex<Vec<Account>>,
}
impl WarframeGDPRModule {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            was_initialized: Mutex::new(false),
            accounts: Mutex::new(Vec::new()),
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
        let mut current_account: Option<Account> = None;
        let mut log_section: Option<LogSection> = None;
        let mut section: Option<&str> = None;
        let mut awaiting_purchase_shop_id = false;
        let mut awaiting_account_ips = false;

        let trades_re = Regex::new(r"^TRADES\s*:\s*(\d+)").unwrap();
        let logins_re = Regex::new(r"^LOGINS\s*:\s*(\d+)").unwrap();
        let purchases_re = Regex::new(r"^PURCHASES\s*:\s*(\d+)").unwrap();
        let transactions_re = Regex::new(r"^TRANSACTIONS").unwrap();
        let date_re = Regex::new(r"^\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2}\s+UTC$").unwrap();
        let item_re = Regex::new(r"^(.+?)(?:\s*:\s*(-?\d+))?$").unwrap();
        let platinum_re = Regex::new(r"^PREMIUM CREDITS\s*:\s*(\d+)").unwrap();
        let transaction_index_re = Regex::new(r"^(\d+)\s*:\s*$").unwrap();

        let mut accounts = self.accounts.lock().unwrap();
        accounts.clear();
        let mut was_initialized = self.was_initialized.lock().unwrap();
        *was_initialized = true;
        // Start Time
        let start = Instant::now();
        let mut previous_line = String::new();
        for line in lines {
            /* =======================
               METADATA
            ======================= */
            if line.eq("ACCOUNT") {
                if let Some(acc) = current_account.take() {
                    accounts.push(acc);
                }
                section = Some("account");
                current_account = Some(Account::default());
                awaiting_account_ips = false;
                continue;
            }

            // Separator line (e.g. "------...") — flush the current account
            if section == Some("account") && !line.is_empty() && line.chars().all(|c| c == '-') {
                if let Some(acc) = current_account.take() {
                    accounts.push(acc);
                }
                section = None;
                awaiting_account_ips = false;
                continue;
            }

            if let Some(_) = trades_re.captures(&line) {
                section = Some("trades");
                if let Some(acc) = current_account.as_mut() {
                    acc.trades.clear();
                }
                continue;
            }

            if let Some(_) = logins_re.captures(&line) {
                if previous_line.eq("Stats") {
                    continue;
                }
                section = Some("logins");
                if let Some(acc) = current_account.as_mut() {
                    acc.logins.clear();
                }
                continue;
            }

            if let Some(_) = purchases_re.captures(&line) {
                section = Some("purchases");
                if let Some(acc) = current_account.as_mut() {
                    acc.purchases.clear();
                }
                continue;
            }

            if let Some(_) = transactions_re.captures(&line) {
                section = Some("transactions");
                if let Some(acc) = current_account.as_mut() {
                    acc.transactions.clear();
                }
                continue;
            }

            /* =======================
               ACCOUNT DETAILS
            ======================= */
            if section == Some("account") {
                if let Some(acc) = current_account.as_mut() {
                    if line.starts_with("OID :") {
                        acc.oid = line.replace("OID :", "").trim().to_string();
                        continue;
                    }
                    if line.starts_with("EMAIL :") {
                        acc.email = line.replace("EMAIL :", "").trim().to_string();
                        continue;
                    }
                    if line.starts_with("DISPLAY NAME :") {
                        acc.display_name = line.replace("DISPLAY NAME :", "").trim().to_string();
                        continue;
                    }
                    if line.starts_with("ACTIVATED :") {
                        acc.activated = line.replace("ACTIVATED :", "").trim() == "1";
                        continue;
                    }
                    if line.starts_with("SUBSCRIBED TO EMAILS :") {
                        acc.subscribed_to_emails =
                            line.replace("SUBSCRIBED TO EMAILS :", "").trim() == "1";
                        continue;
                    }
                    if line.starts_with("SIGNUP LANGUAGE :") {
                        acc.signup_language =
                            line.replace("SIGNUP LANGUAGE :", "").trim().to_string();
                        continue;
                    }
                    if line.starts_with("SIGNUP COUNTRY CODE :") {
                        acc.signup_country_code =
                            line.replace("SIGNUP COUNTRY CODE :", "").trim().to_string();
                        continue;
                    }
                    if line.starts_with("COUNTRY CODE :") {
                        acc.country_code = line.replace("COUNTRY CODE :", "").trim().to_string();
                        continue;
                    }
                    if line.starts_with("SIGNUP PAGE :") {
                        acc.signup_page = line.replace("SIGNUP PAGE :", "").trim().to_string();
                        continue;
                    }
                    if line.starts_with("IP :") {
                        let inline = line.replace("IP :", "").trim().to_string();
                        if !inline.is_empty() {
                            acc.ips.push(inline);
                        }
                        awaiting_account_ips = true;
                        continue;
                    }
                    if awaiting_account_ips {
                        if line.is_empty() {
                            awaiting_account_ips = false;
                        } else {
                            acc.ips.push(line.clone());
                        }
                        continue;
                    }
                    if line.starts_with("ACCOUNT CREATION DATE :") {
                        let date_str = line
                            .replace("ACCOUNT CREATION DATE :", "")
                            .trim()
                            .to_string();
                        if let Ok(ndt) =
                            NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S")
                        {
                            acc.account_creation_date = Some(ndt.and_utc());
                        }
                        continue;
                    }
                    if line.starts_with("LAST LOGIN DATE :") {
                        let date_str = line.replace("LAST LOGIN DATE :", "").trim().to_string();
                        if let Ok(ndt) =
                            NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S")
                        {
                            acc.last_login_date = Some(ndt.and_utc());
                        }
                        continue;
                    }
                    if line.starts_with("LANGUAGE :") {
                        acc.language = line.replace("LANGUAGE :", "").trim().to_string();
                        continue;
                    }
                }
            }
            /* =======================
               DATE LINE
            ======================= */
            if date_re.is_match(&line) {
                match section {
                    Some("trades") => {
                        if let Some(acc) = current_account.as_mut() {
                            if let Some(trade) = current_trade.take() {
                                acc.add_trade(trade);
                            }
                            current_trade = Some(PlayerTrade::default().set_time(to_date(&line)));
                        }
                    }

                    Some("logins") => {
                        if let Some(acc) = current_account.as_mut() {
                            if let Some(login) = current_login.take() {
                                acc.logins.push(login);
                            }
                            current_login = Some(Login {
                                date: to_date(&line),
                                ip: None,
                                client_type: None,
                            });
                        }
                    }

                    Some("purchases") => {
                        if let Some(acc) = current_account.as_mut() {
                            if let Some(purchase) = current_purchase.take() {
                                acc.purchases.push(purchase);
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
                    }

                    Some("transactions") => {
                        if let Some(acc) = current_account.as_mut() {
                            if let Some(transaction) = current_transaction.take() {
                                acc.transactions.push(transaction);
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
                    "LEGENDARY CORE" | "Legendary Core" => {
                        raw = "Legendary Core (LEGENDARY RANK 0)".to_string();
                    }
                    _ if raw.contains("RIVEN MOD") || raw.contains("Riven Mod") => {
                        raw.push_str(" (RIVEN RANK 0)");
                    }
                    _ if raw.ends_with("PLATINUM") || raw.ends_with("PREMIUM CREDITS") => {
                        raw = detection.platinum_name.clone();
                        trade.platinum = quantity;
                    }
                    _ if raw.ends_with("CREDITS") || raw.ends_with("REGULAR CREDITS") => {
                        raw = detection.credits_name.clone();
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
                        item.properties.set_property_value("item_name", &item.raw);
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
                        item.properties
                            .set_property_value("item_name", cached_item.name.clone());
                        item.properties
                            .set_property_value("tags", cached_item.tags.clone());
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

                            let mut qty = caps
                                .get(2)
                                .and_then(|m| m.as_str().parse::<i64>().ok())
                                .unwrap_or(1);

                            // If the name Contains "Booster", it's likely a "Booster Pack" where th quantity indicates the duration in seconds Edit the name to it wil be .. Booster 3 Days
                            let name = if name.contains("Booster") && qty > 1 {
                                let days = qty / 86400; // Convert seconds to days
                                qty = 1; // Set quantity to 1 since it's a single booster pack
                                format!("{} {} Days", name, days)
                            } else {
                                name
                            };

                            purchase.items_received.push(PurchaseItem::new(name, qty));
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
                    if let Some(acc) = current_account.as_mut() {
                        if let Some(transaction) = current_transaction.take() {
                            acc.transactions.push(transaction);
                        }
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
                        transaction.date = to_date(&date_str);
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
        if let Some(acc) = current_account.as_mut() {
            if let Some(trade) = current_trade.take() {
                acc.add_trade(trade);
            }
            if let Some(login) = current_login.take() {
                acc.logins.push(login);
            }
            if let Some(purchase) = current_purchase.take() {
                acc.purchases.push(purchase);
            }
            if let Some(transaction) = current_transaction.take() {
                acc.transactions.push(transaction);
            }
        }
        if let Some(acc) = current_account {
            accounts.push(acc);
        }

        let total_trades: usize = accounts.iter().map(|a| a.trades.len()).sum();
        let total_logins: usize = accounts.iter().map(|a| a.logins.len()).sum();
        let total_purchases: usize = accounts.iter().map(|a| a.purchases.len()).sum();
        let total_transactions: usize = accounts.iter().map(|a| a.transactions.len()).sum();
        info(
            format!("{}:Load", COMPONENT),
            format!(
                "Finished loading Warframe GDPR data in: {:.2?} | Accounts: {} | Trades: {} | Logins: {} | Purchases: {} | Transactions: {}",
                start.elapsed(),
                accounts.len(),
                total_trades,
                total_logins,
                total_purchases,
                total_transactions,
            ),
            &LoggerOptions::default(),
        );
        notify_gui!(
            "warframe_gdpr_data_loaded",
            "green",
            "success",
            json!({
                "accounts": accounts.len(),
                "trades": total_trades,
                "logins": total_logins,
                "purchases": total_purchases,
                "transactions": total_transactions,
            })
        );
        enable_logging(true);
        Ok(())
    }

    pub fn was_initialized(&self) -> bool {
        let was_initialized = self.was_initialized.lock().unwrap();
        *was_initialized
    }
    pub fn accounts(&self) -> Vec<Account> {
        self.accounts.lock().unwrap().clone()
    }
}
