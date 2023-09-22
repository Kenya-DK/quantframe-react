use chrono::Duration;
use directories::BaseDirs;
use eyre::eyre;
use once_cell::sync::Lazy;
use polars::{
    lazy::dsl::col,
    prelude::{DataFrame, Expr, IntoLazy, SortOptions},
    series::Series,
};
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};
use tauri::Window;

use crate::{
    error::AppError,
    logger::LogLevel,
};

pub static WINDOW: Lazy<Mutex<Option<Window>>> = Lazy::new(|| Mutex::new(None));

pub enum ColumnType {
    Bool,
    F64,
    I64,
    String,
}
pub enum ColumnValues {
    Bool(Vec<bool>),
    F64(Vec<f64>),
    I64(Vec<i64>),
    String(Vec<String>),
}
pub enum ColumnValue {
    Bool(Option<bool>),
    F64(Option<f64>),
    I64(Option<i64>),
    String(Option<String>),
}

pub fn send_message_to_window(event: &str, data: Option<Value>) {
    let window = WINDOW.lock().unwrap();
    if let Some(window) = &*window {
        let rep = window.emit("message", json!({ "event": event, "data": data }));
        match rep {
            Ok(_) => {}
            Err(e) => {
                println!("Error while sending message to window {:?}", e);
            }
        }
    }
}

pub fn emit_update(update_type: &str, operation: &str,data: Option<Value>) {
    helper::send_message_to_window("Client:Update", Some(json!({ "type": update_type, "operation": operation, "data": data})));   
}

pub fn get_app_local_path() -> PathBuf {
    if let Some(base_dirs) = BaseDirs::new() {
        // App path for csv file
        let local_path = Path::new(base_dirs.data_local_dir());
        local_path.to_path_buf()
    } else {
        panic!("Could not find app path");
    }
}

pub fn get_app_roaming_path() -> PathBuf {
    if let Some(base_dirs) = BaseDirs::new() {
        // App path for csv file
        let roaming_path = Path::new(base_dirs.data_dir());
        let app_path = roaming_path.join("dev.kenya.quantframe");
        // Check if the app path exists, if not create it
        if !app_path.exists() {
            fs::create_dir_all(app_path.clone()).unwrap();
        }
        app_path
    } else {
        panic!("Could not find app path");
    }
}

pub fn sort_dataframe(df: DataFrame, column: &str, ascending: bool) -> Result<DataFrame, AppError> {
    let df = df
        .clone()
        .lazy()
        .sort(
            column,
            SortOptions {
                descending: ascending,
                nulls_last: false,
                multithreaded: false,
            },
        )
        .collect()
        .map_err(|e| AppError::new("Helper", eyre!(e.to_string())))?;
    Ok(df)
}

pub fn filter_and_extract(
    df: DataFrame,
    filter: Option<Expr>,
    select_cols: Vec<&str>,
) -> Result<DataFrame, AppError> {
    let selected_columns: Vec<_> = select_cols.into_iter().map(col).collect();

    let df = match filter {
        Some(filter) => df
            .lazy()
            .filter(filter)
            .collect()
            .map_err(|e| AppError::new("Helper", eyre!(e.to_string())))?,
        None => df,
    };

    let df_select = df.lazy().select(&selected_columns).collect();
    match df_select {
        Ok(df_select) => Ok(df_select),
        Err(e) => Err(AppError::new("Helper", eyre!(e.to_string()))),
    }
}

