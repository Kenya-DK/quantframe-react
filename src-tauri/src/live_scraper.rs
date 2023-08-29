use crate::auth::AuthState;
use crate::price_scraper::PriceScraper;
use crate::structs::Order;
use crate::{
    database::DatabaseClient,
    helper::{self, ColumnType, ColumnValue, ColumnValues},
    logger,
    settings::SettingsState,
    structs::GlobleError,
    wfm_client::WFMClientState,
};
use polars::prelude::*;
use serde_json::json;
use std::time::Duration;
use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

// Structs for the Warframe Market API

#[derive(Clone)]
pub struct LiveScraper {
    log_file: String,
    is_running: Arc<AtomicBool>,
    settings: Arc<Mutex<SettingsState>>,
    price_scraper: Arc<Mutex<PriceScraper>>,
    wfm: Arc<Mutex<WFMClientState>>,
    auth: Arc<Mutex<AuthState>>,
    db: Arc<Mutex<DatabaseClient>>,
}

impl LiveScraper {
    pub fn new(
        settings: Arc<Mutex<SettingsState>>,
        price_scraper: Arc<Mutex<PriceScraper>>,
        wfm: Arc<Mutex<WFMClientState>>,
        auth: Arc<Mutex<AuthState>>,
        db: Arc<Mutex<DatabaseClient>>,
    ) -> Self {
        LiveScraper {
            log_file: "live_scraper.log".to_string(),
            price_scraper,
            settings,
            is_running: Arc::new(AtomicBool::new(false)),
            wfm,
            auth,
            db,
        }
    }

