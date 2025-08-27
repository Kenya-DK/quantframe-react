use std::sync::Mutex;

use crate::{
    app::{client::AppState, SummaryCategorySetting},
    helper::generate_transaction_summary,
    DATABASE,
};
use chrono::{DateTime, Utc};
use entity::{
    dto::*,
    transaction::{self, dto::*},
};
use serde_json::{json, Value};
use service::*;
use utils::*;

pub async fn get_total_summary(transactions: &Vec<transaction::Model>) -> Result<Value, Error> {
    let (this_year_total, this_year_graph) = generate_transaction_summary(
        transactions,
        Utc::now(),
        GroupByDate::Year,
        &[GroupByDate::Year, GroupByDate::Month],
        false,
    );

    let (last_year_total, last_year_graph) = generate_transaction_summary(
        transactions,
        Utc::now() - chrono::Duration::days(365),
        GroupByDate::Year,
        &[GroupByDate::Year, GroupByDate::Month],
        false,
    );

    let mut payload = json!(FinancialReport::from(transactions));
    payload["present_year"] = json!({
        "summary": this_year_total,
        "chart": this_year_graph
    });
    payload["last_year"] = json!({
        "summary": last_year_total,
        "chart": last_year_graph
    });

    Ok(payload)
}

pub async fn get_today_summary(transactions: &Vec<transaction::Model>) -> Result<Value, Error> {
    let (report, graph) = generate_transaction_summary(
        transactions,
        Utc::now(),
        GroupByDate::Day,
        &[GroupByDate::Hour],
        false,
    );
    Ok(json!({
        "summary": report,
        "chart": graph
    }))
}
pub async fn get_best_selling_items(
    transactions: &Vec<transaction::Model>,
) -> Result<FinancialReport, Error> {
    let mut reports = Vec::new();

    let items = group_by(transactions, |item| item.wfm_id.clone());

    for (_, group) in items {
        if let Some(first_item) = group.first() {
            let data = json!({
                "wfm_id": first_item.wfm_id,
                "item_name": first_item.item_name,
                "item_type": first_item.item_type,
            });

            let report = FinancialReport::from(&group);
            reports.push(report.with_properties(data));
        }
    }

    reports.sort_by(|a, b| b.total_profit.cmp(&a.total_profit));

    let best_seller = reports.into_iter().next().unwrap_or_else(|| {
        FinancialReport::default().with_properties(json!({
            "wfm_id": "N/A",
            "item_name": "N/A",
            "item_type": "N/A"
        }))
    });

    Ok(best_seller)
}
pub async fn get_recent_days_summary(
    transactions: &Vec<transaction::Model>,
    days: i64,
) -> Result<Value, Error> {
    let (start, mut end) = get_start_end_of(Utc::now(), GroupByDate::Day);
    end = end + chrono::Duration::days(days); // Include the end date

    let transactions = filters_by(transactions, |t| {
        t.created_at >= start && t.created_at <= end
    });

    let mut grouped = group_by_date(
        &transactions,
        |t| t.created_at,
        &[GroupByDate::Year, GroupByDate::Month, GroupByDate::Day],
    );

    fill_missing_date_keys(
        &mut grouped,
        start,
        end,
        &[GroupByDate::Year, GroupByDate::Month, GroupByDate::Day],
    );

    let graph = FinancialGraph::<i64>::from(&grouped, |group| {
        FinancialReport::from(&group.to_vec()).total_profit
    });

    Ok(json!({
        "summary": FinancialReport::from(&transactions),
        "chart": graph
    }))
}
pub async fn get_category_summary(
    transactions: &Vec<transaction::Model>,
    categories: &Vec<SummaryCategorySetting>,
) -> Result<Vec<FinancialReport>, Error> {
    let mut items = vec![];
    for category in categories {
        let tags = &category.tags;
        let types = &category.types;
        let filtered_transactions = filters_by(transactions, |t| {
            let tag_matches = t
                .tags
                .split(',')
                .any(|tag| tags.contains(&tag.trim().to_string()));

            let type_matches = types.contains(&t.item_type.to_string());

            tag_matches || type_matches
        });
        items.push(
            FinancialReport::from(&filtered_transactions).with_properties(json!({
                "icon": category.icon,
                "name": category.name,
            })),
        );
    }
    Ok(items)
}
#[tauri::command]
pub async fn dashboard_summary(app: tauri::State<'_, Mutex<AppState>>) -> Result<Value, Error> {
    let conn = DATABASE.get().unwrap();
    let transactions = TransactionQuery::get_all(
        &conn,
        TransactionPaginationQueryDto::new(1, -1)
            .set_sort_by("created_at".to_string())
            .set_sort_direction(SortDirection::Desc),
    )
    .await.unwrap() ;

    let app = app.lock()?.clone();
    let category_summary = get_category_summary(
        &transactions.results,
        &app.settings.summary_settings.categories,
    )
    .await?;

    Ok(json!({
        "total": get_total_summary(&transactions.results).await.unwrap(),
        "today": get_today_summary(&transactions.results).await.unwrap(),
        "recent_days": get_recent_days_summary(&transactions.results, 7).await.unwrap(),
        "best_seller": get_best_selling_items(&transactions.results).await?,
        "categories": category_summary,
        "resent_transactions": transactions.take_top(app.settings.summary_settings.resent_transactions as usize)
    }))
}
