use std::collections::HashMap;

use chrono::{DateTime, Utc};
use entity::dto::*;
use serde_json::json;
use utils::*;

use crate::log_parser::*;

/* =======================
    HELPER METHODS
======================= */
pub fn to_date(text: &str) -> DateTime<Utc> {
    match text.parse::<DateTime<Utc>>() {
        Ok(dt) => dt,
        Err(e) => {
            println!("Failed to parse date line '{}': {}", text, e);
            Utc::now()
        }
    }
}
/* =======================
    TRADE METHODS
======================= */
pub fn generate_trade_financial_report(trades: &Vec<PlayerTrade>) -> FinancialReport {
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
pub fn generate_trade_financial_graph(
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
    (generate_trade_financial_report(&trades), graph)
}

/* =======================
    LOGIN METHODS
======================= */

/* =======================
   PURCHASE METHODS
======================= */

/* =======================
    TRANSACTION METHODS
======================= */

/* =======================
    HELPER METHODS
======================= */
