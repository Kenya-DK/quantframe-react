use crate::{
    auth::AuthState, cache::CacheState, database::DatabaseClient, error::AppError, logger,
    settings::SettingsState, structs::Ordres, wfm_client::WFMClientState,
};
use eyre::eyre;
use serde_json::Value;
use sqlx::{Pool, Row, Sqlite, SqlitePool};
use std::{
    fs::File,
    sync::{Arc, Mutex},
};

#[derive(Clone, Debug)]
pub struct DebugClient {
    log_file: String,
    cache: Arc<Mutex<CacheState>>,
    wfm: Arc<Mutex<WFMClientState>>,
    auth: Arc<Mutex<AuthState>>,
    db: Arc<Mutex<DatabaseClient>>,
    settings: Arc<Mutex<SettingsState>>,
}

impl DebugClient {
    pub fn new(
        cache: Arc<Mutex<CacheState>>,
        wfm: Arc<Mutex<WFMClientState>>,
        auth: Arc<Mutex<AuthState>>,
        db: Arc<Mutex<DatabaseClient>>,
        settings: Arc<Mutex<SettingsState>>,
    ) -> Self {
        DebugClient {
            log_file: "debug.log".to_string(),
            cache,
            wfm,
            auth,
            db,
            settings,
        }
    }

    // TODO: Remove in production
    pub async fn import_warframe_algo_trader_data(
        &self,
        db_path: String,
        import_type: String,
    ) -> Result<bool, AppError> {
        let db = self.db.lock()?.clone();
        let db: Pool<Sqlite> = db.get_connection().clone().lock()?.clone();

        let watdb = SqlitePool::connect(db_path.as_str()).await.unwrap();

        if import_type == "inventory" {
            // Delete all data in the database to prevent duplicates and errors
            sqlx::query("DELETE FROM inventorys;").execute(&db).await.map_err(|e| {AppError("Debug", eyre!(e.to_string()))} )?;

            let inventory_vec = sqlx::query("SELECT * FROM inventory;")
                .fetch_all(&watdb)
                .await
                .unwrap();
            for row in inventory_vec {
                let name = row.try_get::<String, _>(1).unwrap();
                let price = row.try_get::<f64, _>(2).unwrap();
                let owned = row.try_get::<i64, _>(4).unwrap();

                let item = self.cache.lock()?.get_item_by_url_name(&name);
                if item.is_none() {
                    logger::error(
                        "Database",
                        format!("Could not find item with name {}", name).as_str(),
                        true,
                        Some(self.log_file.as_str()),
                    );
                    continue;
                }
                let item = item.unwrap();
                let item_id = item.id.clone();
                sqlx::query(
                    "INSERT INTO inventorys (item_id, item_url, item_name, rank, price, owned) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")
                    .bind(item_id.clone())
                    .bind(item.url_name)
                    .bind(item.item_name)
                    .bind(0)
                    .bind(price)
                    .bind(owned)
                    .execute(&db).await.map_err(|e| {AppError("Debug", eyre!(e.to_string()))} )?;
            }
        } else if import_type == "transactions" {
            // Delete all data in the database to prevent duplicates and errors
            sqlx::query("DELETE FROM transactions;")
                .execute(&db)
                .await
                .map_err(|e| AppError("Debug", eyre!(e.to_string())))?;
            let transactions_vec = sqlx::query("SELECT * FROM transactions;")
                .fetch_all(&watdb)
                .await
                .unwrap();

            for row in transactions_vec {
                let name = row.try_get::<String, _>(1).unwrap();
                let datetime = row.try_get::<String, _>(2).unwrap();
                let transaction_type = row.try_get::<String, _>(3).unwrap();
                let price = row.try_get::<i64, _>(4).unwrap();

                let item = self.cache.lock()?.get_item_by_url_name(&name);
                if item.is_none() {
                    logger::error(
                        "Database",
                        format!("Could not find item with name {}", name).as_str(),
                        true,
                        Some(self.log_file.as_str()),
                    );
                    continue;
                }
                let item = item.unwrap();
                let item_id = item.id.clone();
                sqlx::query(
                    "INSERT INTO transactions (item_id, item_type, item_url, item_name, datetime, transaction_type, quantity, rank, price) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)")
                    .bind(item_id.clone())
                                .bind(item.tags.unwrap().join(","))
                                .bind(item.url_name)
                                .bind(item.item_name)
                                .bind(datetime)
                                .bind(transaction_type)
                                .bind(1)
                                .bind(0)
                                .bind(price)
                                .execute(&db).await.map_err(|e| {AppError("Debug", eyre!(e.to_string()))} )?;
            }
        } else {
            logger::error_con(
                "Debug",
                format!("Could not find import type {}", import_type).as_str(),
            );
        }

        Ok(true)
    }

