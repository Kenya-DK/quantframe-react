use crate::app::types::transaction_summary::TransactionSummary;
use entity::{
    sub_type::SubType,
    transaction::transaction::{self},
};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct TransactionItemSummary {
    pub wfm_id: String,
    pub item_name: String,
    pub item_type: String,
    pub sub_type: Option<SubType>,
    pub tags: String,
    pub profit: i64,
    pub profit_margin: f64,
    pub average_price: f64,
    pub total_transactions: usize,
    pub expenses: i64,
    pub revenue: i64,
    pub purchases: usize,
    pub sales: usize,
    pub quantity: i64,
}
impl TransactionItemSummary {
    pub fn new(
        wfm_id: String,
        item_name: String,
        item_type: String,
        sub_type: Option<SubType>,
        tags: String,
        profit: i64,
        profit_margin: f64,
        average_price: f64,
        total_transactions: usize,
        expenses: i64,
        revenue: i64,
        purchases: usize,
        sales: usize,
        quantity: i64,
    ) -> Self {
        Self {
            wfm_id,
            item_name,
            item_type,
            sub_type,
            tags,
            profit,
            profit_margin,
            average_price,
            total_transactions,
            expenses,
            revenue,
            purchases,
            sales,
            quantity,
        }
    }
    pub fn from_transactions(transactions: &Vec<transaction::Model>) -> Vec<Self> {
        use std::collections::HashMap;
        let mut grouped: HashMap<String, Vec<transaction::Model>> = HashMap::new();
        for item in transactions.iter() {
            grouped
                .entry(item.wfm_id.clone())
                .or_insert_with(Vec::new)
                .push(item.clone());
        }
        let mut items_summary: Vec<TransactionItemSummary> = Vec::new();
        for (wfm_id, items) in grouped {
            if items.is_empty() {
                continue;
            }
            let summarize_transactions = TransactionSummary::from(&items);
            let first_item = items.first().unwrap();
            let quantity = items.iter().map(|item| item.quantity).sum::<i64>();
            items_summary.push(TransactionItemSummary::new(
                wfm_id.clone(),
                first_item.item_name.clone(),
                first_item.item_type.to_string(),
                first_item.sub_type.clone(),
                first_item.tags.clone(),
                summarize_transactions.profit,
                summarize_transactions.profit_margin,
                summarize_transactions.profit_margin,
                summarize_transactions.total_transactions,
                summarize_transactions.expenses,
                summarize_transactions.revenue,
                summarize_transactions.purchases,
                summarize_transactions.sales,
                quantity,
            ));
        }
        items_summary
    }
}
