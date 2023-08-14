use crate::structs::GlobleError;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use polars::prelude::*;
use reqwest::{Client, Method, Url};
use serde::Deserialize;
use serde_json::Value;
use crate::wfm_client;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufWriter, Cursor},
    path::PathBuf,
};
extern crate chrono;
use chrono::Duration;

pub static CSV_PATH: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("".to_string()));
pub static CSV_BACKOP_PATH: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("".to_string()));
pub static WINDOW: Lazy<Mutex<Option<Window>>> = Lazy::new(|| Mutex::new(None));


/// Returns a vector of strings representing the last `x` days, including today.
/// Each string is formatted as "YYYY-MM-DD".
fn last_x_days(x: i64) -> Vec<String> {
    let today = chrono::Local::now().naive_local();
    (0..x)
        .rev()
        .map(|i| {
            (today - Duration::days(i + 1))
                .format("%Y-%m-%d")
                .to_string()
        })
        .collect()
}

/// Reads the price history data from a CSV file and returns it as a DataFrame.
/// If the backup file is available, it is used instead of the main file.
pub fn get_price_historys(&self) -> Result<DataFrame, PolarsError> {
    let csv_path = CSV_PATH.lock().unwrap();
    let csv_backop_path = CSV_BACKOP_PATH.lock().unwrap();
    // Try to read from "allItemDataBackup.csv", and if it fails, read from "allItemData.csv".
    let file = File::open(csv_backop_path).or_else(|_| File::open(csv_path))?;

    // Parse the CSV file into a DataFrame
    CsvReader::new(file)
        .infer_schema(None)
        .has_header(true)
        .finish()
}

/// Returns a JSON object containing price data for the given platform and day.
/// The `platform` argument should be one of "pc", "ps4", or "xb1".
/// The `day` argument should be a string in the format "YYYY-MM-DD".
/// If the request fails, returns a `GlobleError` with information about the error.
async fn get_price_by_day(platform: &str, day: &str) -> Result<Value, GlobleError> {
    let mut url = format!("https://relics.run/history/price_history_{}.json", day);
    if platform != "pc" {
        url = format!(
            "https://relics.run/history/{}/price_history_{}.json",
            platform, day
        );
    }
    let client = Client::new();
    let request = client.request(Method::GET, Url::parse(&url).unwrap());
    let response = request.send().await;
    if let Err(e) = response {
        return Err(GlobleError::ReqwestError(e));
    }
    let response_data = response.unwrap();
    let status = response_data.status();
    if status == 429 {
        // Sleep for 3 second
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        return Err(GlobleError::TooManyRequests(
            "Too Many Requests".to_string(),
        ));
    }
    if status != 200 {
        return Err(GlobleError::HttpError(
            status,
            response_data.text().await.unwrap(),
            url.to_string(),
        ));
    }
    let response = response_data.json::<Value>().await.unwrap();
    Ok(response)
}

/// Returns the number of keys in the given JSON object value.
/// If the value is not an object, returns 0.
fn key_count(val: &Value) -> i32 {
    if let Value::Object(map) = val {
        return map.len() as i32;
    } else {
        return 0;
    }
}

/// Returns true if the given vector of item data is valid for price scraping, false otherwise.
/// A valid item data vector must have at least one element, and the first element must have either 3 or 6 keys.
/// The first element must also have a "mod_rank" key.
fn is_valid_price_data(item_datas: &Vec<Value>) -> bool {
    if item_datas.len() == 0 {
        return false;
    }
    match item_datas[0].get("mod_rank") {
        Some(_mod_rank) => {
            if key_count(&item_datas[0]) == 6 || key_count(&item_datas[0]) == 3 {
                return true;
            }
            return false;
        }
        None => {
            return true;
        }
    }
}

/// Returns a map of item names to their corresponding IDs, based on the `items` list.
/// The map is represented as a `HashMap` with `String` keys and values.
fn get_items_map_url_map() -> Result<Map<String, String>, GlobleError> {
    let items = wfm_client::get_tradable_items()?;

    // Filter items where url_name does not contain "relic"
    let filtered_items: Vec<Item> = items
        .into_iter()
        .filter(|item| !item.url_name.contains("relic"))
        .collect();

    let item_map: std::collections::HashMap<String, String> = 
        filtered_items.iter()
            .map(|item| (item.item_name.clone(), item.url_name.clone()))
            .collect();
    Ok(item_map)
}


