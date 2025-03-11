use entity::{
    stock::{
        item::{create::CreateStockItem, stock_item},
        riven::{create::CreateStockRiven, stock_riven},
    },
    sub_type::SubType,
    transaction::transaction::TransactionType,
    wish_list::{create::CreateWishListItem, wish_list},
};
use eyre::eyre;
use regex::Regex;
use serde_json::{json, Map, Value};
use service::{StockItemMutation, StockRivenMutation, TransactionMutation, WishListMutation};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use tauri::{Manager, State};

use zip::{write::FileOptions, CompressionMethod, ZipWriter};

use crate::{
    app::client::AppState,
    cache::client::CacheClient,
    notification::client::NotifyClient,
    qf_client::client::QFClient,
    utils::{
        enums::ui_events::{UIEvent, UIOperationEvent},
        modules::{error::AppError, states},
    },
    wfm_client::{client::WFMClient, enums::order_type::OrderType, types::order::Order},
    APP, DATABASE,
};

pub static APP_PATH: &str = "dev.kenya.quantframe";

#[derive(Clone, Debug)]
pub struct ZipEntry {
    pub file_path: PathBuf,
    pub sub_path: Option<String>,
    pub content: Option<String>,
    pub include_dir: bool,
}

pub fn add_metric(key: &str, value: &str) {
    let key = key.to_string();
    let value = value.to_string();
    tauri::async_runtime::spawn({
        async move {
            // Create a new instance of the QFClient and store it in the app state
            let qf_handle = APP.get().expect("failed to get app handle");
            let qf_state: State<Arc<Mutex<QFClient>>> = qf_handle.state();
            let qf = qf_state.lock().expect("failed to lock app state").clone();
            qf.analytics().add_metric(&key, &value);
        }
    });
}
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

pub fn dose_app_exist() -> bool {
    let app = APP.get().unwrap();
    let local_path = match app.path().local_data_dir() {
        Ok(val) => val,
        Err(_) => {
            return false;
        }
    };
    let app_path = local_path.join(APP_PATH);
    app_path.exists()
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

pub fn remove_special_characters(input: &str) -> String {
    // Define the pattern for special characters except _ and space
    let pattern = Regex::new("[^a-zA-Z0-9_ ]").unwrap();

    // Replace special characters with empty string
    let result = pattern.replace_all(input, "");

    result.into_owned()
}

pub fn get_local_data_path() -> PathBuf {
    let app = APP.get().unwrap();
    let local_path = match app.path().local_data_dir() {
        Ok(val) => val,
        Err(_) => {
            panic!("Could not find local data path");
        }
    };
    local_path
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

pub fn match_pattern(
    input: &str,
    regex: Vec<String>,
) -> Result<(bool, Vec<Option<String>>), regex::Error> {
    for regex in regex {
        let re: Regex = Regex::new(&regex)?;
        if let Some(captures) = re.captures(input) {
            let mut result: Vec<Option<String>> = vec![];
            for i in 1..captures.len() {
                let group = captures.get(i).map(|m| m.as_str().to_string());
                let group: Option<String> =
                    group.map(|s| s.chars().filter(|c| c.is_ascii()).collect());
                result.push(group);
            }
            return Ok((true, result));
        }
    }
    Ok((false, vec![]))
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
            let sub_file_entries = read_zip_entries(file_entry.file_path.clone(), true)?;
            for mut sub_file_entry in sub_file_entries {
                if sub_file_entry.sub_path.is_some() {
                    sub_file_entry.sub_path = Some(format!(
                        "{}/{}",
                        file_entry.sub_path.clone().unwrap_or("".to_string()),
                        sub_file_entry.sub_path.clone().unwrap_or("".to_string())
                    ));
                }
                files_to_compress.push(sub_file_entry);
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
            } else {
                args_map.insert(part.to_string(), "".to_string());
            }
        }
    }

    args_map
}