    pub fn start_loop(&mut self) -> Result<(), GlobleError> {
        self.is_running.store(true, Ordering::SeqCst);
        let is_running = Arc::clone(&self.is_running);
        let forced_stop = Arc::clone(&self.is_running);
        let scraper = self.clone();

        tauri::async_runtime::spawn(async move {
            // A loop that takes output from the async process and sends it
            // to the webview via a Tauri Event
            logger::info_con("LiveScraper", "Loop live scraper is started");
            match scraper.delete_all_orders().await {
                Ok(_) => {
                    logger::info_con("LiveScraper:DeleteAllOrders", "Delete all orders success");
                }
                Err(e) => {
                    logger::error_con("LiveScraper:DeleteAllOrders", format!("{:?}", e).as_str());
                    helper::send_message_to_window(
                        "live_scraper_error",
                        Some(json!({ "error": format!("{:?}", e) })),
                    );

                    forced_stop.store(false, Ordering::SeqCst);
                    eprint!("{:?}", e);
                }
            }

            while is_running.load(Ordering::SeqCst) && forced_stop.load(Ordering::SeqCst) {
                logger::info_con("LiveScraper", "Loop live scraper is running...");
                match scraper.run().await {
                    Ok(_) => {}
                    Err(e) => {
                        logger::error_con("LiveScraper", format!("{:?}", e).as_str());
                        helper::send_message_to_window(
                            "live_scraper_error",
                            Some(json!({ "error": format!("{:?}", e) })),
                        );

                        forced_stop.store(false, Ordering::SeqCst);
                        eprint!("{:?}", e);
                    }
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            logger::info_con("LiveScraper", "Loop live scraper is stopped");
        });
        Ok(())
    }

    pub fn stop_loop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        // Return the current value of is_running
        self.is_running.load(Ordering::SeqCst)
    }

    pub async fn run(&self) -> Result<(), GlobleError> {
        let buy_sell_overlap = self.get_buy_sell_overlap().await?;
        let settings = self.settings.lock()?.clone();
        let db = self.db.lock()?.clone();
        let wfm: WFMClientState = self.wfm.lock()?.clone();

        let inventory_df = db.get_inventorys_df().await?;
        let whitelist = settings.whitelist.clone();
        // Call the database to get the inventory names
        let inventory_names = db.get_inventory_names().await?;

        // Get interesting items from buy_sell_overlap
        let interesting_items: Vec<String> = match helper::get_column_values(
            buy_sell_overlap.clone(),
            None,
            "name",
            ColumnType::String,
        )? {
            ColumnValues::String(values) => values,
            _ => return Err(GlobleError::OtherError("Expected f64 values".to_string())),
        };

        // Get current orders from Warframe Market Sell and Buy orders
        let (mut current_buy_orders_df, current_sell_orders_df) =
            wfm.get_ordres_data_frames().await?;

        if current_buy_orders_df.shape().0 != 0 {
            current_buy_orders_df = current_buy_orders_df
                .lazy()
                .filter(col("url_name").is_in(lit(Series::new(
                    "interesting_items",
                    interesting_items.clone(),
                ))))
                .collect()?;

            let order_buy_df = helper::filter_and_extract(
                buy_sell_overlap.clone(),
                None,
                vec!["name", "closedAvg"],
            )?;

            current_buy_orders_df =
                current_buy_orders_df.inner_join(&order_buy_df, ["url_name"], ["name"])?;

            current_buy_orders_df = current_buy_orders_df
                .clone()
                .lazy()
                .fill_nan(lit(0.0).alias("closedAvg"))
                .fill_nan(lit(0.0).alias("platinum"))
                .with_column((col("closedAvg") - col("platinum")).alias("potential_profit"))
                .collect()?;
        }

        // Combine inventory_names and interesting_items and whitelist
        let all_interesting_items = inventory_names
            .clone()
            .into_iter()
            .chain(interesting_items.clone().into_iter())
            .chain(whitelist.into_iter())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        // Remove duplicates
        let all_interesting_items: HashSet<String> = HashSet::from_iter(all_interesting_items);

        logger::info_file(
            "LiveScraper",
            format!("Interesting items: {:?}", all_interesting_items).as_str(),
            Some(self.log_file.as_str()),
        );
        // Loop through all interesting items
        for item in all_interesting_items.clone() {
            if self.is_running() == false || item == "" {
                break;
            }

            logger::info_con("LiveScraper", format!("Checking item: {item}").as_str());

            let mut item_live_orders_df = wfm.get_ordres_by_item(&item).await?;
            logger::log_dataframe(
                &mut item_live_orders_df,
                "live_scraper_item_live_orders_df.csv",
            );
            // Check if item_orders_df is empty and skip if it is
            if item_live_orders_df.height() == 0 {
                continue;
            }
            let item_stats = buy_sell_overlap
                .clone()
                .lazy()
                .filter(col("name").eq(lit(item.clone())))
                .collect()?;
            // Check if item is in all_interesting_items
            if !interesting_items.contains(&item) {
                logger::info_file(
                    "LiveScraper",
                    format!("Item: {item} is not in all_interesting_items").as_str(),
                    Some(self.log_file.as_str()),
                );
                let item_info = wfm.get_item(item.to_string()).await?;

                let item_id = item_info.id;
                let item_rank = item_info.items_in_set.get(0).unwrap().mod_max_rank;
                self.compare_live_orders_when_selling(
                    &item,
                    &item_id,
                    item_rank,
                    current_sell_orders_df.clone(),
                    &item_live_orders_df,
                    &item_stats,
                    &inventory_df,
                )
                .await?;
                continue;
            }

            // Get the item_id and item_rank
            let item_id: String = match helper::get_column_value(
                buy_sell_overlap.clone(),
                Some(col("name").eq(lit(item.clone()))),
                "item_id",
                ColumnType::String,
            )? {
                ColumnValue::String(values) => values.unwrap_or("".to_string()),
                _ => return Err(GlobleError::OtherError("Expected f64 values".to_string())),
            };

            let item_rank: Option<f64> = match helper::get_column_value(
                buy_sell_overlap.clone(),
                Some(col("name").eq(lit(item.clone()))),
                "mod_rank",
                ColumnType::F64,
            )? {
                ColumnValue::F64(values) => values,
                _ => return Err(GlobleError::OtherError("Expected f64 values".to_string())),
            };

            let item_stats = buy_sell_overlap
                .clone()
                .lazy()
                .filter(col("name").eq(lit(item.clone())))
                .collect()?;

            self.compare_live_orders_when_buying(
                &item,
                &item_id,
                item_rank,
                current_buy_orders_df.clone(),
                &item_live_orders_df,
                &item_stats,
                &inventory_df,
            )
            .await?;

            self.compare_live_orders_when_selling(
                &item,
                &item_id,
                item_rank,
                current_sell_orders_df.clone(),
                &item_live_orders_df,
                &item_stats,
                &inventory_df,
            )
            .await?;
        }

        // if current_sell_orders_df.height() != 0 {}
        logger::log_dataframe(
            &mut current_buy_orders_df.clone(),
            "live_scraper_current_buy_orders_df.csv",
        );
        logger::log_dataframe(
            &mut current_sell_orders_df.clone(),
            "live_scraper_current_sell_orders_df.csv",
        );
        Ok(())
    }

    fn get_week_increase(&self, df: &DataFrame, row_name: &str) -> Result<f64, GlobleError> {
        // Pre-filter DataFrame based on "order_type" == "closed"
        let week_df = df
            .clone()
            .lazy()
            .filter(
                col("order_type")
                    .eq(lit("closed"))
                    .and(col("name").eq(lit(row_name))),
            )
            .collect()?;

        // Sort the DataFrame by "datetime" column
        let week_df = helper::sort_dataframe(week_df, "datetime", true)?;

        // Assuming the filtered DataFrame has at least 7 rows
        if week_df.height() >= 7 {
            let avg_price_series = week_df.column("median")?;
            let avg_price_array = avg_price_series.f64()?;
            let first_avg_price = avg_price_array.get(0).unwrap(); // Now a f64
            let last_avg_price = avg_price_array.get(6).unwrap(); // Now a f64

            let change = first_avg_price - last_avg_price;
            Ok(change)
        } else {
            Ok(0.0)
        }
    }
    pub async fn delete_all_orders(&self) -> Result<(), GlobleError> {
        let wfm = self.wfm.lock()?.clone();
        let db = self.db.lock()?.clone();
        let settings = self.settings.lock()?.clone();
        let blacklist = settings.blacklist.clone();

        let current_orders = wfm.get_user_ordres().await?;

        for order in current_orders.sell_orders {
            // Check if item is in blacklist
            if blacklist.contains(&order.item.url_name) {
                continue;
            }
            db.update_inventory_by_url(order.item.url_name, None)
                .await?;
            wfm.delete_order(&order.id, "None", "None", "Any").await?;
        }
        for order in current_orders.buy_orders {
            // Check if item is in blacklist
            if blacklist.contains(&order.item.url_name) {
                continue;
            }
            wfm.delete_order(&order.id, "None", "None", "Any").await?;
        }
        Ok(())
    }
    pub async fn get_buy_sell_overlap(&self) -> Result<DataFrame, GlobleError> {
        let settings = self.settings.lock()?.clone();
        let db = self.db.lock()?.clone();
        println!("{:?}", settings);
        let df = self.price_scraper.lock()?.get_price_historys()?;
        let volume_threshold = settings.volume_threshold;
        let range_threshold = settings.range_threshold;
        let avg_price_cap = settings.avg_price_cap;
        let price_shift_threshold = settings.price_shift_threshold;
        let strict_whitelist = settings.strict_whitelist;
        let whitelist = settings.whitelist.clone();

        // Group by the "name" and "order_type" columns, and compute the mean of the other columns
        let mut averaged_df = df
            .clone()
            .lazy()
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
                col("item_id").first().alias("item_id"),
            ])
            .collect()?;