pub async fn generate(platform: &str) -> Result<(), GlobleError> {
    let csv_path = Path::new(CSV_PATH.lock().unwrap()) ;
    let csv_backop_path = Path::new(CSV_BACKOP_PATH.lock().unwrap());
    if path.exists() {
        fs::copy(csv_path, csv_backop_path)?
    }
    let last_days = last_x_days(2).clone();
    let mut dataframes: Vec<DataFrame> = Vec::new();
    let url_map = get_items_map_url_map()?;

    for day in last_days.clone() {
        let items = get_price_by_day(platform, &day).await;
        match items {
            Ok(items) => {
                if let Value::Object(map) = &items {
                    for (item_name, item_data_list) in map {
                        println!("Item: {}, Day: {}", item_name, day);
                        if let Value::Array(array) = item_data_list {
                            if !is_valid_price_data(array) {
                                continue;
                            }

                            let name_vec: Vec<String> = array
                                .iter()
                                .filter_map(|name| url_map.get(&item_name))
                                .collect();

                            let order_type_vec: Vec<Option<String>> = array
                                .iter()
                                .map(|item_data| {
                                    item_data
                                        .get("order_type")
                                        .and_then(|v| v.as_str())
                                        .filter(|&s| s != "closed")
                                        .map(String::from)
                                })
                                .collect();

                            let volume_vec: Vec<i64> = array
                                .iter()
                                .filter_map(|item_data| {
                                    item_data
                                        .get("volume")
                                        .and_then(|v| v.as_i64())
                                        .map(i64::from)
                                })
                                .collect();

                            let datetime_vec: Vec<String> = array
                                .iter()
                                .filter_map(|item_data| {
                                    item_data
                                        .get("datetime")
                                        .and_then(|v| v.as_str())
                                        .map(String::from)
                                })
                                .collect();

                            let max_price_vec: Vec<i64> = array
                                .iter()
                                .filter_map(|item_data| {
                                    item_data
                                        .get("max_price")
                                        .and_then(|v| v.as_i64())
                                        .map(i64::from)
                                })
                                .collect();

                            let min_price_vec: Vec<i64> = array
                                .iter()
                                .filter_map(|item_data| {
                                    item_data
                                        .get("min_price")
                                        .and_then(|v| v.as_i64())
                                        .map(i64::from)
                                })
                                .collect();

                            println!(
                                "Item: {},Name Vec: {}, Datetime Vec: {}, Volume Vec: {}, OrderType Vec: {}, MinPrice Vec: {}, MaxPrice Vec: {}",
                                item_name,
                                name_vec.clone().len(),
                                datetime_vec.clone().len(),
                                volume_vec.clone().len(),
                                order_type_vec.clone().len(),
                                min_price_vec.clone().len(),
                                max_price_vec.clone().len()
                            );
                            let df = DataFrame::new_no_checks(vec![
                                Series::new("name", name_vec),
                                Series::new("datetime", datetime_vec),
                                Series::new("volume", volume_vec),
                                Series::new("order_type", order_type_vec),
                                Series::new("min_price", min_price_vec),
                                Series::new("max_price", max_price_vec),
                            ]);

                            let df: DataFrame = df
                                .clone()
                                .lazy()
                                .fill_nan(lit(0.0).alias("max_price"))
                                .fill_nan(lit(0.0).alias("min_price"))
                                .with_column((col("max_price") - col("min_price")).alias("range"))
                                .collect()?;
                            dataframes.push(df);
                        }
                    }
                }
            }
            Err(e) => {
                println!("{}: {:?}", day, e);
            }
        }
    }
    let mut big_df = dataframes[0].clone();
    for df in &dataframes[1..] {
        big_df = big_df.vstack(df)?;
    }
    let mut sorted_df = big_df
        .lazy()
        .sort(
            "volume",
            SortOptions {
                descending: true,
                nulls_last: false,
                multithreaded: false,
            },
        )
        .collect()?;

    // Cerate a csv file with the sorted DataFrame of price data
    let output_file: File = File::create(csv_path)?;
    let writer = BufWriter::new(output_file);
    // Write the DataFrame to a CSV file
    CsvWriter::new(writer).finish(df)?;

    // Delete the backup file if it exists
    if csv_backop_path.exists() {
        fs::remove_file(csv_backop_path)?;
    }
    Ok(())
}