    pub async fn reset_data(&self, reset_type: String) -> Result<bool, AppError> {
        let db = self.db.lock()?.clone();
        let db: Pool<Sqlite> = db.get_connection().clone().lock()?.clone();
        if reset_type == "inventory" {
            // Delete all data in the database to prevent duplicates and errors
            sqlx::query("DELETE FROM inventorys;")
                .execute(&db)
                .await
                .map_err(|e| AppError("Debug", eyre!(e.to_string())))?;
        } else if reset_type == "transactions" {
            // Delete all data in the database to prevent duplicates and errors
            sqlx::query("DELETE FROM transactions;")
                .execute(&db)
                .await
                .map_err(|e| AppError("Debug", eyre!(e.to_string())))?;
        } else {
            logger::error_con(
                "Debug",
                format!("Could not find import type {}", reset_type).as_str(),
            );
        }
        Ok(true)
    }

    //---------------------------------Test---------------------------------//
    pub async fn dowload_string(&self, url: &str) -> Result<Value, AppError> {
        let client = reqwest::Client::new();
        let res = client
            .get(url)
            .send()
            .await
            .map_err(|e| AppError("Debug", eyre!(e.to_string())))?;
        let text = res
            .json()
            .await
            .map_err(|e| AppError("Debug", eyre!(e.to_string())))?;
        Ok(text)
    }

    pub async fn test_error(&self) -> Result<(), AppError> {
        // self.error_io().await?;
        self.error_json().await?;
        // self.error_reqwest().await?;
        // self.error_polars().await?;

        Ok(())
    }

    async fn error_io(&self) -> Result<(), AppError> {
        let filename = "13.csv";
        let file = File::open(filename).map_err(|e| AppError("Debug", eyre!(e.to_string())))?;
        println!("Done: {:?}", file);
        Ok(())
    }

    async fn error_json(&self) -> Result<(), AppError> {
        let jsom_data = self
            .dowload_string("https://api.warframe.market/v1/profile/Crystal4444_Worm/orders")
            .await?;
        let jsom_data = jsom_data["payload"].clone();
        let item: Ordres = serde_json::from_value(jsom_data)
            .map_err(|e| AppError("Debug", eyre!(e.to_string())))?;
        println!("{:?}", item.buy_orders[0].platinum);
        Ok(())
    }

    // async fn error_reqwest(&self) -> Result<(), eyre::Report> {
    //     use AppError::*;
    //     let jsom_data = self
    //         .dowload_string("https://api.warframe.market/v1/profile/Crystal_Worm/orders")
    //         .await?;
    //     let jsom_data = jsom_data["payload"].clone();
    //     let item: Ordres = serde_json::from_value(jsom_data).map_err(|e| {
    //         JsonParseError(
    //             e,
    //             "Error".to_string(),
    //             "".into(),
    //             error::convert_backtrace(backtrace::Backtrace::new()),
    //         )
    //     })?;
    //     Ok(())
    // }

    // async fn error_polars(&self) -> Result<(), eyre::Report> {
    //     use AppError::*;
    //     let jsom_data = self
    //         .dowload_string("https://api.warframe.market/v1/profile/Crystal_Worm/orders")
    //         .await?;
    //     let jsom_data = jsom_data["payload"].clone();
    //     let item: Ordres = serde_json::from_value(jsom_data).map_err(|e| {
    //         JsonParseError(
    //             e,
    //             "Error".to_string(),
    //             "".into(),
    //             error::convert_backtrace(backtrace::Backtrace::new()),
    //         )
    //     })?;
    //     Ok(())
    // }
}