        logger::log_dataframe(&mut averaged_df, "live_scraper_averaged_df.csv");

        // Call the database to get the inventory names and DataFrame
        let inventory_names = db.get_inventory_names().await?;
        let inventory_names_s = Series::new("desired_column_name", inventory_names);

        // Filters the DataFrame based on the given predicates and returns a new DataFrame.
        // The `volume_threshold` and `range_threshold` arguments are used to filter by volume and range.
        // The `inventory_names_s` argument is used to filter by name.
        // The `closed` order type is used to filter by order type.
        let filtered_df = averaged_df
            .clone()
            .lazy()
            .filter(
                col("order_type").eq(lit("closed")).and(
                    col("volume")
                        .gt(lit(volume_threshold))
                        .and(col("range").gt(lit(range_threshold)))
                        .or(col("name").is_in(lit(inventory_names_s.clone()))),
                ),
            )
            .collect()?;

        // Sort by "range" in descending order
        let mut filtered_df = helper::sort_dataframe(filtered_df, "range", true)?;
        logger::log_dataframe(&mut filtered_df, "live_scraper_filtered_df.csv");

        // If the DataFrame is empty, return an empty DataFrame
        if filtered_df.height() == 0 {
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

        // Get the "name" column from the DataFrame
        let name_column = filtered_df.column("name")?;

        // Create a new Series with the calculated week price shifts
        let week_price_shifts: Vec<f64> = name_column
            .utf8()?
            .into_iter()
            .filter_map(|opt_name| {
                opt_name.map(|name| self.get_week_increase(&df, name).unwrap_or(0.0))
            })
            .collect();

        let mut filtered_df = filtered_df
            .with_column(Series::new("weekPriceShift", week_price_shifts))
            .cloned()?;
        logger::log_dataframe(&mut filtered_df, "live_scraper_weekPriceShift.csv");

        // Handle the whitelist if it is strict or not
        let whitelist_s = Series::new("whitelist", whitelist);
        if strict_whitelist {
            filtered_df = filtered_df
                .lazy()
                .filter(col("name").is_in(lit(whitelist_s)))
                .collect()?;
        } else {
            filtered_df = filtered_df
                .lazy()
                .filter(
                    col("avg_price")
                        .lt(lit(avg_price_cap))
                        .and(col("weekPriceShift").gt_eq(lit(price_shift_threshold)))
                        .or(col("name").is_in(lit(inventory_names_s)))
                        .or(col("name").is_in(lit(whitelist_s))),
                )
                .collect()?;
        }

        // Extract unique names from filtered_df into a HashSet
        let name_set: HashSet<String> = HashSet::from_iter(
            match helper::get_column_values(filtered_df.clone(), None, "name", ColumnType::String)?
            {
                ColumnValues::String(values) => values,
                _ => return Err(GlobleError::OtherError("Expected f64 values".to_string())),
            },
        );
        let unique_names = name_set.into_iter().collect::<Vec<_>>();

        let unique_names_series = Series::new("name", unique_names.clone());
        let mut df_filtered = averaged_df
            .clone()
            .lazy()
            .filter(col("name").is_in(lit(unique_names_series.clone())))
            .collect()?;
        logger::log_dataframe(&mut df_filtered, "live_scraper_dffiltered.csv");

        // Start the creation of the buy_sell_overlap DataFrame
        let buy_sell_overlap = DataFrame::new(vec![unique_names_series])?;

        // Get Order type "sell" and "buy" into separate DataFrames
        let mut order_sell_df = helper::filter_and_extract(
            df_filtered.clone(),
            Some(col("order_type").eq(lit("sell"))),
            vec!["name", "min_price"],
        )?;
        let order_sell_df = order_sell_df.rename("min_price", "minSell")?;

        let mut order_buy_df = helper::filter_and_extract(
            df_filtered.clone(),
            Some(col("order_type").eq(lit("buy"))),
            vec!["name", "max_price"],
        )?;
        let order_buy_df = order_buy_df.rename("max_price", "maxBuy")?;

        // Remove unnecessary columns
        let filtered_df = filtered_df.drop_many(&["range", "order_type"]);

        // Join the DataFrames together
        let buy_sell_overlap = buy_sell_overlap
            .inner_join(&order_sell_df, ["name"], ["name"])?
            .inner_join(&order_buy_df, ["name"], ["name"])?
            .inner_join(&filtered_df, ["name"], ["name"])?;

        // Calculate the overlap
        let mut buy_sell_overlap: DataFrame = buy_sell_overlap
            .clone()
            .lazy()
            .fill_nan(lit(0.0).alias("maxBuy"))
            .fill_nan(lit(0.0).alias("minSell"))
            .with_column((col("maxBuy") - col("minSell")).alias("overlap"))
            .collect()?;

        // Rename the columns
        let buy_sell_overlap = buy_sell_overlap
            .rename("volume", "closedVol")?
            .rename("min_price", "closedMin")?
            .rename("max_price", "closedMax")?
            .rename("avg_price", "closedAvg")?
            .rename("median", "closedMedian")?
            .rename("weekPriceShift", "priceShift")?;

        logger::log_dataframe(buy_sell_overlap, "live_scraper_buy_sell_overlap.csv");
        return Ok(buy_sell_overlap.clone());
    }
    async fn get_my_order_information(
        &self,
        item_name: &str,
        df: &DataFrame,
    ) -> Result<(Option<String>, bool, i64, bool), GlobleError> {
        // To print the first few rows of df

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
        let id =
            match helper::get_column_value(orders_by_item.clone(), None, "id", ColumnType::String)?
            {
                ColumnValue::String(values) => values,
                _ => return Err(GlobleError::OtherError("Expected f64 values".to_string())),
            };

        let visibility = match helper::get_column_value(
            orders_by_item.clone(),
            None,
            "visible",
            ColumnType::Bool,
        )? {
            ColumnValue::Bool(values) => values.unwrap_or(false),
            _ => return Err(GlobleError::OtherError("Expected f64 values".to_string())),
        };

        let price: i64 = match helper::get_column_value(
            orders_by_item.clone(),
            None,
            "platinum",
            ColumnType::I64,
        )? {
            ColumnValue::I64(values) => values.unwrap_or(0),
            _ => return Err(GlobleError::OtherError("Expected f64 values".to_string())),
        };
        Ok((id.clone(), visibility, price, true))
    }
    async fn restructure_live_order_df(
        &self,
        item_live_orders_df: &DataFrame,
    ) -> Result<(DataFrame, DataFrame, i64, i64, i64), GlobleError> {
        let in_game_name = self.auth.lock()?.clone().ingame_name;
        let buy_orders_df = item_live_orders_df
            .clone()
            .lazy()
            .filter(
                col("username")
                    .neq(lit(in_game_name.clone()))
                    .and(col("order_type").eq(lit("buy"))), // Add this line
            )
            .collect()?;
        let buy_orders_df = helper::sort_dataframe(buy_orders_df, "platinum", true)?;

        let sell_orders_df = item_live_orders_df
            .clone()
            .lazy()
            .filter(
                col("username")
                    .neq(lit(in_game_name))
                    .and(col("order_type").eq(lit("sell"))), // Add this line
            )
            .collect()?;
        let sell_orders_df = helper::sort_dataframe(sell_orders_df, "platinum", false)?;

        let mut lowest_price = 0;
        let mut highest_price = 0;

        let buyers = buy_orders_df.height() as i64;
        let sellers = sell_orders_df.height() as i64;
        if buyers > 0 {
            lowest_price = match helper::get_column_value(
                buy_orders_df.clone(),
                None,
                "platinum",
                ColumnType::I64,
            )? {
                ColumnValue::I64(values) => values.unwrap_or(0),
                _ => return Err(GlobleError::OtherError("Expected f64 values".to_string())),
            };
        }

        if sellers > 0 {
            highest_price = match helper::get_column_value(
                sell_orders_df.clone(),
                None,
                "platinum",
                ColumnType::I64,
            )? {
                ColumnValue::I64(values) => values.unwrap_or(0),
                _ => return Err(GlobleError::OtherError("Expected f64 values".to_string())),
            };
        }
        let range = highest_price - lowest_price;
        Ok((buy_orders_df, sell_orders_df, buyers, sellers, range))
    }

