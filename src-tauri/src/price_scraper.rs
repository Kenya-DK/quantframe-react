use crate::structs::GlobleError;
use polars::prelude::*;
use reqwest::{Client, Method, Url};
use serde::Deserialize;
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufWriter, Cursor},
    path::PathBuf,
};
extern crate chrono;
use chrono::Duration;

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

pub async fn generate(platform: &str) -> Result<(), GlobleError> {
    let last_days = last_x_days(2).clone();
    let mut dataframes: Vec<DataFrame> = Vec::new();
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
                                "Item: {}, datetime_vec: {}, volume_vec: {}, order_type_vec: {}, min_price_vec: {}, max_price_vec: {}",
                                item_name,
                                datetime_vec.clone().len(),
                                volume_vec.clone().len(),
                                order_type_vec.clone().len(),
                                min_price_vec.clone().len(),
                                max_price_vec.clone().len()
                            );
                            let df = DataFrame::new_no_checks(vec![
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
    dump_dataframe(&mut sorted_df, "big_df.csv")?;
    Ok(())
}

fn dump_dataframe(df: &mut DataFrame, name: &str) -> Result<(), GlobleError> {
    let mut log_path = PathBuf::from("logs");
    // Create the directory if it does not exist
    if !log_path.exists() {
        fs::create_dir_all(&log_path)?;
    }
    log_path.push(name);
    let output_file: File = File::create(log_path)?;
    let writer = BufWriter::new(output_file);
    // Write the DataFrame to a CSV file
    CsvWriter::new(writer).finish(df)?;

    println!("{} generated", name);
    Ok(())
}
