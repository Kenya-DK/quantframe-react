use crate::{enums::*, stock_item, stock_riven, transaction::*, wish_list};
use serde::Serialize;
use serde_json::json;
use utils::group_by;

#[derive(Serialize, Debug)]
pub struct FinancialReport {
    // General transaction metrics
    pub total_transactions: usize,
    pub average_transaction: f64,

    // Profit metrics
    pub total_profit: i64,
    pub average_profit: f64,
    pub profit_margin: f64,
    pub roi: f64, // Return on Investment percentage

    // Revenue metrics
    pub sale_count: usize,
    pub highest_revenue: f64,
    pub lowest_revenue: f64,
    pub average_revenue: f64,
    pub revenue: f64,

    // Expense metrics
    pub purchases_count: usize,
    pub highest_expense: f64,
    pub lowest_expense: f64,
    pub average_expense: f64,
    pub expenses: f64,

    // Extra properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl FinancialReport {
    pub fn new(
        total_transactions: usize,
        sale_count: usize,
        highest_revenue: f64,
        lowest_revenue: f64,
        revenue: i64,
        purchases_count: usize,
        highest_expense: f64,
        lowest_expense: f64,
        expenses: i64,
    ) -> Self {
        let total_value = (revenue + expenses) as f64;
        let average_transaction = if total_transactions > 0 {
            total_value / total_transactions as f64
        } else {
            0.0
        };

        let total_profit = revenue - expenses;
        let average_profit = if sale_count > 0 {
            total_profit as f64 / sale_count as f64
        } else {
            0.0
        };

        let profit_margin = if revenue != 0 {
            total_profit as f64 / revenue as f64 * 100.0
        } else {
            0.0
        };

        let average_revenue = if sale_count > 0 {
            revenue as f64 / sale_count as f64
        } else {
            0.0
        };

        let average_expense = if purchases_count > 0 {
            expenses as f64 / purchases_count as f64
        } else {
            0.0
        };

        // ROI calculation: (Revenue - Expenses) / Expenses * 100
        // Returns the percentage return on investment
        let roi = if expenses != 0 {
            (total_profit as f64 / expenses as f64) * 100.0
        } else {
            0.0
        };

        Self {
            total_transactions,
            average_transaction,
            total_profit,
            average_profit,
            profit_margin,
            roi,
            highest_revenue,
            lowest_revenue,
            sale_count,
            revenue: revenue as f64,
            average_revenue,
            highest_expense,
            lowest_expense,
            purchases_count,
            expenses: expenses as f64,
            average_expense,
            properties: None,
        }
    }
    pub fn with_properties(mut self, properties: serde_json::Value) -> Self {
        self.properties = Some(properties);
        self
    }
}

impl Default for FinancialReport {
    fn default() -> Self {
        Self {
            total_transactions: 0,
            average_transaction: 0.0,
            total_profit: 0,
            average_profit: 0.0,
            profit_margin: 0.0,
            roi: 0.0,
            highest_revenue: 0.0,
            lowest_revenue: 0.0,
            sale_count: 0,
            revenue: 0.0,
            average_revenue: 0.0,
            highest_expense: 0.0,
            lowest_expense: 0.0,
            purchases_count: 0,
            expenses: 0.0,
            average_expense: 0.0,
            properties: None,
        }
    }
}

impl From<&Vec<transaction::Model>> for FinancialReport {
    fn from(transactions: &Vec<transaction::Model>) -> Self {
        let total_transactions = transactions.len();

        let purchases: Vec<&transaction::Model> = transactions
            .iter()
            .filter(|t| t.transaction_type == TransactionType::Purchase)
            .collect();
        let mut purchase_quantities_by_item = group_by(&purchases, |item| item.item_name.clone())
            .iter()
            .map(|(name, items)| (name.clone(), items.iter().map(|i| i.quantity).sum()))
            .collect::<Vec<(String, i64)>>();
        purchase_quantities_by_item.sort_by(|a, b| b.1.cmp(&a.1));
        let expenses: i64 = purchases.iter().map(|t| t.price).sum();
        let highest_expense = purchases.iter().map(|t| t.price).max().unwrap_or(0) as f64;
        let lowest_expense = purchases.iter().map(|t| t.price).min().unwrap_or(0) as f64;

        let sales: Vec<&transaction::Model> = transactions
            .iter()
            .filter(|t| t.transaction_type == TransactionType::Sale)
            .collect();
        let mut sale_quantities_by_item = group_by(&sales, |item| item.item_name.clone())
            .iter()
            .map(|(name, items)| (name.clone(), items.iter().map(|i| i.quantity).sum()))
            .collect::<Vec<(String, i64)>>();
        sale_quantities_by_item.sort_by(|a, b| b.1.cmp(&a.1));
        let revenue: i64 = sales.iter().map(|t| t.price).sum();
        let highest_revenue = sales.iter().map(|t| t.price).max().unwrap_or(0) as f64;
        let lowest_revenue = sales.iter().map(|t| t.price).min().unwrap_or(0) as f64;

        let total_credits: i64 = transactions.iter().map(|t| t.credits).sum();

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
        )
        .with_properties(json!({
            "total_credits": total_credits,
            "most_purchased_items": purchase_quantities_by_item.into_iter().take(5).collect::<Vec<(String, i64)>>(),
            "most_sold_items": sale_quantities_by_item.into_iter().take(5).collect::<Vec<(String, i64)>>(),
        }));
        report
    }
}

