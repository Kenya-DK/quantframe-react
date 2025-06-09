use entity::transaction::transaction::{self, TransactionType};
use serde::Serialize;
#[derive(Serialize, Debug)]
pub struct TransactionSummary {
    pub profit: i64,
    pub profit_margin: f64,
    pub average_profit: f64,
    pub total_transactions: usize,
    pub expenses: i64,
    pub average_expense: f64,
    pub revenue: i64,
    pub average_revenue: f64,
    pub purchases: usize,
    pub sales: usize,
}
impl TransactionSummary {
    pub fn new(
        profit: i64,
        total_transactions: usize,
        expenses: i64,
        revenue: i64,
        purchases: usize,
        sales: usize,
    ) -> Self {
        Self {
            profit: revenue - expenses,
            profit_margin: if revenue != 0 {
                profit as f64 / revenue as f64 * 100.0
            } else {
                0.0
            },
            average_profit: if sales != 0 {
                profit as f64 / sales as f64
            } else {
                0.0
            },
            total_transactions,
            expenses,
            average_expense: if purchases != 0 {
                expenses as f64 / purchases as f64
            } else {
                0.0
            },
            revenue,
            average_revenue: if sales != 0 {
                revenue as f64 / sales as f64
            } else {
                0.0
            },
            purchases,
            sales,
        }
    }
}
impl From<&Vec<transaction::Model>> for TransactionSummary {
    fn from(transactions: &Vec<transaction::Model>) -> Self {
        let expenses = transactions
            .iter()
            .filter(|item| item.transaction_type == TransactionType::Purchase)
            .collect::<Vec<&transaction::Model>>();
        let expenses_total = expenses.iter().map(|item| item.price).sum::<i64>();
        let sales = transactions
            .iter()
            .filter(|item| item.transaction_type == TransactionType::Sale)
            .collect::<Vec<&transaction::Model>>();
        let revenue = sales.iter().map(|item| item.price).sum::<i64>();
        let profit = revenue - expenses_total;
        TransactionSummary::new(
            profit,
            transactions.len(),
            expenses_total,
            revenue,
            expenses.len(),
            sales.len(),
        )
    }
}
