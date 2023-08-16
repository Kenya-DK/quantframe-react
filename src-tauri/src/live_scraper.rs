use crate::database;
use crate::price_scraper;
use crate::structs::GlobleError;
use crate::structs::OrderItem;
use crate::structs::Settings;
use crate::wfm_client;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use polars::prelude::*;
use std::error::Error;
use std::fs::File;
extern crate csv;
use serde::de::DeserializeOwned;
use serde_json::from_str;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;
use tauri::Window;

// Structs for the Warframe Market API

#[derive(Clone)]
pub struct LiveScraper {
    is_running: Arc<AtomicBool>,
    token: String,
    in_game_name: String,
    settings: Option<Settings>,
}

impl LiveScraper {
    pub fn new(token: String, in_game_name: String) -> Self {
        LiveScraper {
            is_running: Arc::new(AtomicBool::new(false)),
            token,
            in_game_name,
            settings: None,
        }
    }

    pub fn start_loop(&mut self, token: String, settings: Settings) -> Result<(), GlobleError> {
        println!("Start loop live scraper");
        self.token = token;
        self.settings = Some(settings);
        self.in_game_name = self.settings.as_ref().unwrap().in_game_name.clone();

        self.is_running.store(true, Ordering::SeqCst);
        let is_running = Arc::clone(&self.is_running);

        let scraper = self.clone();
        tauri::async_runtime::spawn(async move {
            // A loop that takes output from the async process and sends it
            // to the webview via a Tauri Event
            while is_running.load(Ordering::SeqCst) {
                println!("Loop live scraper is running...");
                match scraper.run().await {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
        Ok(())
    }

    pub fn dump_dataframe(&self, df: &DataFrame, name: &str) -> Result<(), GlobleError> {
        let mut log_path = PathBuf::from("logs");
        // Create the directory if it does not exist
        if !log_path.exists() {
            fs::create_dir_all(&log_path)?;
        }
        log_path.push(name);
        let mut output_file: File = File::create(log_path).unwrap();

        // Write the DataFrame to a CSV file
        CsvWriter::new(&mut output_file)
            .finish(&mut df.clone())
            .unwrap();
        Ok(())
    }

    fn get_sell_map_by_ordre_type(
        &self,
        df_filtered: &DataFrame,
        unique_names: &[&str],
        order_type: &str,
        col_name: &str,
    ) -> Result<Vec<f64>, GlobleError> {
        let sell_df = df_filtered
            .clone()
            .lazy()
            .filter(col("order_type").eq(lit(order_type)))
            .collect()?;

        // Create a HashMap to map names to min_sell values
        let mut min_sell_map = HashMap::new();
        for name in unique_names.iter() {
            let filtered_sell_df = sell_df
                .clone()
                .lazy()
                .filter(col("name").eq(lit(*name)))
                .select(&[col(col_name)])
                .collect()?;

            if let Ok(min_price_series) = filtered_sell_df.column(col_name) {
                if let Ok(min_price_array) = min_price_series.f64() {
                    if let Some(min_price) = min_price_array.get(0) {
                        min_sell_map.insert(*name, min_price);
                    }
                }
            }
        }
        let min_sell_values = unique_names
            .iter()
            .map(|name| *min_sell_map.get(name).unwrap_or(&f64::NAN))
            .collect::<Vec<_>>();
        Ok(min_sell_values)
    }

    fn get_week_increase(&self, df: &DataFrame, row_name: &str) -> Result<f64, GlobleError> {
        // Filter the pre-filtered DataFrame based on the "name"
        let week_df = df
            .clone()
            .lazy()
            .filter(col("name").eq(lit(row_name)))
            .select(&[col("avg_price")])
            .collect()?;

        // Assuming the filtered DataFrame has at least 7 rows
        if week_df.height() >= 7 {
            let first_avg_price_series = week_df.column("avg_price")?;
            let first_avg_price_array = first_avg_price_series.f64()?;
            let first_avg_price = first_avg_price_array.get(0).unwrap(); // Now a f64

            let seventh_avg_price_series = week_df.column("avg_price")?;
            let seventh_avg_price_array = seventh_avg_price_series.f64()?;
            let seventh_avg_price = seventh_avg_price_array.get(6).unwrap(); // Now a f64

            let change = first_avg_price - seventh_avg_price;
            Ok(change)
        } else {
            Ok(0.0)
        }
    }

    fn combine_column(
        &self,
        df_filter: &DataFrame,
        names: &[String],
        col_name: &str,
    ) -> Result<Vec<Option<f64>>, GlobleError> {
        // Convert the DataFrame to a Lazy DataFrame
        let lazy_df_filter = df_filter.clone().lazy();

        // Create a mutable HashMap to store the volume values
        let mut closed_vol_map = HashMap::new();

        // Iterate through the names and fetch the volume for each
        for name in names {
            let name_str = name.to_string();
            // Clone the Lazy DataFrame for each iteration
            let lazy_df_clone = lazy_df_filter.clone();
            let filtered_df = lazy_df_clone
                .filter(col("name").eq(lit(name_str.clone())))
                .collect()?;
            let volume_series = filtered_df.column(col_name)?;
            let volume_array = volume_series.f64()?;
            let volume_value = volume_array.get(0);
            closed_vol_map.insert(name_str, volume_value);
        }

        // Create a Vec from the HashMap, preserving the order of the original `names`
        let closed_vol_values: Vec<Option<f64>> = names
            .iter()
            .map(|name| *closed_vol_map.get(name).unwrap_or(&None))
            .collect();

        Ok(closed_vol_values)
    }

    pub fn get_itemid_byurl(&self, url_name: &str) -> Result<String, GlobleError> {
        let df = price_scraper::get_price_historys()?;
        let item_id_column = df
            .clone()
            .lazy()
            .filter(col("name").eq(lit(url_name)))
            .select(&[col("item_id")])
            .collect()?;

        let volume_series = item_id_column.column("item_id")?;
        let volume_array = volume_series.utf8()?;
        let volume_value = volume_array.get(0).unwrap_or("");
        Ok(volume_value.to_string())
    }
    pub fn get_item_avg_price(
        &self,
        buy_sell_overlap: &DataFrame,
        url_name: &str,
    ) -> Result<f64, GlobleError> {
        let item_id_column = buy_sell_overlap
            .clone()
            .lazy()
            .filter(col("name").eq(lit(url_name)))
            .select(&[col("closedAvg")])
            .collect()?;

        let volume_series = item_id_column.column("closedAvg")?;
        let volume_array = volume_series.f64()?;
        let volume_value = volume_array.get(0).unwrap_or(0.0);
        Ok(volume_value)
    }

    pub fn get_item_rank(
        &self,
        buy_sell_overlap: &DataFrame,
        url_name: &str,
    ) -> Result<Option<f64>, GlobleError> {
        let item_id_column = buy_sell_overlap
            .clone()
            .lazy()
            .filter(col("name").eq(lit(url_name)))
            .select(&[col("mod_rank")])
            .collect()?;

        let volume_series = item_id_column.column("mod_rank")?;
        let volume_array = volume_series.f64()?;
        let volume_value = volume_array.get(0);
        Ok(volume_value)
    }

    pub fn stop_loop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        // Return the current value of is_running
        self.is_running.load(Ordering::SeqCst)
    }

    pub fn get_buy_sell_overlap(&self) -> Result<DataFrame, GlobleError> {
        let df = price_scraper::get_price_historys()?;
        let volume_threshold = self.settings.as_ref().unwrap().volume_threshold; // Change according to your config
        let range_threshold = self.settings.as_ref().unwrap().range_threshold; // Change according to your config
        let avg_price_cap = self.settings.as_ref().unwrap().avg_price_cap; // assuming config contains the Rust value for avgPriceCap
        let price_shift_threshold = self.settings.as_ref().unwrap().price_shift_threshold; // assuming config contains the Rust value for priceShiftThreshold

        // Drop the "datetime" and "item_id" columns
        let averaged_df = df.drop("datetime")?.drop("item_id")?;

        // Create a lazy DataFrame
        let lazy_df = averaged_df.lazy();

        // Group by the "name" and "order_type" columns, and compute the mean of the other columns
        let averaged_df = lazy_df
            .groupby(&["name", "order_type"])
            .agg(&[
                // List the other columns you want to average
                col("volume").mean().alias("volume"),
                col("min_price").mean().alias("min_price"),
                col("max_price").mean().alias("max_price"),
                col("range").mean().alias("range"),
                col("median").mean().alias("median"),
                col("avg_price").mean().alias("avg_price"),
                col("mod_rank").mean().alias("mod_rank"),
            ])
            .collect()?;
        // Call the database to get the inventory names
        let inventory_names = match database::get_inventory_names() {
            Ok(names) => names,
            Err(e) => {
                println!("Error: {}", e);
                return Ok(averaged_df);
            }
        };
        let name_expr = inventory_names
            .clone()
            .into_iter()
            .map(|name| col("name").eq(lit(name)))
            .fold(lit(false), |acc, x| acc.or(x));

        let mask = (col("volume")
            .gt(lit(volume_threshold))
            .and(col("range").gt(lit(range_threshold))))
        .or(name_expr)
        .and(col("order_type").eq(lit("closed")));

        let filtered_df = averaged_df.clone().lazy().filter(mask).collect().unwrap();

        // Sort by "range" in descending order
        let mut sorted_df = filtered_df
            .lazy()
            .sort(
                "range",
                SortOptions {
                    descending: true,
                    nulls_last: false,
                    multithreaded: false,
                },
            )
            .collect()?;

        if sorted_df.height() == 0 {
            return Ok(DataFrame::new(vec![
                Series::new("name", &[] as &[&str]),
                Series::new("minSell", &[] as &[f64]),
                Series::new("maxBuy", &[] as &[f64]),
                Series::new("overlap", &[] as &[f64]),
                Series::new("closedVol", &[] as &[f64]),
                Series::new("closedMin", &[] as &[f64]),
                Series::new("closedMax", &[] as &[f64]),
                Series::new("closedAvg", &[] as &[f64]),
                Series::new("closedMedian", &[] as &[f64]),
                Series::new("priceShift", &[] as &[f64]),
                Series::new("mod_rank", &[] as &[i32]),
                Series::new("item_id", &[] as &[&str]),
            ])?);
        }

        // Pre-filter DataFrame based on "order_type" == "closed"
        let closed_df = price_scraper::get_price_historys()?
            .lazy()
            .filter(col("order_type").eq(lit("closed")))
            .collect()?;

        let name_column = sorted_df.column("name")?;
        let week_price_shifts: Vec<f64> = name_column
            .utf8()?
            .into_iter()
            .filter_map(|opt_name| {
                opt_name.map(|name| self.get_week_increase(&closed_df, name).unwrap_or(0.0))
            })
            .collect();

        // Create a new Series with the calculated week price shifts
        let week_price_shift_series = Series::new("weekPriceShift", week_price_shifts);
        let sorted_df = sorted_df.with_column(week_price_shift_series).cloned()?;

        let inventory_names_clone = inventory_names.clone(); // Clone the vector

        let inventory_names_expr = inventory_names_clone
            .iter()
            .map(|name| col("name").eq(lit(name.as_str())))
            .fold(lit(false), |acc, x| acc.or(x));

        let filter_condition = (col("avg_price")
            .lt(lit(avg_price_cap))
            .and(col("weekPriceShift").gt_eq(lit(price_shift_threshold))))
        .or(inventory_names_expr);
        let df_filter = sorted_df.lazy().filter(filter_condition).collect()?;

        let names = df_filter
            .column("name")?
            .utf8()?
            .into_iter()
            .filter_map(|opt_name| opt_name)
            .collect::<HashSet<_>>();

        let names_set: HashSet<String> = names.into_iter().map(|name| name.to_string()).collect();
        let name_column = averaged_df.column("name")?.utf8()?;
        let mask: BooleanChunked = name_column
            .into_iter()
            .map(|name| name.map(|n| names_set.contains(n)).unwrap_or(false))
            .collect();

        let df_filtered = averaged_df.filter(&mask)?;

        if df_filtered.column("name")?.unique()?.len() == 0 {
            return Ok(DataFrame::new(vec![
                Series::new("name", &[] as &[&str]),
                Series::new("minSell", &[] as &[f64]),
                Series::new("maxBuy", &[] as &[f64]),
                Series::new("overlap", &[] as &[f64]),
                Series::new("closedVol", &[] as &[f64]),
                Series::new("closedMin", &[] as &[f64]),
                Series::new("closedMax", &[] as &[f64]),
                Series::new("closedAvg", &[] as &[f64]),
                Series::new("closedMedian", &[] as &[f64]),
                Series::new("priceShift", &[] as &[f64]),
                Series::new("mod_rank", &[] as &[i32]),
                Series::new("item_id", &[] as &[&str]),
            ])?);
        }

        let unique_names = df_filter
            .column("name")?
            .utf8()?
            .into_iter()
            .filter_map(|opt_name| opt_name)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        let unique_names_series = Series::new("name", unique_names.clone());

        let mut buy_sell_overlap = DataFrame::new(vec![unique_names_series])?;

        let min_sell_values = self.get_sell_map_by_ordre_type(
            &df_filtered.clone(),
            &unique_names,
            "sell",
            "min_price",
        )?;

        buy_sell_overlap = buy_sell_overlap
            .with_column(Series::new("minSell", min_sell_values))?
            .clone();

        let max_sell_values = self.get_sell_map_by_ordre_type(
            &df_filtered.clone(),
            &unique_names,
            "buy",
            "max_price",
        )?;

        buy_sell_overlap = buy_sell_overlap
            .with_column(Series::new("maxBuy", max_sell_values))?
            .clone();

        let max_buy = buy_sell_overlap.column("maxBuy")?.f64()?;
        let min_sell = buy_sell_overlap.column("minSell")?.f64()?;

        // We need to make sure both ChunkedArrays are of the same type to perform arithmetic
        let overlap: ChunkedArray<Float64Type> = max_buy
            .into_iter()
            .zip(min_sell.into_iter())
            .map(|(max, min)| match (max, min) {
                (Some(max), Some(min)) => Some(max - min),
                _ => None, // Handle NaN or missing values as you see fit
            })
            .collect();

        let overlap_series = Series::new("overlap", overlap);

        // Add the new "overlap" column to the DataFrame
        let buy_sell_overlap = buy_sell_overlap.with_column(overlap_series)?;

        // ----------------------------------------------------------------
        let names = buy_sell_overlap
            .column("name")?
            .utf8()?
            .into_iter()
            .filter_map(|opt_name| opt_name.map(|name| name.to_string()))
            .collect::<Vec<_>>();

        // Add the new "closedVol" column to the DataFrame
        let buy_sell_overlap = buy_sell_overlap.with_column(Series::new(
            "closedVol",
            self.combine_column(&df_filter, &names, "volume")?,
        ))?;
        let buy_sell_overlap = buy_sell_overlap.with_column(Series::new(
            "closedMin",
            self.combine_column(&df_filter, &names, "min_price")?,
        ))?;
        let buy_sell_overlap = buy_sell_overlap.with_column(Series::new(
            "closedMax",
            self.combine_column(&df_filter, &names, "max_price")?,
        ))?;
        let buy_sell_overlap = buy_sell_overlap.with_column(Series::new(
            "closedAvg",
            self.combine_column(&df_filter, &names, "avg_price")?,
        ))?;
        let buy_sell_overlap = buy_sell_overlap.with_column(Series::new(
            "closedMedian",
            self.combine_column(&df_filter, &names, "median")?,
        ))?;
        let buy_sell_overlap = buy_sell_overlap.with_column(Series::new(
            "priceShift",
            self.combine_column(&df_filter, &names, "weekPriceShift")?,
        ))?;
        let buy_sell_overlap = buy_sell_overlap.with_column(Series::new(
            "mod_rank",
            self.combine_column(&df_filter, &names, "mod_rank")?,
        ))?;

        let buy_sell_overlap = buy_sell_overlap
            .with_column(self.add_item_ids(&df.clone(), &mut buy_sell_overlap.clone())?)?;
        Ok(buy_sell_overlap.clone())
    }
    fn add_item_ids(
        &self,
        df: &DataFrame,
        buy_sell_overlap: &DataFrame,
    ) -> Result<Series, GlobleError> {
        // Assuming "name" is of type String
        // Assuming "name" is of type String
        let names: Vec<String> = buy_sell_overlap
            .column("name")?
            .utf8()?
            .into_iter()
            .filter_map(|name| name.map(|n| n.to_string()))
            .collect();

        let mut item_id_values: Vec<Option<String>> = Vec::new();

        for name in &names {
            // Create the filter expression
            let filter_expr = col("name").eq(lit(name.to_owned()));

            // Apply the filter and collect the result
            let filtered_df = df
                .clone()
                .lazy()
                .filter(filter_expr)
                .select(&[col("item_id")])
                .collect()?;

            // Extract the value from the DataFrame
            let item_id_value = filtered_df
                .column("item_id")?
                .utf8()?
                .get(0)
                .map(|value| value.to_owned());

            // Append to the result vector
            item_id_values.push(item_id_value);
        }

        let item_id_series = Series::new("item_id", item_id_values);

        Ok(item_id_series)
    }

    // Http Request to Warframe Market API
    pub async fn run(&self) -> Result<(), GlobleError> {
        let buy_sell_overlap = self.get_buy_sell_overlap()?;
        self.dump_dataframe(&buy_sell_overlap, "buy_sell_overlap.csv")?;

        // let interesting_items = buy_sell_overlap
        //     .column("name")?
        //     .utf8()?
        //     .into_iter()
        //     .filter_map(|opt_name| opt_name)
        //     .collect::<HashSet<_>>();

        // TODO: Remove this
        //Create a new hash set with the items you want to track
        let mut interesting_items = HashSet::new();
        interesting_items.insert("fulmin_prime_set");

        let (mut my_buy_orders_df, my_sell_orders_df) =
            wfm_client::get_ordres_data_frames(&self.token, &self.in_game_name).await?;

        if my_buy_orders_df.height() != 0 {
            let closed_avg_series: Result<Vec<_>, polars::prelude::PolarsError> = my_buy_orders_df
                .column("url_name")?
                .utf8()?
                .into_iter()
                .map(|opt_name| {
                    opt_name.map_or(Ok(0.0), |name| {
                        // Clone the DataFrame before using it in the closure
                        let selected_df = buy_sell_overlap
                            .clone()
                            .lazy()
                            .filter(col("name").eq(lit(name)))
                            .select(&[col("closedAvg")])
                            .collect()?;
                        let buy_selected_df = my_buy_orders_df
                            .clone()
                            .lazy()
                            .filter(col("url_name").eq(lit(name)))
                            .select(&[col("platinum")])
                            .collect()?;
                        let maybe_first_row_df = selected_df.head(Some(1));
                        let buy_maybe_first_row_df = buy_selected_df.head(Some(1));
                        // Check if maybe_first_row_df is empty
                        let (rows, _) = maybe_first_row_df.shape();
                        if rows == 0 {
                            return Ok(0.0);
                        } else {
                            let closed_avg = maybe_first_row_df
                                .column("closedAvg")?
                                .f64()?
                                .get(0)
                                .unwrap_or(0.0);

                            let platinum = buy_maybe_first_row_df
                                .column("platinum")?
                                .i64()?
                                .get(0)
                                .unwrap_or(0);
                            return Ok(closed_avg - platinum as f64);
                        }
                    })
                })
                .collect();
            let closed_avg_series = closed_avg_series?;
            let potential_profit_series = Series::new("potential_profit", closed_avg_series);
            my_buy_orders_df = my_buy_orders_df
                .with_column(potential_profit_series)
                .cloned()?;
        }
        self.dump_dataframe(&my_buy_orders_df, "my_buy_orders_df.csv")?;
        self.dump_dataframe(&my_sell_orders_df, "my_sell_orders_df.csv")?;

        let inventorys =
            database::get_inventorys("SELECT * FROM Inventorys WHERE owned > 0").await?;
        // Loop through all to all interesting items
        for item in interesting_items {
            if self.is_running() == false {
                break;
            }
            let item_orders_df = wfm_client::get_ordres_by_item(&self.token, item).await?;
            // // Check if item_orders_df is empty and skip if it is
            if item_orders_df.height() == 0 {
                continue;
            }
            let item_id = self.get_itemid_byurl(item)?;
            let item_rank = self.get_item_rank(&buy_sell_overlap, item)?;

            let asd = self
                .compare_live_orders_when_buying(
                    &item,
                    &buy_sell_overlap,
                    &my_buy_orders_df,
                    &item_orders_df,
                    &item_id,
                    item_rank,
                )
                .await?;

            self.dump_dataframe(&item_orders_df, &format!("interesting_item_{}.csv", item))?;
        }
        Ok(())
    }

    pub async fn restructure_live_order_df(
        &self,
        live_item_orders: &DataFrame,
        in_game_name: &str,
    ) -> Result<(DataFrame, DataFrame, i64, i64, i64), GlobleError> {
        let buy_orders_df = live_item_orders
            .clone()
            .lazy()
            .filter(
                col("username")
                    .neq(lit(in_game_name))
                    .and(col("order_type").eq(lit("buy"))), // Add this line
            )
            .sort(
                "platinum",
                SortOptions {
                    descending: false,
                    nulls_last: false,
                    multithreaded: false,
                },
            )
            .collect()?;

        let sell_orders_df = live_item_orders
            .clone()
            .lazy()
            .filter(
                col("username")
                    .neq(lit(in_game_name))
                    .and(col("order_type").eq(lit("sell"))), // Add this line
            )
            .sort(
                "platinum",
                SortOptions {
                    descending: true,
                    nulls_last: false,
                    multithreaded: false,
                },
            )
            .collect()?;

        let mut lowest_price = 0;
        let mut highest_price = 0;

        let buyers = buy_orders_df.height() as i64;
        let sellers = sell_orders_df.height() as i64;

        if buyers > 0 {
            lowest_price = buy_orders_df.column("platinum")?.i64()?.get(0).unwrap_or(0);
        }

        if sellers > 0 {
            highest_price = sell_orders_df
                .column("platinum")?
                .i64()?
                .get(0)
                .unwrap_or(0);
        }

        let range = highest_price - lowest_price;
        // new DataFrame
        Ok((buy_orders_df, sell_orders_df, buyers, sellers, range))
    }
    pub async fn get_my_order_information(
        &self,
        item_name: &str,
        df: &DataFrame,
    ) -> Result<(Option<String>, bool, i64, bool), GlobleError> {
        let orders_by_item = df
            .clone()
            .lazy()
            .filter(col("url_name").eq(lit(item_name)))
            .collect()?;

        let id: Option<String> = None;
        let visibility = false;
        let price = 0;
        if orders_by_item.height() == 0 {
            return Ok((id, visibility, price, false));
        }

        let id = orders_by_item
            .column("id")?
            .utf8()?
            .get(0)
            .map(|x| x.to_owned());
        let visibility = orders_by_item
            .column("visible")?
            .bool()?
            .get(0)
            .unwrap_or(false);
        let price = orders_by_item
            .column("platinum")?
            .i64()?
            .get(0)
            .unwrap_or(0);
        // new DataFrame
        Ok((id.clone(), visibility, price, true))
    }

    pub async fn compare_live_orders_when_buying(
        &self,
        item_name: &str,
        buy_sell_overlap: &DataFrame,
        my_current_orders: &DataFrame,
        live_item_orders: &DataFrame,
        item_id: &str,
        item_rank: Option<f64>,
    ) -> Result<Option<DataFrame>, GlobleError> {
        let (order_id, visibility, price, active) = self
            .get_my_order_information(item_name, my_current_orders)
            .await?;

        // TODO: Delete this
        println!(
            "Id: {:?}, Visibility: {:?}, Price {:?}, Active: {:?}",
            order_id.unwrap_or("default".to_string()),
            visibility,
            price,
            active
        );

        let (mut live_buy_orders_df, live_sell_orders_df, buyers, sellers, price_range) = self
            .restructure_live_order_df(live_item_orders, &self.in_game_name)
            .await?;

        if sellers == 0 {
            return Ok(None);
        }
        let item_avg_price = self.get_item_avg_price(buy_sell_overlap, &item_name)?;
        // TODO: Delete this
        println!(
            "Buyers: {:?}, Sellers: {:?}, Price Range {:?}, Item avg price {:?}",
            buyers, sellers, price_range, item_avg_price
        );

        // if buyers == 0 && item_avg_price > 25.0 {
        //     let mut post_price = (price_range - 40).max((price_range / 3) - 1);
        //     if price_range > self.settings.as_ref().unwrap().avg_price_cap as i64 {
        //         println!("Price Range is to high");
        //         return Ok(None);
        //     }
        //     if post_price < 1 {
        //         post_price = 1;
        //     }
        //     if active {
        //         let updatede_order = wfm_client::update_order_listing(
        //             &self.token,
        //             order_id.clone().unwrap().as_str(),
        //             post_price.clone(),
        //             1,
        //             visibility,
        //         )
        //         .await?;
        //     } else {
        //         let postede_order = wfm_client::post_ordre(
        //             &self.token,
        //             item_id,
        //             "buy",
        //             post_price.clone(),
        //             1,
        //             true,
        //             item_rank,
        //         )
        //         .await?;
        //     }
        // } else if buyers == 0 {
        //     return Ok(None);
        // }

        // let post_price_column = live_buy_orders_df
        //     .clone()
        //     .lazy()
        //     .select(&[col("platinum")])
        //     .collect()?;
        // let post_price = post_price_column
        //     .column("platinum")?
        //     .i64()?
        //     .get(0)
        //     .unwrap_or(0);

        // let closed_avg_metric = item_avg_price as i64 - post_price;

        // let potential_profit = closed_avg_metric - 1;

        // if post_price as i32 > self.settings.as_ref().unwrap().avg_price_cap {
        //     println!("Price Range is to high");
        //     return Ok(None);
        // }
        // if (closed_avg_metric >= 30 && price_range >= 15) || price_range >= 21 {
        //     if active {
        //         if price != post_price as i64 {
        //             let updatede_order = wfm_client::update_order_listing(
        //                 &self.token,
        //                 order_id.clone().unwrap().as_str(),
        //                 post_price,
        //                 1,
        //                 visibility,
        //             )
        //             .await?;

        //         // live_buy_orders_df.clone().lazy().with_column(pl.when(pl.col('email')=='someGivenEmail').then(pl.lit("newValue")).otherwise(pl.col('col1')).alias('col1'))
        //         } else {
        //             println!("Price is the same");
        //         }
        //     } else {
        //     }
        // } else if active {
        //     let rep = wfm_client::delete_order(&self.token, order_id.unwrap().as_str()).await?;
        // }
        Ok(None)
    }
}
