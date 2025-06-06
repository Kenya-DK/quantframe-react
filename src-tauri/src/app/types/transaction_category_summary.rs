use crate::{app::types::TransactionSummary, settings::SummaryCategorySetting};
use entity::transaction::transaction::{self};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct TransactionCategorySummary {
    icon: String,
    name: String,
    revenue: i64,
    expenses: i64,
    profit: i64,
    profit_margin: f64,
}
impl TransactionCategorySummary {
    pub fn new(
        icon: String,
        name: String,
        revenue: i64,
        expenses: i64,
        profit: i64,
        profit_margin: f64,
    ) -> Self {
        Self {
            icon,
            name,
            revenue,
            expenses,
            profit,
            profit_margin,
        }
    }
    pub fn from_transactions(
        transactions: &Vec<transaction::Model>,
        category: &SummaryCategorySetting,
    ) -> Self {
        let tags = &category.tags;
        let types = &category.types;
        // Filter transactions based on the current category
        let filtered_transactions: Vec<transaction::Model> = transactions
            .iter()
            .filter(|t| {
                let tag_matches = t
                    .tags
                    .split(',')
                    .any(|tag| tags.contains(&tag.trim().to_string()));

                let type_matches = types.contains(&t.item_type.to_string());

                tag_matches || type_matches
            })
            .cloned()
            .collect();

        let summary = TransactionSummary::from(&filtered_transactions);
        TransactionCategorySummary::new(
            category.icon.clone(),
            category.name.clone(),
            summary.revenue,
            summary.expenses,
            summary.profit,
            summary.profit_margin,
        )
    }
}
