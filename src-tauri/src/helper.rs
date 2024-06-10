use eyre::eyre;
use serde_json::{json, Map, Value};
use std::{
    collections::HashMap, fs::{self, File}, io::{self, Read, Write}, path::{Path, PathBuf}
};

use zip::{write::FileOptions, CompressionMethod, ZipWriter};

use crate::{utils::modules::error::AppError};

pub fn get_app_storage_path() -> PathBuf {
    let local_path = match tauri::api::path::local_data_dir() {
        Some(val) => val,
        None => {
            panic!("Could not find app path");
        }
    };
    let app_path = local_path.join("dev.kenya.quantframe");
    if !app_path.exists() {
        fs::create_dir_all(app_path.clone()).unwrap();
    }
    app_path
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

pub fn parse_args_from_string(args: &str) -> HashMap<String, String> {
    let mut args_map = HashMap::new();
    let mut parts = args.split_whitespace().peekable();

    while let Some(part) = parts.next() {
        if part.starts_with("--") {
            if let Some(value) = parts.peek() {
                if !value.starts_with("--") {
                    args_map.insert(part.to_string(), value.to_string());
                    parts.next();
                }
            }
        }
    }

    args_map
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
