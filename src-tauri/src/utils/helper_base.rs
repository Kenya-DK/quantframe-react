use chrono::{format, Duration};
use directories::{BaseDirs, UserDirs};
use eyre::eyre;
use once_cell::sync::Lazy;
use polars::{
    lazy::dsl::col,
    prelude::{DataFrame, Expr, IntoLazy, SortOptions},
    series::Series,
};
use serde_json::{json, Value};
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    sync::Mutex,
};
use tauri::{api::file, Window};
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

use crate::{
    error::AppError,
    logger::{self},
    structs::WarframeLanguage,
    PACKAGEINFO,
};
pub static WINDOW: Lazy<Mutex<Option<Window>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug)]
pub enum ColumnType {
    Bool,
    F64,
    I64,
    I32,
    String,
}
pub enum ColumnValues {
    Bool(Vec<bool>),
    F64(Vec<f64>),
    I64(Vec<i64>),
    I32(Vec<i32>),
    String(Vec<String>),
}
pub enum ColumnValue {
    Bool(Option<bool>),
    F64(Option<f64>),
    I64(Option<i64>),
    I32(Option<i32>),
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

pub async fn get_app_info() -> Result<serde_json::Value, AppError> {
    let packageinfo = PACKAGEINFO
        .lock()
        .unwrap()
        .clone()
        .expect("Could not get package info");
    let version = packageinfo.version.to_string();
    let url = "https://raw.githubusercontent.com/Kenya-DK/quantframe-react/main/src-tauri/tauri.conf.json";
    let client = reqwest::Client::new();
    let request = client.request(reqwest::Method::GET, reqwest::Url::parse(&url).unwrap());
    let response = request.send().await;
    if let Err(e) = response {
        return Err(AppError::new("CHECKFORUPDATES", eyre!(e.to_string())));
    }
    let response_data = response.unwrap();
    let status = response_data.status();

    if status != 200 {
        return Err(AppError::new(
            "CHECKFORUPDATES",
            eyre!("Could not get package.json. Status: {}", status.to_string()),
        ));
    }
    let response = response_data.json::<Value>().await.unwrap();

    let current_version_str = response["package"]["version"].as_str().unwrap();
    let current_version = current_version_str.replace(".", "");
    let current_version = current_version.parse::<i32>().unwrap();

    let version_str = version;
    let version = version_str.replace(".", "").parse::<i32>().unwrap();

    let update_state = json!({
        "update_available": current_version > version,
        "version": current_version_str,
        "current_version": version_str,
        "release_notes": "New version available",
        "download_url": "https://github.com/Kenya-DK/quantframe-react/releases",
    });

    let rep = json!({
        "app_name": packageinfo.name,
        "app_description": packageinfo.description,
        "app_author": packageinfo.authors,
        "app_version": update_state,
    });

    Ok(rep)
}

pub fn emit_update(update_type: &str, operation: &str, data: Option<Value>) {
    send_message_to_window(
        "Client:Update",
        Some(json!({ "type": update_type, "operation": operation, "data": data})),
    );
}

pub fn emit_undate_initializ_status(status: &str, data: Option<Value>) {
    send_message_to_window(
        "set_initializstatus",
        Some(json!({
            "status": status,
            "data": data
        })),
    );
}

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
pub fn send_message_to_discord(
    webhook: String,
    title: String,
    content: String,
    user_ids: Option<Vec<String>>,
) {
    // Check if the webhook is empty
    if webhook.is_empty() {
        logger::warning_con("Helper", "Discord webhook is empty");
        return;
    }
    tauri::async_runtime::spawn(async move {
        let client = reqwest::Client::new();

        let mut body = json!({
            "username": "Quantframe",
            "avatar_url": "https://i.imgur.com/bgR6vAd.png",
            "embeds": [
                {
                    "title": title,
                    "description": content,
                    "color": 5814783,
                    "footer": {
                        "text": format!("Quantframe v{}", PACKAGEINFO.lock().unwrap().clone().unwrap().version.to_string()),
                        "timestamp": chrono::Local::now()
                        .naive_utc()
                        .to_string()
                    }
                }
            ]
        });

        let mut pings: Vec<String> = Vec::new();
        if let Some(user_ids) = user_ids {
            for user_id in user_ids {
                pings.push(format!("<@{}>", user_id));
            }
        }
        if pings.len() > 0 {
            body["content"] = json!(format!("{}", pings.join(" ")).replace("\"", ""));
        } else {
            body["content"] = json!("");
        }

        let res = client.post(webhook).json(&body).send().await;
        match res {
            Ok(_) => {
                logger::info_con("Helper", "Message sent to discord");
            }
            Err(e) => {
                println!("Error while sending message to discord {:?}", e);
            }
        }
    });
}

pub async fn alter_table(
    connection: sqlx::Pool<sqlx::Sqlite>,
    alter_sql: &str,
) -> Result<bool, AppError> {
    let re = regex::Regex::new(r#"ALTER TABLE "(?P<table>[^"]+)" ADD COLUMN "(?P<column>[^"]+)"#)
        .unwrap();
    if let Some(captures) = re.captures(alter_sql) {
        let table_name = captures.name("table").map_or("", |m| m.as_str());
        let column_name = captures.name("column").map_or("", |m| m.as_str());
        if table_name != "" && column_name != "" {
            let rep = sqlx::query(format!("PRAGMA table_info(\"{}\")", table_name).as_str())
                .fetch_all(&connection)
                .await
                .map_err(|e| AppError::new("Database", eyre!(e.to_string())));
            match rep {
                Ok(r) => {
                    for row in r {
                        let name: String = sqlx::Row::get(&row, "name");
                        if name == column_name {
                            return Ok(true);
                        }
                    }
                    sqlx::query(&alter_sql)
                        .execute(&connection)
                        .await
                        .map_err(|e| AppError::new("Database", eyre!(e.to_string())))?;
                    return Ok(false);
                }
                Err(_) => {
                    return Err(AppError::new(
                        "Database",
                        eyre!("Could not find table in database."),
                    ));
                }
            }
        } else {
            logger::warning_con(
                "Helper",
                "Could not find table name or column name in the SQL.",
            );
        }
    } else {
        logger::warning_con(
            "Helper",
            "Could not find table name or column name in the SQL.",
        );
    }
    Ok(false)
}

pub fn calculate_trade_tax(item_tags: Vec<String>, rank: Option<i64>) -> i64 {
    // If tags contains "arcane_upgrade" then it is an arcane
    if item_tags.contains(&"arcane_enhancement".to_string()) {
        if item_tags.contains(&"common".to_string()) {
            return 2000;
        } else if item_tags.contains(&"uncommon".to_string()) {
            return 4000;
        } else if item_tags.contains(&"rare".to_string()) {
            return 8000;
        } else if item_tags.contains(&"legendary".to_string()) {
            let rank_tax = Vec::from([100000, 300000, 600000, 1000000, 1500000, 2100000]);
            let rank = rank.unwrap_or(0);
            if rank > 0 && rank < 7 {
                return rank_tax[rank as usize];
            } else {
                return rank_tax[0];
            }
        }
    }
    if item_tags.contains(&"mod".to_string()) {
        if item_tags.contains(&"common".to_string()) {
            return 2000;
        } else if item_tags.contains(&"uncommon".to_string()) {
            return 4000;
        } else if item_tags.contains(&"rare".to_string()) {
            return 8000;
        } else if item_tags.contains(&"legendary".to_string())
            || item_tags.contains(&"archon".to_string())
        {
            return 1000000;
        }
    }
    2000
}

pub fn get_warframe_language() -> WarframeLanguage {
    let path = get_app_local_path().join("Warframe").join("Launcher.log");

    let log_file = "get_warframe_language.log";

    if !path.exists() {
        return WarframeLanguage::English;
    }

    let file_result = fs::File::open(&path);
    let mut contents = String::new();

    if let Ok(mut file) = file_result {
        if let Ok(_) = std::io::Read::read_to_string(&mut file, &mut contents) {
            if let Some(num) = contents.rfind("-language:") {
                let lang_code = &contents[num + 10..num + 12];

                // Ensure lang_code is exactly two characters
                if lang_code.len() == 2 {
                    return WarframeLanguage::from_str(lang_code);
                } else {
                    logger::info_con(
                        "Helper",
                        format!(
                            "Could not find language code in Warframe launcher log file at {:?}",
                            path.to_str()
                        )
                        .as_str(),
                    );
                }
            } else {
                logger::info_con(
                    "Helper",
                    format!(
                        "Could not find language code in Warframe launcher log file at {:?}",
                        path.to_str()
                    )
                    .as_str(),
                );
            }
        } else {
            logger::info_con(
                "Helper",
                format!(
                    "Could not read Warframe launcher log file at {:?}",
                    path.to_str()
                )
                .as_str(),
            );
        }
    } else {
        logger::info_con(
            "Helper",
            format!(
                "Could not open Warframe launcher log file at {:?}",
                path.to_str()
            )
            .as_str(),
        );
    }

    // Default to English in case of any error
    WarframeLanguage::English
}
