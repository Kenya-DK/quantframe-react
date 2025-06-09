use std::sync::{Arc, Mutex};

use crate::{
    app::types::{TransactionCategorySummary, TransactionItemSummary, TransactionSummary},
    helper::{get_end_of, get_start_of, group_by_date, GroupBy},
    settings::{SettingsState, SummaryCategorySetting},
    utils::modules::error::{self, AppError},
    DATABASE,
};
use chrono::{DateTime, Datelike, Utc};
use entity::{
    dto::sort::SortDirection,
    transaction::transaction::{self},
};
use serde_json::{json, Value};
use service::TransactionQuery;

// Helper: Generate labels and values based on groupings
fn generate_label_value_summary(
    transactions: &[transaction::Model],
    group_by: Vec<GroupBy>,
    format: &dyn Fn(usize) -> String,
    range: usize,
) -> (Vec<String>, Vec<i64>) {
    let grouped = group_by_date(transactions, |item| item.created_at, group_by);

    let labels: Vec<String> = (0..range).map(format).collect();
    let mut values = vec![0; labels.len()];
    for i in 0..labels.len() {
        if let Some(items) = grouped.get(&labels[i]) {
            let summary = TransactionSummary::from(items);
            values[i] = summary.profit;
        }
    }
    (labels, values)
}

//Get transactions between two dates
pub fn get_transactions_between(
    transactions: &[transaction::Model],
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> Vec<transaction::Model> {
    let filtered_transactions: Vec<transaction::Model> = transactions
        .iter()
        .filter(|t| t.created_at >= from && t.created_at <= to)
        .cloned()
        .collect();
    filtered_transactions
}

pub async fn get_summary_from_year(
    transactions: &[transaction::Model],
    year: i32,
) -> Result<Value, AppError> {
    let from_date = get_start_of(GroupBy::Year).with_year(year).unwrap();
    let to_date = get_end_of(GroupBy::Year).with_year(year).unwrap();
    let transactions = get_transactions_between(transactions, from_date, to_date);
    let (labels, values) = generate_label_value_summary(
        &transactions,
        vec![GroupBy::Year, GroupBy::Month],
        &|i| format!("{:04} {:02}", year, (i % 12) + 1),
        12,
    );
    let mut payload = json!(TransactionSummary::from(&transactions));
    payload["chart"] = json!({"labels": labels, "values": values});
    Ok(payload)
}

pub async fn get_today_summary(transactions: &[transaction::Model]) -> Result<Value, AppError> {
    let transactions = get_transactions_between(
        transactions,
        get_start_of(GroupBy::Day),
        get_end_of(GroupBy::Day),
    );

    let (labels, values) = generate_label_value_summary(
        &transactions,
        vec![GroupBy::Hour],
        &|i| format!("{:02}:00", i % 24),
        24,
    );

    let mut payload = json!(TransactionSummary::from(&transactions));
    payload["chart"] = json!({"labels": labels, "values": values});
    Ok(payload)
}

pub async fn get_total_summary(transactions: &[transaction::Model]) -> Result<Value, AppError> {
    let year = Utc::now().year();
    let mut payload = json!(TransactionSummary::from(&transactions.to_vec()));
    payload["present_year"] = json!(get_summary_from_year(transactions, year).await?);
    payload["last_year"] = json!(get_summary_from_year(transactions, year - 1).await?);
    Ok(payload)
}

pub async fn get_recent_days_summary(
    transactions: &[transaction::Model],
    days: i64,
) -> Result<Value, AppError> {
    let from_date = Utc::now()
        .naive_utc()
        .checked_sub_signed(chrono::Duration::days(days))
        .unwrap();
    let to_date = Utc::now().naive_utc();

    let transactions = get_transactions_between(
        transactions,
        DateTime::<Utc>::from_utc(from_date, Utc),
        DateTime::<Utc>::from_utc(to_date, Utc),
    );

    let (labels, values) = generate_label_value_summary(
        &transactions,
        vec![GroupBy::Year, GroupBy::Month, GroupBy::Day],
        &|i| {
            let date = from_date
                .checked_add_signed(chrono::Duration::days(i as i64))
                .unwrap();
            date.format("%Y %m %d").to_string()
        },
        (to_date - from_date).num_days() as usize + 1, // Include the end date
    );
    let mut payload = json!(TransactionSummary::from(&transactions));
    payload["chart"] = json!({"labels": labels, "values": values});
    Ok(payload)
}

pub async fn get_best_selling_items(
    transactions: &[transaction::Model],
) -> Result<Vec<TransactionItemSummary>, AppError> {
    let mut items = TransactionItemSummary::from_transactions(&transactions.to_vec());

    // Sort items by profit in descending order
    items.sort_by(|a, b| b.profit.cmp(&a.profit));

    Ok(items.into_iter().take(10).collect())
}

pub async fn get_category_summary(
    transactions: &[transaction::Model],
    categories: Vec<SummaryCategorySetting>,
) -> Result<Vec<TransactionCategorySummary>, AppError> {
    let mut items = vec![];
    for category in categories {
        items.push(TransactionCategorySummary::from_transactions(
            &transactions.to_vec(),
            &category,
        ));
    }
    Ok(items)
}

#[tauri::command]
pub async fn summary_overview(
    settings: tauri::State<'_, Arc<Mutex<SettingsState>>>,
) -> Result<Value, AppError> {
    let conn = DATABASE.get().unwrap();
    let settings = settings.lock()?.clone();

    // Get Lasted transactions
    let transactions = match TransactionQuery::get_all(
        conn,
        entity::transaction::dto::TransactionPaginationQueryDto::new(1, -1)
            .set_sort_by(Some("created_at".to_string()))
            .set_sort_direction(Some(SortDirection::Desc)),
    )
    .await
    {
        Ok(transactions) => transactions.results,
        Err(e) => {
            let error: AppError = AppError::new_db("TransactionQuery::get_all", e);
            error::create_log_file("transaction_get_lasted.log", &error);
            return Err(error);
        }
    };

    let today = get_today_summary(&transactions).await?;
    let total = get_total_summary(&transactions).await?;
    let recent_days =
        get_recent_days_summary(&transactions, settings.summary_settings.resent_days).await?;
    let best_selling_items = get_best_selling_items(&transactions).await?;
    let category_summary =
        get_category_summary(&transactions, settings.summary_settings.categories).await?;

    Ok(json!({
        "today": today,
        "total": total,
         "recent_days": recent_days,
         "best_selling_items": best_selling_items,
         "category_summary": category_summary,
         "resent_transactions": transactions.into_iter().take(settings.summary_settings.resent_transactions as usize).collect::<Vec<_>>()
    }))
}