pub fn validate_args(
    args: &str,
    requirements: Vec<&str>,
) -> Result<HashMap<String, String>, AppError> {
    let args_map = parse_args_from_string(args);

    for req in requirements {
        // Split the requirement to check for conditional requirements
        let parts: Vec<&str> = req.split(':').collect();
        if parts.len() == 1 {
            // Simple required argument
            let arg = parts[0];
            if !args_map.contains_key(arg) {
                return Err(AppError::new(
                    "ValidateArgs",
                    eyre!(format!("Missing required argument: {}", arg)),
                ));
            }
        } else if parts.len() == 2 {
            // Conditional required arguments
            let conditional_parts: Vec<&str> = parts[1].split('|').collect();
            if conditional_parts.len() == 2 {
                let (value, additional_args_str) = (conditional_parts[0], conditional_parts[1]);
                let additional_args: Vec<&str> = additional_args_str.split_whitespace().collect();

                if let Some(arg_value) = args_map.get(parts[0]) {
                    if arg_value == value {
                        for additional_arg in additional_args {
                            if !args_map.contains_key(additional_arg) {
                                return Err(AppError::new(
                                    "ValidateArgs",
                                    eyre!(format!(
                                        "Missing required argument due to {}={}: {}",
                                        parts[0], value, additional_arg
                                    )),
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(args_map)
}

pub fn is_match(
    input: &str,
    to_match: &str,
    ignore_case: bool,
    remove_string: Option<&String>,
) -> bool {
    let mut input = input.to_string();
    if let Some(remove_string) = remove_string {
        input = input.replace(remove_string, "");
    }
    if ignore_case {
        input.to_lowercase() == to_match.to_lowercase()
    } else {
        input == to_match
    }
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

pub async fn progress_wfm_order(
    url: &str,
    sub_type: Option<SubType>,
    quantity: i64,
    operation: OrderType,
    need_update: bool,
    from: &str,
) -> Result<(String, Option<Order>), AppError> {
    let wfm = states::wfm_client()?;
    let notify = states::notify_client()?;
    // Process the order on WFM
    match wfm
        .orders()
        .progress_order(
            &url,
            sub_type.clone(),
            quantity,
            operation.clone(),
            need_update,
        )
        .await
    {
        Ok((operation, order)) => {
            if operation == "Deleted" && order.is_some() {
                add_metric("WFM_OrderDeleted", from);
                notify.gui().send_event_update(
                    UIEvent::UpdateOrders,
                    UIOperationEvent::Delete,
                    Some(json!({ "id": order.clone().unwrap().id })),
                );
            } else if operation == "Updated" {
                add_metric("WFM_OrderUpdated", from);
                notify.gui().send_event_update(
                    UIEvent::UpdateOrders,
                    UIOperationEvent::CreateOrUpdate,
                    Some(json!(order)),
                );
            }
            return Ok((operation, order));
        }
        Err(e) => {
            return Err(e);
        }
    }
}

pub async fn progress_transaction(
    transaction: &mut entity::transaction::transaction::Model,
    from: &str,
) -> Result<entity::transaction::transaction::Model, AppError> {
    let conn = DATABASE.get().unwrap();
    let notify = states::notify_client()?;
    let qf = states::qf_client()?;
    match TransactionMutation::create(conn, &transaction).await {
        Ok(inserted) => {
            add_metric("Transaction_Create", from);
            notify.gui().send_event_update(
                UIEvent::UpdateTransaction,
                UIOperationEvent::CreateOrUpdate,
                Some(json!(inserted)),
            );
            transaction.id = inserted.id;
        }
        Err(e) => {
            return Err(AppError::new_db("TransactionCreate", e));
        }
    };

    // Add the transaction to the QuantFrame analytics stars
    match qf.transaction().create_transaction(&transaction).await {
        Ok(_) => {}
        Err(e) => {
            return Err(e);
        }
    }
    Ok(transaction.clone())
}

pub async fn progress_wish_item(
    entity: &mut CreateWishListItem,
    validate_by: &str,
    user_name: &str,
    operation: OrderType,
    options: Vec<String>,
    from: &str,
) -> Result<(wish_list::Model, Vec<String>), AppError> {
    let conn = DATABASE.get().unwrap();
    let mut response = vec![];
    let cache = states::cache()?;
    let notify = states::notify_client()?;
    if operation == OrderType::Sell {
        return Err(AppError::new(
            "ProgressWishItem",
            eyre!("Invalid operation"),
        ));
    }

    // Validate the stock item
    match cache
        .tradable_items()
        .validate_create_wish_item(entity, validate_by)
    {
        Ok(_) => {}
        Err(e) => {
            return Err(e);
        }
    };

    //Get stock item from the entity
    let wish_item = entity.to_model();

    // Progress the stock item based on the operation

    match WishListMutation::bought_by_url_and_sub_type(
        conn,
        wish_item.wfm_url.as_str(),
        wish_item.sub_type.clone(),
        wish_item.quantity,
    )
    .await
    {
        Ok((operation, item)) => {
            response.push(format!("WishItem_{}", operation));
            if operation == "NotFound" {
                if !options.contains(&"WishContinueOnError".to_string()) {
                    return Err(AppError::new(
                        "WishItemCreate",
                        eyre!(format!(
                            "Wish Item not found: {} {:?}",
                            entity.wfm_url, entity.sub_type
                        )),
                    ));
                }
            } else if operation == "Deleted" {
                notify.gui().send_event_update(
                    UIEvent::UpdateWishList,
                    UIOperationEvent::Delete,
                    Some(json!({ "id": item.unwrap().id })),
                );
            } else if operation == "Updated" {
                notify.gui().send_event_update(
                    UIEvent::UpdateWishList,
                    UIOperationEvent::CreateOrUpdate,
                    Some(json!(item)),
                );
            }
            add_metric("Wish_ItemBought", from);
            response.push("WishItem_Bought".to_string());
        }
        Err(e) => {
            response.push("WishItemDbError".to_string());
            return Err(AppError::new("WishItemCreate", eyre!(e)));
        }
    }

    // Process the order on WFM
    match progress_wfm_order(
        entity.wfm_url.as_str(),
        entity.sub_type.clone(),
        entity.quantity,
        OrderType::Buy,
        true,
        from,
    )
    .await
    {
        Ok((operation, _)) => {
            response.push(format!("WFM_{}", operation));
        }
        Err(e) => {
            response.push("WFMOrderError".to_string());
            if !options.contains(&"WFMContinueOnError".to_string()) {
                return Err(e);
            }
        }
    }

    if entity.bought.unwrap_or(0) <= 0 {
        return Ok((wish_item, response));
    }

    // Add Transaction to the database
    let mut transaction = wish_item.to_transaction(
        user_name,
        entity.tags.clone(),
        entity.quantity,
        entity.bought.unwrap_or(0),
        TransactionType::Purchase,
    );

    match progress_transaction(&mut transaction, from).await {
        Ok(_) => {}
        Err(e) => {
            response.push("TransactionDbError".to_string());
            return Err(e);
        }
    };
    return Ok((wish_item, response));
}

pub async fn progress_stock_item(
    entity: &mut CreateStockItem,
    validate_by: &str,
    user_name: &str,
    operation: OrderType,
    options: Vec<String>,
    from: &str,
) -> Result<(stock_item::Model, Vec<String>), AppError> {
    let conn = DATABASE.get().unwrap();
    let mut response = vec![];
    let cache = states::cache()?;
    let notify = states::notify_client()?;
    // Validate the stock item
    match cache
        .tradable_items()
        .validate_create_item(entity, validate_by)
    {
        Ok(_) => {}
        Err(e) => {
            return Err(e);
        }
    };

    //Get stock item from the entity
    let stock = entity.to_model();

    // Progress the stock item based on the operation
    if operation == OrderType::Sell {
        match StockItemMutation::sold_by_url_and_sub_type(
            conn,
            &entity.wfm_url,
            entity.sub_type.clone(),
            entity.quantity,
        )
        .await
        {
            Ok((operation, item)) => {
                response.push(format!("StockItem_{}", operation));
                if operation == "NotFound" {
                    if !options.contains(&"StockContinueOnError".to_string()) {
                        return Err(AppError::new(
                            "StockItemSell",
                            eyre!(format!(
                                "Stock Item not found: {} {:?}",
                                entity.wfm_url, entity.sub_type
                            )),
                        ));
                    }
                } else if operation == "Deleted" {
                    notify.gui().send_event_update(
                        UIEvent::UpdateStockItems,
                        UIOperationEvent::Delete,
                        Some(json!({ "id": item.unwrap().id })),
                    );
                } else if operation == "Updated" {
                    notify.gui().send_event_update(
                        UIEvent::UpdateStockItems,
                        UIOperationEvent::CreateOrUpdate,
                        Some(json!(item)),
                    );
                }
                add_metric(format!("StockItem_{}", operation).as_str(), from);
            }
            Err(e) => {
                response.push("StockDbError".to_string());
                return Err(AppError::new("StockItemSell", eyre!(e)));
            }
        }
    } else if operation == OrderType::Buy {
        match StockItemMutation::add_item(conn, stock.clone()).await {
            Ok(inserted) => {
                let rep = "StockItem_Created".to_string();
                response.push(rep.clone());
                notify.gui().send_event_update(
                    UIEvent::UpdateStockItems,
                    UIOperationEvent::CreateOrUpdate,
                    Some(json!(inserted)),
                );
                add_metric(rep.as_str(), from);
            }
            Err(e) => {
                response.push("StockItem_DbError".to_string());
                return Err(AppError::new("StockItemCreate", eyre!(e)));
            }
        }
    } else {
        return Err(AppError::new(
            "ProgressStockItem",
            eyre!("Invalid operation"),
        ));
    }

    // Process the order on WFM
    // match progress_wfm_order(
    //     notify,
    //     wfm,
    //     entity.wfm_url.as_str(),
    //     entity.sub_type.clone(),
    //     entity.quantity,
    //     operation.clone(),
    //     operation == OrderType::Sell,
    //     from,
    // )
    // .await
    // {
    //     Ok((operation, _)) => {
    //         response.push(format!("WFM_{}", operation));
    //     }
    //     Err(e) => {
    //         response.push("WFM_Error".to_string());
    //         if !options.contains(&"WFMContinueOnError".to_string()) {
    //             return Err(e);
    //         }
    //     }
    // }

    if entity.bought.unwrap_or(0) <= 0 {
        return Ok((stock, response));
    }

    // Add Transaction to the database
    let transaction_type = if operation == OrderType::Buy {
        TransactionType::Purchase
    } else {
        TransactionType::Sale
    };
    let mut transaction = stock.to_transaction(
        user_name,
        entity.tags.clone(),
        entity.quantity,
        entity.bought.unwrap_or(0),
        transaction_type,
    );

    match progress_transaction(&mut transaction, from).await {
        Ok(_) => {}
        Err(e) => {
            response.push("Transaction_DbError".to_string());
            return Err(e);
        }
    };
    return Ok((stock, response));
}

pub async fn progress_stock_riven(
    entity: &mut CreateStockRiven,
    validate_by: &str,
    user_name: &str,
    operation: OrderType,
    from: &str,
) -> Result<(stock_riven::Model, Vec<String>), AppError> {
    let conn = DATABASE.get().unwrap();
    let mut response = vec![];
    let cache = states::cache()?;
    let qf = states::qf_client()?;
    let notify = states::notify_client()?;
    let wfm = states::wfm_client()?;
    // Validate the stock item
    match cache.riven().validate_create_riven(entity, validate_by) {
        Ok(_) => {}
        Err(e) => {
            return Err(e);
        }
    };

    //Get stock riven from the entity
    let stock = entity.to_model();

    // Progress the stock riven based on the operation
    if operation == OrderType::Sell && entity.stock_id.is_some() {
        // Delete the stock from the database
        match StockRivenMutation::delete(conn, entity.stock_id.unwrap()).await {
            Ok(_) => {
                response.push("StockRiven_Deleted".to_string());
                add_metric("StockRiven_Deleted", from);
                notify.gui().send_event_update(
                    UIEvent::UpdateStockRivens,
                    UIOperationEvent::Delete,
                    Some(json!({ "id": entity.stock_id })),
                );
            }
            Err(e) => return Err(AppError::new("StockItemSell", eyre!(e))),
        }
    } else if operation == OrderType::Buy {
        match StockRivenMutation::create(conn, stock.clone()).await {
            Ok(inserted) => {
                add_metric("StockRiven_Create", from);
                response.push("StockRivenAdd".to_string());
                notify.gui().send_event_update(
                    UIEvent::UpdateStockRivens,
                    UIOperationEvent::CreateOrUpdate,
                    Some(json!(inserted)),
                );
            }
            Err(e) => {
                response.push("StockDbError".to_string());
                let err = AppError::new_db("ProgressStockRiven", e);
                return Err(err);
            }
        }
    } else {
        return Err(AppError::new(
            "ProgressStockRiven",
            eyre!("Invalid operation"),
        ));
    }

    // Process the action on WFM
    if operation == OrderType::Sell && entity.wfm_order_id.is_some() {
        let id = entity.wfm_order_id.clone().unwrap();
        match wfm.auction().delete(&id).await {
            Ok(_) => {
                add_metric("WFM_RivenDeleted", from);
                response.push("WFM_RivenDeleted".to_string());
                notify.gui().send_event_update(
                    UIEvent::UpdateAuction,
                    UIOperationEvent::Delete,
                    Some(json!({ "id": id })),
                );
            }
            Err(e) => {
                if e.cause().contains("app.form.not_exist") {
                    response.push("WFMRivenNotExist".to_string());
                }
                response.push("WFMRivenError".to_string());
            }
        }
    }

    if entity.bought.unwrap_or(0) <= 0 {
        return Ok((stock, response));
    }

    // Add Transaction to the database
    let transaction_type = if operation == OrderType::Buy {
        TransactionType::Purchase
    } else {
        TransactionType::Sale
    };
    let mut transaction =
        stock.to_transaction(user_name, entity.bought.unwrap_or(0), transaction_type);

    match TransactionMutation::create(conn, &transaction).await {
        Ok(inserted) => {
            add_metric("Transaction_RivenCreate", from);
            response.push("TransactionCreated".to_string());
            notify.gui().send_event_update(
                UIEvent::UpdateTransaction,
                UIOperationEvent::CreateOrUpdate,
                Some(json!(inserted)),
            );
            transaction.id = inserted.id;
        }
        Err(e) => {
            response.push("TransactionDbError".to_string());
            return Err(AppError::new_db("StockItemCreate", e));
        }
    };
    // Add the transaction to the QuantFrame analytics stars
    match qf.transaction().create_transaction(&transaction).await {
        Ok(_) => {}
        Err(e) => {
            response.push("TransactionAnalyticsError".to_string());
            return Err(e);
        }
    }
    return Ok((stock, response));
}

// pub fn read_json_file<T: DeserializeOwned>(path: &PathBuf) -> Result<T, AppError> {
//     // Check if the file exists
//     if !path.exists() {
//         return Err(AppError::new(
//             "ReadJsonFile",
//             eyre!(format!("File does not exist: {:?}", path.to_str())),
//         ));
//     }

//     let file = File::open(path).map_err(|e| {
//         AppError::new(
//             "ReadJsonFile",
//             eyre!(format!("Could not open file: {}", e.to_string())),
//         )
//     })?;
//     let reader = io::BufReader::new(file);
//     let data: Value = serde_json::from_reader(reader).map_err(|e| {
//         AppError::new(
//             "ReadJsonFile",
//             eyre!(format!("Could not read file: {}", e.to_string())),
//         )
//     })?;
//     match serde_json::from_value(data.clone()) {
//         Ok(payload) => Ok(payload),
//         Err(e) => {
//             return Err(AppError::new(
//                 "Helper:ReadJsonFile",
//                 eyre!(format!("Could not parse payload: {}", e)),
//             ));
//         }
//     }
// }
pub fn calculate_average_of_top_lowest_prices(
    prices: Vec<i64>,          // The list of prices to consider
    limit_to: i64,             // Limit the number of auctions to consider
    threshold_percentage: f64, // The threshold percentage to filter prices
) -> i64 {
    if prices.is_empty() {
        return -1;
    }

    // Returning an Option<i64> in case there are no valid prices
    // Get the top `limit_to` lowest starting prices
    let mut top_price: Vec<i64> = prices.iter().cloned().take(limit_to as usize).collect();

    // Find the maximum price in the top lowest prices
    let max_price = *top_price.iter().max().unwrap_or(&0);

    // Find the minimum price in the top lowest prices
    let min_price = *top_price.iter().min().unwrap_or(&0);

    // Calculate `threshold_percentage` of the maximum price
    let threshold = max_price as f64 * (1.0 - threshold_percentage);

    // Remove the minimum price if it is less than the threshold
    if min_price < threshold as i64 {
        top_price.retain(|&price| price != min_price);
    }

    // Ensure we have valid prices before calculating the average
    if top_price.is_empty() {
        return -1;
    }
    // Calculate and return the average price
    top_price.iter().sum::<i64>() / top_price.len() as i64
}

pub fn pad_end(s: &str, len: usize, pad_char: char) -> String {
    let padding_needed = len.saturating_sub(s.len());
    format!("{}{}", s, pad_char.to_string().repeat(padding_needed))
}

pub fn pad_start(s: &str, len: usize, pad_char: char) -> String {
    let padding_needed = len.saturating_sub(s.len());
    format!("{}{}", pad_char.to_string().repeat(padding_needed), s)
}