impl From<&Vec<stock_item::Model>> for FinancialReport {
    fn from(items: &Vec<stock_item::Model>) -> Self {
        let total_transactions = items.len();

        let purchases: Vec<&stock_item::Model> =
            items.iter().filter(|item| item.bought > 0).collect();
        let expenses: i64 = purchases.iter().map(|item| item.bought * item.owned).sum();

        let sales: Vec<&stock_item::Model> = items
            .iter()
            .filter(|item| item.list_price.unwrap_or(0) > 0)
            .collect();
        let highest_revenue = sales
            .iter()
            .map(|item| item.list_price.unwrap_or(0) * item.owned)
            .max()
            .unwrap_or(0) as f64;
        let lowest_revenue = sales
            .iter()
            .map(|item| item.list_price.unwrap_or(0) * item.owned)
            .min()
            .unwrap_or(0) as f64;
        let revenue: i64 = sales
            .iter()
            .map(|item| item.list_price.unwrap_or(0) * item.owned)
            .sum();
        let lowest_expense = purchases
            .iter()
            .map(|item| item.bought * item.owned)
            .min()
            .unwrap_or(0) as f64;
        let highest_expense = purchases
            .iter()
            .map(|item| item.bought * item.owned)
            .max()
            .unwrap_or(0) as f64;

        FinancialReport::new(
            total_transactions,
            sales.len(),
            highest_revenue,
            lowest_revenue,
            revenue,
            purchases.len(),
            highest_expense,
            lowest_expense,
            expenses,
        )
    }
}

impl From<&Vec<stock_riven::Model>> for FinancialReport {
    fn from(items: &Vec<stock_riven::Model>) -> Self {
        let total_transactions = items.len();

        let purchases: Vec<&stock_riven::Model> =
            items.iter().filter(|item| item.bought > 0).collect();
        let expenses: i64 = purchases.iter().map(|item| item.bought).sum();
        let highest_expense = purchases.iter().map(|item| item.bought).max().unwrap_or(0) as f64;
        let lowest_expense = purchases.iter().map(|item| item.bought).min().unwrap_or(0) as f64;

        let sales: Vec<&stock_riven::Model> = items
            .iter()
            .filter(|item| item.list_price.unwrap_or(0) > 0)
            .collect();
        let revenue: i64 = sales.iter().map(|item| item.list_price.unwrap_or(0)).sum();
        let highest_revenue = sales
            .iter()
            .map(|item| item.list_price.unwrap_or(0))
            .max()
            .unwrap_or(0) as f64;
        let lowest_revenue = sales
            .iter()
            .map(|item| item.list_price.unwrap_or(0))
            .min()
            .unwrap_or(0) as f64;
        FinancialReport::new(
            total_transactions,
            sales.len(),
            highest_revenue,
            lowest_revenue,
            revenue,
            purchases.len(),
            highest_expense,
            lowest_expense,
            expenses,
        )
    }
}

impl From<&Vec<wish_list::wish_list::Model>> for FinancialReport {
    fn from(items: &Vec<wish_list::wish_list::Model>) -> Self {
        let total_transactions = items.len();

        let purchases: Vec<&wish_list::wish_list::Model> = items
            .iter()
            .filter(|item| item.list_price.unwrap_or(0) > 0)
            .collect();
        let expenses: i64 = purchases
            .iter()
            .map(|item| item.list_price.unwrap_or(0))
            .sum();
        let highest_expense = purchases
            .iter()
            .map(|item| item.list_price.unwrap_or(0))
            .max()
            .unwrap_or(0) as f64;
        let lowest_expense = purchases
            .iter()
            .map(|item| item.list_price.unwrap_or(0))
            .min()
            .unwrap_or(0) as f64;

        let sales: Vec<&wish_list::wish_list::Model> = items
            .iter()
            .filter(|item| item.list_price.unwrap_or(0) > 0)
            .collect();
        let revenue: i64 = sales.iter().map(|item| item.list_price.unwrap_or(0)).sum();
        let highest_revenue = sales
            .iter()
            .map(|item| item.list_price.unwrap_or(0))
            .max()
            .unwrap_or(0) as f64;
        let lowest_revenue = sales
            .iter()
            .map(|item| item.list_price.unwrap_or(0))
            .min()
            .unwrap_or(0) as f64;

        FinancialReport::new(
            total_transactions,
            sales.len(),
            highest_revenue,
            lowest_revenue,
            revenue,
            purchases.len(),
            highest_expense,
            lowest_expense,
            expenses,
        )
    }
}