    fn is_item_blacklisted(&self, item_name: &str) -> Result<bool, GlobleError> {
        let settings = self.settings.lock()?.clone();
        let blacklist = settings.blacklist.clone();
        let blacklist_s = Series::new("blacklist", blacklist);
        let blacklist_df = DataFrame::new(vec![blacklist_s]).unwrap();
        let blacklist_df = blacklist_df
            .lazy()
            .filter(col("blacklist").eq(lit(item_name)))
            .collect()
            .unwrap();
        if blacklist_df.height() == 0 {
            return Ok(false);
        }
        Ok(true)
    }

    fn knapsack(
        &self,
        items: Vec<(i64, f64, String, String)>,
        max_weight: i64,
    ) -> Result<
        (
            i64,
            Vec<(i64, f64, String, String)>,
            Vec<(i64, f64, String, String)>,
        ),
        GlobleError,
    > {
        let n = items.len();
        let mut dp = vec![vec![0; (max_weight + 1) as usize]; (n + 1) as usize];

        for i in 1..=n {
            for w in 1..=max_weight {
                let (weight, value, _, _) = items[i - 1];
                if weight <= w {
                    dp[i][w as usize] =
                        dp[i - 1][w as usize].max(dp[i - 1][(w - weight) as usize] + value as i64);
                } else {
                    dp[i][w as usize] = dp[i - 1][w as usize];
                }
            }
        }

        let mut selected_items = Vec::new();
        let mut unselected_items = Vec::new();
        let mut w = max_weight;
        for i in (0..n).rev() {
            if dp[i + 1][w as usize] != dp[i][w as usize] {
                selected_items.push(items[i].clone());
                w -= items[i].0;
            } else {
                unselected_items.push(items[i].clone());
            }
        }

        Ok((dp[n][max_weight as usize], selected_items, unselected_items))
    }
    async fn compare_live_orders_when_buying(
        &self,
        item_name: &str,
        item_id: &str,
        item_rank: Option<f64>,
        current_orders: DataFrame,
        item_live_orders_df: &DataFrame,
        item_stats: &DataFrame,
        inventory_df: &DataFrame,
    ) -> Result<Option<DataFrame>, GlobleError> {
        // Check if item is blacklisted
        if self.is_item_blacklisted(item_name)? {
            return Ok(None);
        }

        let settings = self.settings.lock()?.clone();
        let wfm = self.wfm.lock()?.clone();
        let mut current_orders = current_orders.clone();
        let avg_price_cap = settings.avg_price_cap;
        let max_total_price_cap = settings.max_total_price_cap;
        // Get the current orders for the item from the Warframe Market API
        let (order_id, visibility, price, active) = self
            .get_my_order_information(item_name, &current_orders)
            .await?;
        logger::debug_file(
            "LiveScraper",
            format!(
                "Name: {}, Order: {}, Visibility: {}, Price: {}, Active: {}",
                item_name,
                order_id.clone().unwrap_or("None".to_string()),
                visibility,
                price,
                active
            )
            .as_str(),
            Some("getMyOrderInformation.log"),
        );

        // Get all the live orders for the item from the Warframe Market API
        let (live_buy_orders_df, _live_sell_orders_df, buyers, sellers, price_range) =
            self.restructure_live_order_df(item_live_orders_df).await?;

        logger::debug_file("LiveScraper",format!("Name: {item_name}, Buyers: {buyers}, Sellers: {sellers}, Price Range: {price_range}").as_str(), Some("restructureLiveOrderDF.log"));

        // Probably don't want to be looking at this item right now if there's literally nobody interested in selling it.
        if sellers == 0 {
            return Ok(None);
        }

        // Get the average price of the item from the Warframe Market API
        let item_closed_avg: f64 =
            match helper::get_column_value(item_stats.clone(), None, "closedAvg", ColumnType::F64)?
            {
                ColumnValue::F64(values) => values.unwrap_or(0.0),
                _ => return Err(GlobleError::OtherError("Expected f64 values".to_string())),
            };

        logger::debug_file(
            "LiveScraper",
            format!(
                "Name: {item_name}, Buyers: {buyers}, ClosedAvg: {}",
                item_closed_avg
            )
            .as_str(),
            Some("if_there_are_no_buyers.log"),
        );
        // If there are no buyers, and the average price is greater than 25p, then we should probably update our listing.
        if buyers == 0 && item_closed_avg > 25.0 {
            // If the item is worth more than 40p, then we should probably update our listing.
            let mut post_price = (price_range - 40).max((price_range / 3) - 1);

            if post_price > avg_price_cap as i64 {
                logger::info_con("LiveScraper",format!("Item {item_name} is higher than the price cap you set. cap: {avg_price_cap}, post_price: {post_price}").as_str());
                return Ok(None);
            }
            // If the item is worth less than 1p, then we should probably update our listing.
            if post_price < 1 {
                post_price = 1;
            }
            // If the order is active, then we should update it else we should post a new order.
            if active {
                wfm.update_order_listing(
                    order_id.clone().unwrap().as_str(),
                    post_price,
                    1,
                    visibility,
                    item_name,
                    item_id,
                    "buy",
                )
                .await?;
                return Ok(None);
            } else {
                wfm.post_ordre(item_name, item_id, "buy", post_price, 1, true, item_rank)
                    .await?;
                logger::info_con("LiveScraper",format!("Automatically Posted Visible Buy Order Item: {item_name}, ItemId: {item_id}, Price: {post_price}").as_str());
                return Ok(None);
            }
        } else if buyers == 0 {
            return Ok(None);
        }

        // Get highest buy order price
        let post_price: i64 = match helper::get_column_value(
            live_buy_orders_df.clone(),
            None,
            "platinum",
            ColumnType::I64,
        )? {
            ColumnValue::I64(values) => values.unwrap_or(0),
            _ => return Err(GlobleError::OtherError("Expected i64 values".to_string())),
        };

        // Get the average price of the item from the Warframe Market API
        let closed_avg_metric: f64 =
            match helper::get_column_value(item_stats.clone(), None, "closedAvg", ColumnType::F64)?
            {
                ColumnValue::F64(values) => values.unwrap_or(0.0) - post_price as f64,
                _ => return Err(GlobleError::OtherError("Expected f64 values".to_string())),
            };
        let potential_profit = closed_avg_metric - 1.0;

        // Check if the post price is greater than the average price cap and return if it is
        if post_price > avg_price_cap as i64 {
            logger::info_con("LiveScraper",format!("Item {item_name} is higher than the price cap you set. cap: {avg_price_cap}, post_price: {post_price}").as_str());
            return Ok(None);
        }
        // Get the owned value from the database
        let owned: i64 = match helper::get_column_value(
            inventory_df.clone(),
            Some(col("item_url").eq(lit(item_name))),
            "owned",
            ColumnType::I64,
        )? {
            ColumnValue::I64(values) => values.unwrap_or(0),
            _ => return Err(GlobleError::OtherError("Expected f64 values".to_string())),
        };

        logger::debug_file(
            "LiveScraper",
            format!("Name: {item_name}, Owned: {owned}, ClosedAvgMetric: {closed_avg_metric}")
                .as_str(),
            Some("68.log"),
        );
        if owned > 1 && ((closed_avg_metric as i64) < (25 * owned)) {
            logger::info_con(
                "LiveScraper",
                format!("You're holding too many of this {item_name}! Not putting up a buy order.")
                    .as_str(),
            );
            if active {
                logger::info_con(
                    "LiveScraper",
                    format!("In fact you have a buy order up for this {item_name}! Deleting it.")
                        .as_str(),
                );
                wfm.delete_order(
                    order_id.clone().unwrap().as_str(),
                    item_name,
                    item_id,
                    "buy",
                )
                .await?;
            }
            return Ok(None);
        }
        logger::debug_file(
            "LiveScraper",
            format!("Name: {item_name}, ClosedAvgMetric: {closed_avg_metric}, Price Range: {price_range}")
                .as_str(),
            Some("69.log"),
        );
        if ((closed_avg_metric as i64) >= 30 && price_range >= 15) || price_range >= 21 {
            if active {
                if price != post_price {
                    wfm.update_order_listing(
                        order_id.clone().unwrap().as_str(),
                        post_price,
                        1,
                        visibility,
                        item_name,
                        item_id,
                        "buy",
                    )
                    .await?;
                    let df = DataFrame::new(vec![
                        Series::new("url_name", vec![item_name]),
                        Series::new("platinum", vec![post_price]),
                        Series::new("potential_profit", vec![(post_price - price)]),
                    ])?;
                    let updatede = current_orders.inner_join(&df, ["url_name"], ["url_name"])?;
                    return Ok(Some(updatede));
                } else {
                    logger::info_con("LiveScraper", format!("Your current (possibly hidden) posting on this item {item_name} for {price} plat is a good one. Recommend to make visible.").as_str());
                    return Ok(None);
                }
            } else {
                let mut buy_orders_list: Vec<(i64, f64, String, String)> = vec![];
                // Create a Vec of Tuples from the DataFrame of current orders
                if current_orders.shape().0 != 0 {
                    // Convert to Vec of Tuples
                    let platinum_values = match helper::get_column_values(
                        current_orders.clone(),
                        None,
                        "platinum",
                        ColumnType::I64,
                    )? {
                        ColumnValues::I64(values) => values,
                        _ => {
                            return Err(GlobleError::OtherError("Expected f64 values".to_string()))
                        }
                    };
                    let potential_profit_values = match helper::get_column_values(
                        current_orders.clone(),
                        None,
                        "potential_profit",
                        ColumnType::F64,
                    )? {
                        ColumnValues::F64(values) => values,
                        _ => {
                            return Err(GlobleError::OtherError("Expected f64 values".to_string()))
                        }
                    };

                    let url_name_values = match helper::get_column_values(
                        current_orders.clone(),
                        None,
                        "url_name",
                        ColumnType::String,
                    )? {
                        ColumnValues::String(values) => values,
                        _ => {
                            return Err(GlobleError::OtherError("Expected f64 values".to_string()))
                        }
                    };
                    let id_values = match helper::get_column_values(
                        current_orders.clone(),
                        None,
                        "id",
                        ColumnType::String,
                    )? {
                        ColumnValues::String(values) => values,
                        _ => {
                            return Err(GlobleError::OtherError("Expected f64 values".to_string()))
                        }
                    };
                    buy_orders_list = platinum_values
                        .into_iter()
                        .zip(potential_profit_values.into_iter())
                        .zip(url_name_values.into_iter())
                        .zip(id_values.into_iter())
                        .map(|(((platinum, profit), url_name), id)| {
                            (platinum, profit, url_name, id)
                        })
                        .collect();
                }
                buy_orders_list.append(&mut vec![(
                    post_price,
                    potential_profit,
                    item_name.to_string(),
                    "".to_string(),
                )]);

                let (max_profit, selected_buy_orders, unselected_buy_orders) =
                    self.knapsack(buy_orders_list, max_total_price_cap as i64)?;

                logger::debug_file(
                    "LiveScraper",
                    format!(
                        "Name: {item_name}, MaxProfit: {max_profit}, {:?}, {:?}",
                        selected_buy_orders, unselected_buy_orders
                    )
                    .as_str(),
                    Some("knapsack.log"),
                );

                let selected_item_names: Vec<String> = selected_buy_orders
                    .iter()
                    .map(|order| order.2.clone())
                    .collect();

                if selected_item_names.contains(&item_name.to_string()) {
                    logger::debug_file(
                        "LiveScraper",
                        format!(
                            "Name: {item_name}, MaxProfit: {max_profit}, {:?}, {:?}",
                            selected_buy_orders, unselected_buy_orders
                        )
                        .as_str(),
                        Some("knapsack2.log"),
                    );
                    if !unselected_buy_orders.is_empty() {
                        let unselected_item_names: Vec<String> = unselected_buy_orders
                            .iter()
                            .map(|order| order.2.clone())
                            .collect();

                        current_orders =
                            current_orders
                                .lazy()
                                .filter(col("url_name").is_in(
                                    lit(Series::new("url_name", unselected_item_names)).not(),
                                ))
                                .collect()?;

                        for unselected_item in &unselected_buy_orders {
                            wfm.delete_order(unselected_item.3.as_str(), item_name, item_id, "buy")
                                .await?;
                            logger::debug_con(
                                "component",
                                format!(
                                    "DELETED BUY order for {} since it is not as optimal",
                                    unselected_item.2
                                )
                                .as_str(),
                            );
                        }
                    }
                    let new_order = wfm
                        .post_ordre(item_name, item_id, "buy", post_price, 1, true, item_rank)
                        .await?;
                    let current_orders =
                        self.get_new_buy_data(current_orders.clone(), new_order, item_closed_avg)?;
                    return Ok(Some(current_orders));
                } else {
                    logger::info_con("LiveScraper",format!("Item {item_name} is too expensive or less optimal than current listings").as_str());
                }
            }
        } else if active {
            logger::info_con("LiveScraper",format!("Item {item_name} Not a good time to have an order up on this item. Deleted buy order for {price}").as_str());
            wfm.delete_order(
                order_id.clone().unwrap().as_str(),
                item_name,
                item_id,
                "buy",
            )
            .await?;
        }

        Ok(None)
    }
    async fn compare_live_orders_when_selling(
        &self,
        item_name: &str,
        item_id: &str,
        item_rank: Option<f64>,
        current_orders: DataFrame,
        item_live_orders_df: &DataFrame,
        _item_stats: &DataFrame,
        _inventory_df: &DataFrame,
    ) -> Result<(), GlobleError> {
        let wfm = self.wfm.lock()?.clone();
        let db = self.db.lock()?.clone();

        // Get the current orders for the item from the Warframe Market API
        let (order_id, visibility, price, active) = self
            .get_my_order_information(item_name, &current_orders)
            .await?;

        let inventory_names = db.get_inventory_names().await?;

        if !inventory_names.contains(&item_name.to_string()) && !active {
            return Ok(());
        } else if !inventory_names.contains(&item_name.to_string()) {
            db.update_inventory_by_url(item_name.to_string(), None)
                .await?;
            wfm.delete_order(
                order_id.clone().unwrap().as_str(),
                item_name,
                item_id,
                "sell",
            )
            .await?;
            logger::info_con(
                "LiveScraper",
                format!(
                    "Item {item_name} is not in your inventory. Deleted sell order for {price}"
                )
                .as_str(),
            );
            return Ok(());
        }

        // Get Invantory Data from the database
        let inventory = db
            .get_inventory_by_url(item_name.to_string())
            .await?
            .unwrap();

        // Get all the live orders for the item from the Warframe Market API
        let (_live_buy_orders_df, live_sell_orders_df, _buyers, sellers, _price_range) =
            self.restructure_live_order_df(item_live_orders_df).await?;

        // Get the average price of the item.
        let avg_price = (inventory.price * inventory.owned as f64 / inventory.owned as f64) as i64;

        // Get the quantity of owned item.
        let quantity = inventory.owned as i64;

        logger::info_con(
            "LiveScraper",
            format!("Compare_live_orders_when_selling Checking item: {item_id}").as_str(),
        );

        // If there are no buyers, update order to be 30p above average price
        if sellers == 0 {
            let post_price = (avg_price + 30) as i64;
            db.update_inventory_by_url(item_name.to_string(), Some(post_price))
                .await?;
            if active {
                wfm.update_order_listing(
                    order_id.clone().unwrap().as_str(),
                    post_price,
                    quantity,
                    visibility,
                    item_name,
                    item_id,
                    "sell",
                )
                .await?;
                return Ok(());
            } else {
                wfm.post_ordre(
                    item_name, item_id, "sell", post_price, quantity, true, item_rank,
                )
                .await?;
                return Ok(());
            }
        }

        // Get lowest buy order price
        let post_prices: Vec<i64> = match helper::get_column_values(
            live_sell_orders_df.clone(),
            None,
            "platinum",
            ColumnType::I64,
        )? {
            ColumnValues::I64(values) => values,
            _ => return Err(GlobleError::OtherError("Expected i64 values".to_string())),
        };

        let mut post_price = post_prices.get(0).unwrap_or(&0).clone();

        if (post_price - avg_price as i64) <= -10 {
            logger::info_con(
                "LiveScraper",
                format!("Item {item_name} is too cheap. Not putting up a sell order.").as_str(),
            );
            // SEND NOTIFICATION TO DISCORD HERE AND SAY THAT THE ITEM IS TOO CHEAP
        }

        if post_price + 10 > post_price && sellers >= 2 {
            post_price = (avg_price + 10).max(post_prices.get(0).unwrap_or(&0).clone());
        } else {
            post_price = (avg_price + 10).max(post_price);
        }
        if active {
            if price != post_price {
                wfm.update_order_listing(
                    order_id.clone().unwrap().as_str(),
                    post_price,
                    quantity,
                    visibility,
                    item_name,
                    item_id,
                    "sell",
                )
                .await?;
                db.update_inventory_by_url(item_name.to_string(), Some(post_price))
                    .await?;
                logger::info_con(
                    "LiveScraper",
                    format!(
                        "Automatically updated order {} for {} from {} to {} plat",
                        order_id.unwrap_or("None".to_string()),
                        item_name,
                        price,
                        post_price
                    )
                    .as_str(),
                );
                return Ok(());
            } else {
                logger::info_con("LiveScraper", format!("Your current (possibly hidden) posting on this item {item_name} for {price} plat is a good one. Recommend to make visible.").as_str());
                return Ok(());
            }
        } else {
            wfm.post_ordre(
                item_name, item_id, "sell", post_price, quantity, true, item_rank,
            )
            .await?;
            db.update_inventory_by_url(item_name.to_string(), Some(post_price))
                .await?;
            logger::info_con("LiveScraper",format!("Automatically Posted Visible Sell Order Item: {item_name}, ItemId: {item_id}, Price: {post_price}").as_str());
        }
        Ok(())
    }
    fn get_new_buy_data(
        &self,
        current_orders: DataFrame,
        order: Order,
        item_closed_avg: f64,
    ) -> Result<DataFrame, GlobleError> {
        let mut order_df = self.wfm.lock()?.convet_order_to_datafream(order.clone())?;
        order_df = order_df
            .with_column(Series::new(
                "potential_profit",
                vec![item_closed_avg - order.platinum.clone() as f64],
            ))
            .cloned()?
            .with_column(Series::new("closedAvg", vec![item_closed_avg]))
            .cloned()?;
        Ok(helper::merge_dataframes(vec![current_orders, order_df])?)
    }
}
