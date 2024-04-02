use directories::{BaseDirs, UserDirs};
use eyre::eyre;
use once_cell::sync::Lazy;
use serde_json::{json, Map, Value};
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    sync::Mutex,
};
use tauri::Window;
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

use crate::{logger, utils::modules::error::AppError, PACKAGEINFO};
pub static WINDOW: Lazy<Mutex<Option<Window>>> = Lazy::new(|| Mutex::new(None));

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

pub fn get_app_info() -> Result<serde_json::Value, AppError> {
    let packageinfo = PACKAGEINFO
        .lock()
        .unwrap()
        .clone()
        .expect("Could not get package info");
    let version = packageinfo.version.to_string();

    let rep = json!({
        "app_name": packageinfo.name,
        "app_description": packageinfo.description,
        "app_author": packageinfo.authors,
        "app_version": version,
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

pub fn emit_update_initialization_status(status: &str, data: Option<Value>) {
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
    pub content: Option<String>,
    pub include_dir: bool,
}

pub fn read_zip_entries(
    path: PathBuf,
    include_subfolders: bool,
) -> Result<Vec<ZipEntry>, AppError> {
    let mut files: Vec<ZipEntry> = Vec::new();
    for path in fs::read_dir(path).unwrap() {
        let path = path.unwrap().path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_str().unwrap();
            let file_entries = read_zip_entries(path.to_owned(), include_subfolders)?;
            for mut archive_entry in file_entries {
                let sub_path = archive_entry.sub_path.clone().unwrap_or("".to_string());
                // Remove the first slash if it exists
                let full_path = format!("{}/{}", dir_name, sub_path);
                archive_entry.sub_path = Some(full_path);
                files.push(archive_entry);
            }
        }
        if path.is_file() {
            files.push(ZipEntry {
                file_path: path.clone(),
                sub_path: None,
                content: None,
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
            let file_entries = read_zip_entries(file_entry.file_path.clone(), true)?;
            for mut file_entry in file_entries {
                if file_entry.sub_path.is_some() {
                    file_entry.sub_path = Some(format!(
                        "{}/{}",
                        file_entry.sub_path.clone().unwrap_or("".to_string()),
                        file_entry.sub_path.clone().unwrap_or("".to_string())
                    ));
                }
                files_to_compress.push(file_entry);
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
        if file_entry.content.is_some() {
            buffer
                .write_all(file_entry.content.clone().unwrap().as_bytes())
                .map_err(|e| {
                    AppError::new(
                        "Zip:Write",
                        eyre!(format!(
                            "Path: {:?}, Error: {}",
                            file_entry.file_path,
                            e.to_string()
                        )),
                    )
                })?;
        } else {
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
        }

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

pub fn loop_through_properties(data: &mut Map<String, Value>, properties: Vec<String>) {
    // Iterate over each key-value pair in the JSON object
    for (key, value) in data.iter_mut() {
        // Perform actions based on the property key or type
        match value {
            Value::Object(sub_object) => {
                // If the value is another object, recursively loop through its properties
                loop_through_properties(sub_object, properties.clone());
            }
            _ => {
                if properties.contains(&key.to_string()) {
                    *value = json!("***");
                }
            }
        }
    }
}

pub fn open_json_and_replace(path: &str, properties: Vec<String>) -> Result<Value, AppError> {
    match std::fs::File::open(path) {
        Ok(file) => {
            let reader = std::io::BufReader::new(file);
            let mut data: serde_json::Map<String, Value> = serde_json::from_reader(reader)
                .map_err(|e| AppError::new("Logger", eyre!(e.to_string())))
                .expect("Could not read auth.json");
            loop_through_properties(&mut data, properties.clone());
            Ok(json!(data))
        }
        Err(_) => Err(AppError::new(
            "Logger",
            eyre!("Could not open file at path: {}", path),
        )),
    }
}