pub fn get_column_values(
    df: DataFrame,
    filter: Option<Expr>,
    column: &str,
    col_type: ColumnType,
) -> Result<ColumnValues, AppError> {
    let df: DataFrame = match filter {
        Some(filter) => df
            .lazy()
            .filter(filter)
            .collect()
            .map_err(|e| AppError::new("Helper", eyre!(e.to_string())))?,
        None => df,
    };

    let column_series = df
        .column(column)
        .map_err(|e| AppError::new("Helper", eyre!(e.to_string())))?;

    match col_type {
        ColumnType::Bool => {
            let values: Vec<bool> = column_series
                .bool()
                .map_err(|e| AppError::new("Helper", eyre!(e.to_string())))?
                .into_iter()
                .filter_map(|opt_val| opt_val)
                .collect();
            Ok(ColumnValues::Bool(values))
        }

        ColumnType::F64 => {
            let values: Vec<f64> = column_series
                .f64()
                .map_err(|e| AppError::new("Helper", eyre!(e.to_string())))?
                .into_iter()
                .filter_map(|opt_val| opt_val)
                .collect();
            Ok(ColumnValues::F64(values))
        }

        ColumnType::I64 => {
            let values: Vec<i64> = column_series
                .i64()
                .map_err(|e| AppError::new("Helper", eyre!(e.to_string())))?
                .into_iter()
                .filter_map(|opt_val| opt_val)
                .collect();
            Ok(ColumnValues::I64(values))
        }
        ColumnType::String => {
            let values = column_series
                .utf8()
                .map_err(|e| AppError::new("Helper", eyre!(e.to_string())))?
                .into_iter()
                .filter_map(|opt_name| opt_name.map(String::from))
                .collect::<Vec<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            Ok(ColumnValues::String(values))
        }
    }
}
pub fn get_column_value(
    df: DataFrame,
    filter: Option<Expr>,
    column: &str,
    col_type: ColumnType,
) -> Result<ColumnValue, AppError> {
    match get_column_values(df, filter, column, col_type)? {
        ColumnValues::Bool(bool_values) => {
            let value = bool_values.get(0).cloned();
            Ok(ColumnValue::Bool(value))
        }
        ColumnValues::F64(f64_values) => {
            let value = f64_values.get(0).cloned();
            Ok(ColumnValue::F64(value))
        }
        ColumnValues::I64(i64_values) => {
            let value = i64_values.get(0).cloned();
            Ok(ColumnValue::I64(value))
        }
        ColumnValues::String(string_values) => {
            let value = string_values.get(0).cloned();
            Ok(ColumnValue::String(value))
        }
    }
}

pub fn merge_dataframes(frames: Vec<DataFrame>) -> Result<DataFrame, AppError> {
    // Check if there are any frames to merge
    if frames.is_empty() {
        return Err(AppError::new("Helper", eyre!("No frames to merge")));
    }

    // Get the column names from the first frame
    let column_names: Vec<&str> = frames[0].get_column_names();

    // For each column name, stack the series from all frames vertically
    let mut combined_series: Vec<Series> = Vec::new();

    for &col_name in &column_names {
        let first_series = frames[0]
            .column(col_name)
            .map_err(|e| AppError::new("Helper", eyre!(e.to_string())))?
            .clone();
        let mut stacked_series = first_series;

        for frame in frames.iter().skip(1) {
            let series = frame
                .column(col_name)
                .map_err(|e| AppError::new("Helper", eyre!(e.to_string())))?
                .clone();
            stacked_series = stacked_series
                .append(&series)
                .map_err(|e| AppError::new("Helper", eyre!(e.to_string())))?
                .clone();
        }

        combined_series.push(stacked_series);
    }
    // Construct a DataFrame from the merged data
    Ok(DataFrame::new(combined_series)
        .map_err(|e| AppError::new("Helper", eyre!(e.to_string())))?)
}
/// Returns a vector of strings representing the dates of the last `x` days, including today.
/// The dates are formatted as "YYYY-MM-DD".
pub fn last_x_days(x: i64) -> Vec<String> {
    let today = chrono::Local::now().naive_utc();
    (0..x)
        .rev()
        .map(|i| {
            (today - Duration::days(i + 1))
                .format("%Y-%m-%d")
                .to_string()
        })
        .rev()
        .collect()
}
pub fn send_message_to_discord(webhook: String, message: String, ping: bool) {
    tauri::async_runtime::spawn(async move {
        let client = reqwest::Client::new();
        let mut user_id: Option<String> = None;

        if ping {
            let res = client.get(webhook.as_str()).send().await;
            if let Ok(res) = res {
                let json: Value = res.json().await.unwrap();
                if let Some(id) = json.get("user").unwrap().get("id") {
                    user_id = Some(id.to_string());
                }
            }
        }
        let mut message = message.to_string();
        if user_id.is_some() {
            message = format!("<@{}> {}", user_id.unwrap(), message).replace("\"", "");
        }

        let res = client
            .post(webhook)
            .json(&json!({ "content": message }))
            .send()
            .await;
        match res {
            Ok(_) => {}
            Err(e) => {
                println!("Error while sending message to discord {:?}", e);
            }
        }
    });
}
