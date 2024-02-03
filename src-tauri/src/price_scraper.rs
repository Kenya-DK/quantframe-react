use crate::enums::LogLevel;
use crate::error::{AppError, ApiResult, ErrorApiResponse};
use crate::wfm_client::client::WFMClient;
use crate::{helper, logger};
use eyre::eyre;
use polars::prelude::*;
use reqwest::{Client, Method, Url};
use serde_json::{json, Value};
use std::path::Path;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufWriter,
};
extern crate chrono;

use crate::auth::AuthState;

// Structs for the Warframe Market API

#[derive(Clone)]
pub struct PriceScraper {
    csv_path: String,
    csv_backop_path: String,
    wfm: Arc<Mutex<WFMClient>>,
    auth: Arc<Mutex<AuthState>>,
}

impl PriceScraper {
    pub fn new(wfm: Arc<Mutex<WFMClient>>, auth: Arc<Mutex<AuthState>>) -> Self {
        PriceScraper {
            csv_path: helper::get_app_roaming_path()
                .join("price_data.csv")
                .to_str()
                .unwrap()
                .to_string(),
            csv_backop_path: helper::get_app_roaming_path()
                .join("price_data_backup.csv")
                .to_str()
                .unwrap()
                .to_string(),
            wfm,
            auth,
        }
    }
    /// Reads the price history data from a CSV file and returns it as a DataFrame.
    /// If the backup file is available, it is used instead of the main file.
    pub fn get_price_historys(&self) -> Result<DataFrame, AppError> {
        // Try to read from "allItemDataBackup.csv", and if it fails, read from "allItemData.csv".
        let file = File::open(&self.csv_path)
            .or_else(|_| File::open(&self.csv_backop_path))
            .map_err(|e| AppError::new("PriceScraper", eyre!("Error opening csv file: {}", e)))?;

        // Parse the CSV file into a DataFrame
        CsvReader::new(file)
            .infer_schema(None)
            .has_header(true)
            .finish()
            .map_err(|e| AppError::new("PriceScraper", eyre!(e.to_string())))
    }

