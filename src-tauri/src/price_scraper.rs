use crate::structs::GlobleError;
use crate::wfm_client;
use crate::{helper, logger};
use once_cell::sync::Lazy;
use polars::prelude::*;
use reqwest::{Client, Method, Url};
use serde_json::{json, Value};

use std::path::Path;
use std::sync::Mutex;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufWriter,
};
extern crate chrono;
use chrono::Duration;

pub static CSV_PATH: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("".to_string()));
pub static CSV_BACKOP_PATH: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("".to_string()));

/// Returns the path to the CSV file as a string.
/// The path is stored in a global variable that is locked to prevent concurrent access.
pub fn get_csv_path() -> String {
    let locked_csv_path = CSV_PATH.lock().unwrap();
    locked_csv_path.clone()
}

/// Returns the path to the CSV backup file as a string.
/// The path is stored in a global variable that is locked to prevent concurrent access.
pub fn get_csv_backup_path() -> String {
    let locked_csv_backup_path = CSV_BACKOP_PATH.lock().unwrap();
    locked_csv_backup_path.clone()
}

/// Returns a vector of strings representing the dates of the last `x` days, including today.
/// The dates are formatted as "YYYY-MM-DD".
fn last_x_days(x: i64) -> Vec<String> {
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

/// Reads the price history data from a CSV file and returns it as a DataFrame.
/// If the backup file is available, it is used instead of the main file.
pub fn get_price_historys() -> Result<DataFrame, PolarsError> {
    // Try to read from "allItemDataBackup.csv", and if it fails, read from "allItemData.csv".
    let file = File::open(get_csv_path()).or_else(|_| File::open(get_csv_backup_path()))?;

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

/// Returns true if the given vector of item data is valid for price scraping, false otherwise.
/// A valid item data vector must have at least one element, and the first element must have either 3 or 6 keys.
/// The first element must also have a "mod_rank" key.
fn is_valid_price_data(name: &str, item_datas: &Vec<Value>) -> bool {
    if item_datas.len() == 0 {
        return false;
    }
    // Check if the first element has a "mod_rank" key
    let is_mod = match item_datas[0].get("mod_rank") {
        Some(_mod_rank) => true,
        None => false,
    };

    if is_mod && item_datas.len() == 6 {
        return true;
    }
    if !is_mod && item_datas.len() == 3 {
        return true;
    }
    return false;
}

/// Returns a map of item names to their corresponding IDs, based on the `items` list.
/// The map is represented as a `HashMap` with `String` keys and values.
async fn get_items_map_url_map(
) -> Result<(HashMap<String, String>, HashMap<String, String>), GlobleError> {
    let items = wfm_client::get_tradable_items("").await?;

    let item_map_url: std::collections::HashMap<String, String> = items
        .iter()
        .map(|item| (item.item_name.clone(), item.url_name.clone()))
        .collect();
    let item_map_id: std::collections::HashMap<String, String> = items
        .iter()
        .map(|item| (item.url_name.clone(), item.id.clone()))
        .collect();
    Ok((item_map_url, item_map_id))
}

pub async fn generate(platform: &str, days: i64) -> Result<(), GlobleError> {
    logger::debug_con(
        "PriceScraper:generate",
        format!(
            "Generating csv file for platform: {}, for {} days.",
            platform, days
        )
        .as_str(),
    );
    let csv_path_str = get_csv_path();
    let csv_backop_path_str = get_csv_backup_path();
    let csv_path: &Path = Path::new(&csv_path_str);
    let csv_backop_path = Path::new(&csv_backop_path_str);
    if csv_path.exists() {
        logger::debug_con(
            "PriceScraper:generate",
            format!("Backuping csv file: {}", csv_path_str).as_str(),
        );
        fs::copy(csv_path, csv_backop_path)?;
    }
    let last_days = last_x_days(days).clone();
    let mut dataframes: Vec<DataFrame> = Vec::new();
    let (url_map, id_map) = get_items_map_url_map().await?;

    let mut found_data = 0;

    for day in last_days.clone() {
        if found_data >= 7 {
            // Should only get 7 days of data
            continue;
        }
        helper::send_message_to_window(
            "price_scraper_update_progress",
            Some(json!({"current": found_data, "total": days, "day": day})),
        );
        // Get the price data for the day for all items
        let items = get_price_by_day(platform, &day).await;
        match items {
            Ok(items) => {
                found_data += 1;
                logger::info_con(
                    "PriceScraper:generate",
                    format!("Getting data for day: {}", day).as_str(),
                );

                if let Value::Object(map) = &items {
                    for (item_name, item_data_list) in map {
                        if let Value::Array(array) = item_data_list {
                            if !is_valid_price_data(&item_name, array) {
                                // println!("Invalid price data for item: {}", item_name);
                                continue;
                            }

                            // Get the url_name and id for the item
                            let url_name = url_map
                                .get(item_name)
                                .unwrap_or(&"not_found".to_string())
                                .clone();

                            // Get the id for the item
                            let id = id_map
                                .get(&url_name)
                                .unwrap_or(&"not_found".to_string())
                                .clone();

                            let name_vec: Vec<Option<String>> = array
                                .iter()
                                .map(|_item_data| Some(url_name.clone()))
                                .collect();

                            let id_vec: Vec<Option<String>> =
                                array.iter().map(|_item_data| Some(id.clone())).collect();

                            let order_type_vec: Vec<Option<String>> = array
                                .iter()
                                .map(|item_data| {
                                    item_data
                                        .get("order_type")
                                        .and_then(|v| v.as_str())
                                        .map(String::from)
                                })
                                .collect();

                            let volume_vec: Vec<Option<i64>> = array
                                .iter()
                                .map(|item_data| {
                                    item_data
                                        .get("volume")
                                        .and_then(|v| v.as_i64())
                                        .map(i64::from)
                                })
                                .collect();

                            let datetime_vec: Vec<Option<String>> = array
                                .iter()
                                .map(|item_data| {
                                    item_data
                                        .get("datetime")
                                        .and_then(|v| v.as_str())
                                        .map(String::from)
                                })
                                .collect();

                            let max_price_vec: Vec<Option<f64>> = array
                                .iter()
                                .map(|item_data| {
                                    item_data.get("max_price").and_then(|v| v.as_f64())
                                })
                                .collect();

                            let min_price_vec: Vec<Option<f64>> = array
                                .iter()
                                .map(|item_data| {
                                    item_data.get("min_price").and_then(|v| v.as_f64())
                                })
                                .collect();

                            let avg_price_vec: Vec<Option<f64>> = array
                                .iter()
                                .map(|item_data| {
                                    item_data.get("avg_price").and_then(|v| v.as_f64())
                                })
                                .collect();

                            let mod_rank_vec: Vec<Option<f64>> = array
                                .iter()
                                .map(|item_data| item_data.get("mod_rank").and_then(|v| v.as_f64()))
                                .collect();

                            let median_vec: Vec<Option<f64>> = array
                                .iter()
                                .map(|item_data| item_data.get("median").and_then(|v| v.as_f64()))
                                .collect();

                            let df = DataFrame::new_no_checks(vec![
                                Series::new("name", name_vec),
                                Series::new("datetime", datetime_vec),
                                Series::new("order_type", order_type_vec),
                                Series::new("volume", volume_vec),
                                Series::new("min_price", min_price_vec),
                                Series::new("max_price", max_price_vec),
                                Series::new("avg_price", avg_price_vec),
                                Series::new("mod_rank", mod_rank_vec),
                                Series::new("median", median_vec),
                                Series::new("item_id", id_vec),
                            ]);
                            // dump_dataframe(&mut df, format!("{}.csv", item_name).as_str())?;

                            let df: DataFrame = df
                                .clone()
                                .lazy()
                                .fill_nan(lit(0.0).alias("max_price"))
                                .fill_nan(lit(0.0).alias("min_price"))
                                .with_column((col("max_price") - col("min_price")).alias("range"))
                                .collect()?;

                            // Filter out items that are mod_rank 0.
                            let df = df
                                .clone()
                                .lazy()
                                .filter(col("mod_rank").neq(0).or(col("mod_rank").is_null()))
                                .collect()?;
                            // dump_dataframe(&mut df, format!("{} {}.csv", day, item_name).as_str())?;
                            dataframes.push(df);
                        }
                    }
                }
            }
            Err(_e) => {
                logger::error_con(
                    "PriceScraper:generate",
                    format!("Error getting price data for day: {}", day).as_str(),
                );
            }
        }
    }
    logger::info_con(
        "PriceScraper:generate",
        format!(
            "Finished getting price data for all days. Merging dataframes... {:?}",
            dataframes.len()
        )
        .as_str(),
    );
    let mut full_df = helper::merge_dataframes(dataframes)?;
    helper::send_message_to_window(
        "price_scraper_update_complete",
        Some(json!({"total_items": full_df.height()})),
    );
    logger::log_dataframe(&mut full_df, "price_scraper_full_df.csv");

    // Group by name and get the average price
    let mut group_by_name = full_df
        .clone()
        .lazy()
        .groupby(&["name"])
        .agg(&[
            // List the other columns you want to average
            col("name").count().alias("name_count"),
        ])
        .collect()?;
    logger::log_dataframe(&mut group_by_name, "price_scraper_group_by_name.csv");

    // Get the names of the items that are popular

    let mut popular_items = group_by_name
        .clone()
        .lazy()
        .filter(col("name_count").gt_eq(21))
        .collect()?;
    logger::log_dataframe(&mut popular_items, "price_scraper_popular_items.csv");

    // Filter out items that are not popular and sort by name
    let popular_items_s = popular_items.column("name")?;
    let mask = full_df.column("name")?.is_in(&popular_items_s)?;
    let filtered_df = full_df.filter(&mask)?;
    // Sort by name
    let mut filtered_df = filtered_df
        .lazy()
        .sort(
            "name",
            SortOptions {
                descending: false,
                nulls_last: false,
                multithreaded: false,
            },
        )
        .collect()?;
    logger::log_dataframe(&mut filtered_df, "price_scraper_pricehistory.csv");

    // Cerate a csv file with the sorted DataFrame of price data
    let output_file: File = File::create(csv_path)?;
    let writer = BufWriter::new(output_file);
    // Write the DataFrame to a CSV file
    CsvWriter::new(writer).finish(&mut filtered_df)?;

    // Delete the backup file if it exists
    if csv_backop_path.exists() {
        fs::remove_file(csv_backop_path)?;
    }
    Ok(())
}
