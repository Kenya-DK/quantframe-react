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

pub fn emit_progress(id: &str, i18n_key: &str, values: Option<Value>, is_completed: bool) {
    send_message_to_window(
        "Client:Update:Progress",
        Some(
            json!({ "id": id, "i18n_key": i18n_key,"values": values, "isCompleted": is_completed}),
        ),
    );
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
        let roaming_path = Path::new(base_dirs.cache_dir());
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

pub fn get_desktop_path() -> PathBuf {
    if let Some(base_dirs) = UserDirs::new() {
        let local_path = get_app_roaming_path(); // Ensure local_path lives long enough

        let desktop_path = match base_dirs.desktop_dir() {
            Some(desktop_path) => desktop_path,
            None => local_path.as_path(),
        };
        desktop_path.to_path_buf()
    } else {
        panic!("Could not find app path");
    }
}

#[derive(Clone, Debug)]
pub struct ZipEntry {
    pub file_path: PathBuf,
    pub sub_path: Option<String>,
    pub include_dir: bool,
}

pub fn get_zip_entrys(path: PathBuf, in_subfolders: bool) -> Result<Vec<ZipEntry>, AppError> {
    let mut files: Vec<ZipEntry> = Vec::new();
    for path in fs::read_dir(path).unwrap() {
        let path = path.unwrap().path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_str().unwrap();
            let subfiles = get_zip_entrys(path.to_owned(), in_subfolders)?;
            for mut subfile in subfiles {
                let sub_path = subfile.sub_path.clone().unwrap_or("".to_string());
                // Remove the first slash if it exists
                let full_path = format!("{}/{}", dir_name, sub_path);
                subfile.sub_path = Some(full_path);
                files.push(subfile);
            }
        }
        if path.is_file() {
            files.push(ZipEntry {
                file_path: path.clone(),
                sub_path: None,
                include_dir: false,
            });
        }
    }
    Ok(files)
}

