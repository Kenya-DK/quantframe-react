use crate::{
    cache::client::CacheClient,
    database::{client::DBClient, modules::transaction::Transaction},
    error::AppError,
    logger,
};
use eyre::eyre;
use sea_query::{InsertStatement, SqliteQueryBuilder};

use serde_json::{json, Value};
use sqlx::{Pool, Row, Sqlite, SqlitePool};
use std::{
    fs,
    io::Read as _,
    path::Path,
    sync::{Arc, Mutex},
};

#[derive(Clone, Debug)]
pub struct DebugClient {
    log_file: String,
    trades: Vec<Value>,
    cache: Arc<Mutex<CacheClient>>,
    db: Arc<Mutex<DBClient>>,
}

impl DebugClient {
    pub fn new(cache: Arc<Mutex<CacheClient>>, db: Arc<Mutex<DBClient>>) -> Self {
        DebugClient {
            log_file: "debug.log".to_string(),
            cache,
            trades: Vec::new(),
            db,
        }
    }
    pub fn get_trades(&self) -> Result<Vec<Value>, AppError> {
        let app_path = crate::helper::get_app_roaming_path().join("logs");
        let mut trades: Vec<Value> = vec![];
        self.visit_dirs(&app_path, &mut trades).unwrap();
        Ok(trades)
    }
    fn read_file(&self, file_path: &Path) -> std::io::Result<String> {
        let mut contents = String::new();
        fs::File::open(file_path)?.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn visit_dirs(&self, dir: &Path, trades: &mut Vec<Value>) -> std::io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    self.visit_dirs(&path, trades)?;
                } else if path.ends_with("tradings.json") {
                    match self.read_file(&path) {
                        Ok(contents) => {
                            let trade: Vec<Value> = serde_json::from_str(&contents).unwrap();
                            let mut count = 0;
                            let top_dir = path.parent().unwrap().file_name().unwrap();

                            for mut t in trade {
                                t["id"] = json!(format!("{:?}-{}", top_dir, count));
                                trades.push(t);
                                count += 1;
                            }
                        }
                        Err(err) => eprintln!("Error reading file {}: {:?}", path.display(), err),
                    }
                }
            }
        }

        Ok(())
    }
    // TODO: Remove in production
    pub async fn import_warframe_algo_trader_data(
        &self,
        db_path: String,
        import_type: String,
    ) -> Result<bool, AppError> {
        let db = self.db.lock()?.clone();
        let connection_pool: Pool<Sqlite> = db.get_connection().clone().lock()?.clone();

        let database_connection = SqlitePool::connect(db_path.as_str()).await.unwrap();

        if import_type == "inventory" {
            // Delete all data in the database to prevent duplicates and errors
            sqlx::query("DELETE FROM 'stock_item'")
                .execute(&connection_pool)
                .await
                .map_err(|e| AppError::new("Debug", eyre!(e.to_string())))?;

            let inventory_vec = sqlx::query("SELECT * FROM inventory;")
                .fetch_all(&database_connection)
                .await
                .unwrap();
            for row in inventory_vec {
                let name = row.try_get::<String, _>(1).unwrap();
                let price = row.try_get::<f64, _>(2).unwrap();
                let owned = row.try_get::<i64, _>(4).unwrap();

                let item = self.cache.lock()?.item().find_type(&name)?;
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
                db.stock_item()
                    .create(&item.url_name, owned as i32, price, None, 0, None)
                    .await?;
            }
        } else if import_type == "transactions" {
            // Delete all data in the database to prevent duplicates and errors
            sqlx::query("DELETE FROM 'transaction'")
                .execute(&connection_pool)
                .await
                .map_err(|e| AppError::new("Debug", eyre!(e.to_string())))?;
            let transactions_vec = sqlx::query("SELECT * FROM transaction;")
                .fetch_all(&database_connection)
                .await
                .unwrap();

            for row in transactions_vec {
                let name = row.try_get::<String, _>(1).unwrap();
                let timestamp = row.try_get::<String, _>(2).unwrap();
                let transaction_type = row.try_get::<String, _>(3).unwrap();
                let price = row.try_get::<i64, _>(4).unwrap();

                let item = self.cache.lock()?.item().find_type(&name)?;
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
                let sql = InsertStatement::default()
                    .into_table(Transaction::Table)
                    .columns([
                        Transaction::WFMId,
                        Transaction::Url,
                        Transaction::Name,
                        Transaction::ItemType,
                        Transaction::Tags,
                        Transaction::Rank,
                        Transaction::Price,
                        Transaction::TransactionType,
                        Transaction::Quantity,
                        Transaction::Created,
                    ])
                    .values_panic([
                        item.id.clone().into(),
                        item.url_name.clone().into(),
                        item.item_name.clone().into(),
                        "item".into(),
                        item.tags.unwrap().join(",").clone().into(),
                        0.into(),
                        price.into(),
                        transaction_type.clone().into(),
                        1.into(),
                        timestamp.into(),
                    ])
                    .to_string(SqliteQueryBuilder);
                sqlx::query(&sql.replace("\\", ""))
                    .execute(&connection_pool)
                    .await
                    .map_err(|e| AppError::new("Database", eyre!(e.to_string())))?;
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
            sqlx::query("DELETE FROM inventory;")
                .execute(&db)
                .await
                .map_err(|e| AppError::new("Debug", eyre!(e.to_string())))?;
        } else if reset_type == "transactions" {
            // Delete all data in the database to prevent duplicates and errors
            sqlx::query("DELETE FROM transaction;")
                .execute(&db)
                .await
                .map_err(|e| AppError::new("Debug", eyre!(e.to_string())))?;
        } else {
            logger::error_con(
                "Debug",
                format!("Could not find import type {}", reset_type).as_str(),
            );
        }
        Ok(true)
    }
}
