use chrono::{DateTime, Utc};
use entity::dto::{FinancialGraph, FinancialReport, PaginatedResult};
use serde_json::Value;
use std::{
    fs::{self},
    path::PathBuf,
};
use tauri::Manager;
use utils::*;

use crate::APP;

pub static APP_PATH: &str = "dev.kenya.quantframe";

pub fn get_device_id() -> String {
    let app = APP.get().unwrap();
    let home_dir = match app.path().home_dir() {
        Ok(val) => val,
        Err(_) => {
            panic!("Could not find home directory");
        }
    };
    let device_name = home_dir.file_name().unwrap().to_str().unwrap();
    device_name.to_string()
}
pub fn get_app_storage_path() -> PathBuf {
    let app = APP.get().unwrap();
    let local_path = match app.path().local_data_dir() {
        Ok(val) => val,
        Err(_) => {
            panic!("Could not find app path");
        }
    };

    let app_path = local_path.join(APP_PATH);
    if !app_path.exists() {
        fs::create_dir_all(&app_path).unwrap()
    }
    app_path
}
pub fn get_desktop_path() -> PathBuf {
    let app = APP.get().unwrap();
    let desktop_path = match app.path().desktop_dir() {
        Ok(val) => val,
        Err(_) => {
            panic!("Could not find desktop path");
        }
    };
    desktop_path
}
pub fn generate_transaction_summary(
    transactions: &Vec<entity::transaction::Model>,
    date: DateTime<Utc>,
    group_by1: GroupByDate,
    group_by2: &[GroupByDate],
    _previous: bool,
) -> (FinancialReport, FinancialGraph<i64>) {
    let (start, end) = get_start_end_of(date, group_by1);
    let transactions = filters_by(transactions, |t| {
        t.created_at >= start && t.created_at <= end
    });

    let mut grouped = group_by_date(&transactions, |t| t.created_at, group_by2);

    fill_missing_date_keys(&mut grouped, start, end, group_by2);

    let graph = FinancialGraph::<i64>::from(&grouped, |group| {
        FinancialReport::from(&group.to_vec()).total_profit
    });
    (FinancialReport::from(&transactions), graph)
}

/// Paginate a vector of items
pub fn paginate<T: Clone>(items: &[T], page: i64, per_page: i64) -> PaginatedResult<T> {
    let total_items = items.len() as i64;

    let start = (page.saturating_sub(1)) * per_page;
    let end = (start + per_page).min(total_items);

    let start_usize = start as usize;
    let end_usize = end as usize;

    let page_items = if start < total_items && end > 0 {
        items[start_usize..end_usize].to_vec()
    } else if per_page == -1 {
        items.to_vec()
    } else {
        Vec::new()
    };

    PaginatedResult {
        results: page_items,
        page,
        limit: per_page,
        total: total_items,
    }
}