pub fn create_zip_file(mut files: Vec<ZipEntry>, zip_path: &str) -> Result<(), AppError> {
    let zip_file_path = Path::new(&zip_path);
    let zip_file =
        File::create(&zip_file_path).map_err(|e| AppError::new("Zip", eyre!(e.to_string())))?;
    let mut zip = ZipWriter::new(zip_file);

    // Get all files that are directories and add them to the files list
    let mut files_to_compress: Vec<ZipEntry> = Vec::new();

    for file_entry in &files {
        if file_entry.include_dir {
            let subfiles = get_zip_entrys(file_entry.file_path.clone(), true)?;
            for mut subfile in subfiles {
                if subfile.sub_path.is_some() {
                    subfile.sub_path = Some(format!(
                        "{}/{}",
                        file_entry.sub_path.clone().unwrap_or("".to_string()),
                        subfile.sub_path.clone().unwrap_or("".to_string())
                    ));
                }
                files_to_compress.push(subfile);
            }
        }
    }
    files.append(&mut files_to_compress);

    // Set compression options (e.g., compression method)
    let options = FileOptions::default().compression_method(CompressionMethod::DEFLATE);

    for file_entry in &files {
        if file_entry.include_dir {
            continue;
        }

        let file_path = Path::new(&file_entry.file_path)
            .canonicalize()
            .map_err(|e| AppError::new("Zip", eyre!(e.to_string())))?;

        if !file_path.exists() || !file_path.is_file() {
            continue;
        }

        let file = File::open(&file_path).map_err(|e| {
            AppError::new(
                "Zip:Open",
                eyre!(format!(
                    "Path: {:?}, Error: {}",
                    file_entry.file_path.clone(),
                    e.to_string()
                )),
            )
        })?;
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        // Adding the file to the ZIP archive.
        if file_entry.sub_path.is_some() && file_entry.sub_path.clone().unwrap() != "" {
            let mut sub_path = file_entry.sub_path.clone().unwrap();
            if sub_path.starts_with("/") {
                sub_path = sub_path[1..].to_string();
            }
            if sub_path.ends_with("/") {
                sub_path = sub_path[..sub_path.len() - 1].to_string();
            }
            zip.start_file(format!("{}/{}", sub_path, file_name), options)
                .map_err(|e| {
                    AppError::new(
                        "Zip:StartSub",
                        eyre!(format!(
                            "Path: {:?}, ZipPath: {:?}, Error: {}",
                            file_entry.file_path.clone(),
                            file_entry.sub_path.clone(),
                            e.to_string()
                        )),
                    )
                })?;
        } else {
            zip.start_file(file_name, options).map_err(|e| {
                AppError::new(
                    "Zip:Start",
                    eyre!(format!(
                        "Path: {:?}, Error: {}",
                        file_entry.file_path,
                        e.to_string()
                    )),
                )
            })?;
        }
        let mut buffer = Vec::new();
        io::copy(&mut file.take(u64::MAX), &mut buffer).map_err(|e| {
            AppError::new(
                "Zip:Copy",
                eyre!(format!(
                    "Path: {:?}, Error: {}",
                    file_entry.file_path,
                    e.to_string()
                )),
            )
        })?;

        zip.write_all(&buffer).map_err(|e| {
            AppError::new(
                "Zip:Write",
                eyre!(format!(
                    "Path: {:?}, Error: {}",
                    file_entry.file_path,
                    e.to_string()
                )),
            )
        })?;
    }
    zip.finish()
        .map_err(|e| AppError::new("Zip:Done", eyre!(format!("Error: {}", e.to_string()))))?;
    Ok(())
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
    let error = format!(
        "Column: {:?} ColumnType: {:?} Error: [] [J]{}[J]",
        column,
        col_type,
        serde_json::to_value(&df).unwrap().to_string()
    );

    let df: DataFrame = match filter {
        Some(filter) => df.lazy().filter(filter).collect().map_err(|e| {
            AppError::new("Helper", eyre!(error.replace("[]", e.to_string().as_str())))
        })?,
        None => df,
    };

    let column_series = df
        .column(column)
        .map_err(|e| AppError::new("Helper", eyre!(error.replace("[]", e.to_string().as_str()))))?;

    match col_type {
        ColumnType::Bool => {
            let values: Vec<bool> = column_series
                .bool()
                .map_err(|e| {
                    AppError::new("Helper", eyre!(error.replace("[]", e.to_string().as_str())))
                })?
                .into_iter()
                .filter_map(|opt_val| opt_val)
                .collect();
            Ok(ColumnValues::Bool(values))
        }

        ColumnType::F64 => {
            let values: Vec<f64> = column_series
                .f64()
                .map_err(|e| {
                    AppError::new("Helper", eyre!(error.replace("[]", e.to_string().as_str())))
                })?
                .into_iter()
                .filter_map(|opt_val| opt_val)
                .collect();
            Ok(ColumnValues::F64(values))
        }

        ColumnType::I64 => {
            let values: Vec<i64> = column_series
                .i64()
                .map_err(|e| {
                    AppError::new("Helper", eyre!(error.replace("[]", e.to_string().as_str())))
                })?
                .into_iter()
                .filter_map(|opt_val| opt_val)
                .collect();
            Ok(ColumnValues::I64(values))
        }
        ColumnType::I32 => {
            let values: Vec<i32> = column_series
                .i32()
                .map_err(|e| {
                    AppError::new("Helper", eyre!(error.replace("[]", e.to_string().as_str())))
                })?
                .into_iter()
                .filter_map(|opt_val| opt_val)
                .collect();
            Ok(ColumnValues::I32(values))
        }
        ColumnType::String => {
            let values = column_series
                .utf8()
                .map_err(|e| {
                    AppError::new("Helper", eyre!(error.replace("[]", e.to_string().as_str())))
                })?
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
        ColumnValues::I32(i32_values) => {
            let value = i32_values.get(0).cloned();
            Ok(ColumnValue::I32(value))
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

pub fn validate_json(json: &Value, required: &Value, path: &str) -> (Value, Vec<String>) {
    let mut modified_json = json.clone();
    let mut missing_properties = Vec::new();

    if let Some(required_obj) = required.as_object() {
        for (key, value) in required_obj {
            let full_path = if path.is_empty() {
                key.clone()
            } else {
                format!("{}.{}", path, key)
            };

            if !json.as_object().unwrap().contains_key(key) {
                missing_properties.push(full_path.clone());
                modified_json[key] = required_obj[key].clone();
            } else if value.is_object() {
                let sub_json = json.get(key).unwrap();
                let (modified_sub_json, sub_missing) = validate_json(sub_json, value, &full_path);
                if !sub_missing.is_empty() {
                    modified_json[key] = modified_sub_json;
                    missing_properties.extend(sub_missing);
                }
            }
        }
    }

    (modified_json, missing_properties)
}
