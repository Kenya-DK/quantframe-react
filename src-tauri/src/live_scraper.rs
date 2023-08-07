use crate::structs::PriceHistoryDto;
use crate::structs::Response;
use crate::structs::Settings;

use polars::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::Cursor;
extern crate csv;
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
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
    window: Window,
    token: String,
    settings: Option<Settings>,
    csv_path: String,
    csv_backop_path: String,
    database_path: String,
}

impl LiveScraper {
    pub fn new(
        window: Window,
        token: String,
        csv_path: String,
        csv_backop_path: String,
        database_path: String,
    ) -> Self {
        LiveScraper {
            is_running: Arc::new(AtomicBool::new(false)),
            window,
            token,
            csv_path,
            csv_backop_path,
            database_path,
            settings: None,
        }
    }

    pub fn start_loop(&mut self, token: String, settings: Settings) {
        println!("Start loop live scraper");
        self.token = token;
        self.settings = Some(settings);

        self.is_running.store(true, Ordering::SeqCst);
        let is_running = Arc::clone(&self.is_running);

        let scraper = self.clone();
        tauri::async_runtime::spawn(async move {
            // A loop that takes output from the async process and sends it
            // to the webview via a Tauri Event
            while is_running.load(Ordering::SeqCst) {
                println!("Loop live scraper is running...");
                if let Err(e) = scraper.run().await {
                    println!("Error: {}", e);
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
    }

    pub fn get_price_historys(&self) -> Result<DataFrame, PolarsError> {
        // Try to read from "allItemDataBackup.csv", and if it fails, read from "allItemData.csv".
        let file = File::open(&self.csv_backop_path).or_else(|_| File::open(&self.csv_path))?;

        // Parse the CSV file into a DataFrame
        CsvReader::new(file)
            .infer_schema(None)
            .has_header(true)
            .finish()
    }

    fn get_week_increase(&self, row_name: &str) -> Result<f64, PolarsError> {
        let df = self.get_price_historys()?;

        // Filter the DataFrame based on the "name" and "order_type" conditions
        let week_df = df
            .lazy()
            .filter(
                col("name")
                    .eq(lit(row_name))
                    .and(col("order_type").eq(lit("closed"))),
            )
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
    pub fn stop_loop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        // Return the current value of is_running
        self.is_running.load(Ordering::SeqCst)
    }

    pub fn get_buy_sell_overlap(&self) -> Result<DataFrame, PolarsError> {
        let df = self.get_price_historys()?;

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

        let inventory_names = vec!["zhuge_prime_barrel", "vulkar_wraith"]; // Add your inventory names here
        let volume_threshold = 1; // Change according to your config
        let range_threshold = 10; // Change according to your config

        // Apply filters based on volume, range, name, and order_type
        let name_expr = inventory_names
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
            .collect()
            .unwrap();

        let name_column = sorted_df.column("name")?;
        let week_price_shifts: Vec<f64> = name_column
            .utf8()?
            .into_iter()
            .filter_map(|opt_name| opt_name.map(|name| self.get_week_increase(name).unwrap_or(0.0)))
            .collect();

        // Create a new Series with the calculated week price shifts
        let week_price_shift_series = Series::new("weekPriceShift", week_price_shifts);
        let mut sorted_df = sorted_df.with_column(week_price_shift_series)?;

        Ok(sorted_df.clone())
        // if sorted_df.height() == 0 {
        //     Ok(DataFrame::new(vec![
        //         Series::new("name", &[] as &[&str]),
        //         Series::new("minSell", &[] as &[f64]),
        //         Series::new("maxBuy", &[] as &[f64]),
        //         Series::new("overlap", &[] as &[f64]),
        //         Series::new("closedVol", &[] as &[f64]),
        //         Series::new("closedMin", &[] as &[f64]),
        //         Series::new("closedMax", &[] as &[f64]),
        //         Series::new("closedAvg", &[] as &[f64]),
        //         Series::new("closedMedian", &[] as &[f64]),
        //         Series::new("priceShift", &[] as &[f64]),
        //         Series::new("mod_rank", &[] as &[i32]),
        //         Series::new("item_id", &[] as &[&str]),
        //     ]));
        // }
        // sorted_df.ad
    }

    // Http Request to Warframe Market API
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let records = self.get_buy_sell_overlap()?;
        println!("{:?}", records);
        // let records = self.get_csv()?;

        // for record in &records {
        //     println!("{:?}", record);
        // }

        // let url = "https://api.warframe.market/v1/items";
        // let response: Result<Response<ResponseWFMPayload>, String> =
        //     self.perform_request(url).await;
        // match response {
        //     Ok(response) => {
        //         let items = response.data.payload.items;
        //         let mut items_name: Vec<String> = Vec::new();
        //         for item in items {
        //             items_name.push(item.item_name);
        //         }
        //         let items_name = items_name.join(", ");
        //         let settings = Settings {
        //             data: items_name.to_string(),
        //         };
        //     }
        //     Err(e) => {
        //         println!("Error: {}", e);
        //     }
        // }
        Ok(())
    }
    pub async fn perform_request<T: DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<Response<T>, String> {
        // Create an HTTPS connector and client
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        // Make the GET request
        let uri: Uri = url.parse::<Uri>().map_err(|e| e.to_string())?;
        let res = client.get(uri).await.map_err(|e| e.to_string())?;
        let body = hyper::body::to_bytes(res.into_body())
            .await
            .map_err::<String, _>(|e| e.to_string())?;
        let json_str = String::from_utf8_lossy(&body).to_string();

        // Deserialize the JSON response into the generic type T
        let data: T = from_str(&json_str).map_err(|e| e.to_string())?;

        Ok(Response { data })
    }
}