    pub fn get_status(&self) -> Option<u128> {
        // Try to read from "allItemDataBackup.csv", and if it fails, read from "allItemData.csv".
        let file = File::open(&self.csv_path).or_else(|_| File::open(&self.csv_backop_path));
        match file {
            Ok(file) => Some(
                file.metadata()
                    .unwrap()
                    .modified()
                    .unwrap()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
            ),
            Err(_) => None,
        }
    }
    /// Returns a JSON object containing price data for the given platform and day.
    /// The `platform` argument should be one of "pc", "ps4", or "xb1".
    /// The `day` argument should be a string in the format "YYYY-MM-DD".
    /// If the request fails, returns a `AppError` with information about the error.
    async fn get_price_by_day(&self, platform: &str, day: &str) -> Result<ApiResult<Value>, AppError> {
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
        
        // Define the error response
        let mut error_def = ErrorApiResponse {
            status_code: 500,
            error: "UnknownError".to_string(),
            messages: vec![],
            raw_response: None,
            body: None,
            url: Some(url.clone()),
            method: Some("GET".to_string()),
        };


        if let Err(e) = response {
            error_def.messages.push(e.to_string());
            return Err(AppError::new_api(
                "PriceScraper",
                error_def,
                eyre!(format!("There was an error sending the request: {}", e)),
                LogLevel::Critical,
            ));
        }

        // Get the response data from the response
        let response_data = response.unwrap();
        error_def.status_code = response_data.status().as_u16() as i64;
        let headers = response_data.headers().clone();
        let content = response_data.text().await.unwrap_or_default();
        error_def.raw_response = Some(content.clone());

        if error_def.status_code != 200{
            return Ok(ApiResult::Error(error_def,headers));
        }

        // Convert the response to a Value object
        let response: Value = serde_json::from_str(content.as_str()).map_err(|e| {
        error_def.messages.push(e.to_string());
        error_def.error = "ParseError".to_string();
        AppError::new_api(
            "PriceScraper",
            error_def.clone(),
            eyre!(""),
            LogLevel::Critical,
        )})?;
        return Ok(ApiResult::Success(response,headers));
    }
    /// Returns true if the given vector of item data is valid for price scraping, false otherwise.
    /// A valid item data vector must have at least one element, and the first element must have either 3 or 6 keys.
    /// The first element must also have a "mod_rank" key.
    fn is_valid_price_data(&self, _name: &str, item_datas: &Vec<Value>) -> bool {
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
        &self,
    ) -> Result<(HashMap<String, String>, HashMap<String, String>), AppError> {
        let wfm = self.wfm.lock()?.clone();

        let items = wfm.items().get_all_items().await?;

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
    pub async fn generate(&self, days: i64) -> Result<i64, AppError> {
        let auth = self.auth.lock().unwrap().clone();
        // Should only get 7 days of data
        let valid_days = 7;
        let csv_path: &Path = Path::new(self.csv_path.as_str());
        let csv_backop_path = Path::new(self.csv_backop_path.as_str());
        if csv_path.exists() {
            logger::debug_con(
                "PriceScraper",
                format!("Backuping csv file: {}", self.csv_path).as_str(),
            );
            fs::copy(csv_path, csv_backop_path)
                .map_err(|e| AppError::new("PriceScraper", eyre!(e.to_string())))?;
        }
        let last_days = helper::last_x_days(days).clone();
        let mut dataframes: Vec<DataFrame> = Vec::new();
        let (url_map, id_map) = self.get_items_map_url_map().await?;

        let mut found_data = 0;

        for day in last_days.clone() {
            if found_data >= valid_days {
                continue;
            }

            // Get the price data for the day for all items
            match self.get_price_by_day(auth.platform.as_str(), &day).await {
                Ok(ApiResult::Success(items, _headers)) => {
                    found_data += 1;
                    logger::info_con(
                        "PriceScraper",
                        format!("Getting data for day: {}", day).as_str(),
                    );
                    helper::send_message_to_window(
                        "PriceScraper:OnChange",
                        Some(json!({"max": valid_days, "min": 0, "current": found_data})),
                    );
                    if let Value::Object(map) = &items {
                        for (item_name, item_data_list) in map {
                            if let Value::Array(array) = item_data_list {
                                if !self.is_valid_price_data(&item_name, array) {
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
                                    .map(|item_data| {
                                        item_data.get("mod_rank").and_then(|v| v.as_f64())
                                    })
                                    .collect();

                                let median_vec: Vec<Option<f64>> = array
                                    .iter()
                                    .map(|item_data| {
                                        item_data.get("median").and_then(|v| v.as_f64())
                                    })
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
                                    .with_column(
                                        (col("max_price") - col("min_price")).alias("range"),
                                    )
                                    .collect()
                                    .map_err(|e| {
                                        AppError::new("PriceScraper", eyre!(e.to_string()))
                                    })?;

                                // Filter out items that are mod_rank 0.
                                let df = df
                                    .clone()
                                    .lazy()
                                    .filter(col("mod_rank").neq(0).or(col("mod_rank").is_null()))
                                    .collect()
                                    .map_err(|e| {
                                        AppError::new("PriceScraper", eyre!(e.to_string()))
                                    })?;
                                // dump_dataframe(&mut df, format!("{} {}.csv", day, item_name).as_str())?;
                                dataframes.push(df);
                            }
                        }
                    }
                },
                Ok(ApiResult::Error(e, _headers)) => {
                    if e.status_code == 404{
                        logger::info_con("PriceScraper", format!("No data for day: {}", day).as_str());
                    } else {
                        logger::error_file(
                            "PriceScraper",
                            format!("Error getting data for day: {}", day).as_str(),
                            Some("price_scraper.log"),
                        );
                    }
                }
                Err(e) => return Err(e),
            }
        }
        logger::info_con(
            "PriceScraper",
            format!(
                "Finished getting price data for all days. Merging dataframes... {:?}",
                dataframes.len()
            )
            .as_str(),
        );
        let full_df = helper::merge_dataframes(dataframes)?;
        helper::send_message_to_window("PriceScraper:Complete", Some(json!({ "max": valid_days })));

        // Group by name and get the average price
        let group_by_name = full_df
            .clone()
            .lazy()
            .groupby(&["name"])
            .agg(&[
                // List the other columns you want to average
                col("name").count().alias("name_count"),
            ])
            .collect()
            .map_err(|e| AppError::new("PriceScraper", eyre!(e.to_string())))?;

        // Get the names of the items that are popular

        let popular_items = group_by_name
            .clone()
            .lazy()
            .filter(col("name_count").gt_eq(21))
            .collect()
            .map_err(|e| AppError::new("PriceScraper", eyre!(e.to_string())))?;

        // Filter out items that are not popular and sort by name
        let popular_items_s = popular_items
            .column("name")
            .map_err(|e| AppError::new("PriceScraper", eyre!(e.to_string())))?;
        let mask = full_df
            .column("name")
            .map_err(|e| AppError::new("PriceScraper", eyre!(e.to_string())))?
            .is_in(&popular_items_s)
            .map_err(|e| AppError::new("PriceScraper", eyre!(e.to_string())))?;
        let filtered_df = full_df
            .filter(&mask)
            .map_err(|e| AppError::new("PriceScraper", eyre!(e.to_string())))?;
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
            .collect()
            .map_err(|e| AppError::new("PriceScraper", eyre!(e.to_string())))?;

        // Cerate a csv file with the sorted DataFrame of price data
        let output_file: File = File::create(csv_path)
            .map_err(|e| AppError::new("PriceScraper", eyre!(e.to_string())))?;
        let writer = BufWriter::new(output_file);
        // Write the DataFrame to a CSV file
        CsvWriter::new(writer)
            .finish(&mut filtered_df)
            .map_err(|e| AppError::new("PriceScraper", eyre!(e.to_string())))?;

        // Delete the backup file if it exists
        if csv_backop_path.exists() {
            fs::remove_file(csv_backop_path)
                .map_err(|e| AppError::new("PriceScraper", eyre!(e.to_string())))?;
        }
        Ok(full_df.height() as i64)
    }
}
