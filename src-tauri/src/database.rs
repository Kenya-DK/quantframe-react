use crate::{
    cache::CacheState,
    error::AppError,
    helper::{self, ColumnType, ColumnValues},
    logger,
    structs::{Invantory, Order, Transaction},
    wfm_client::WFMClientState,
};
use eyre::eyre;
use polars::{
    lazy::dsl::col,
    prelude::{DataFrame, NamedFrom},
    series::Series,
};
use serde_json::json;
use sqlx::{migrate::MigrateDatabase, Pool, Row, Sqlite, SqlitePool};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct DatabaseClient {
    pub log_file: String,
    connection: Arc<Mutex<Pool<Sqlite>>>,
    cache: Arc<Mutex<CacheState>>,
    wfm: Arc<Mutex<WFMClientState>>,
}

impl DatabaseClient {
    pub async fn new(
        cache: Arc<Mutex<CacheState>>,
        wfm: Arc<Mutex<WFMClientState>>,
    ) -> Result<Self, AppError> {
        let log_file = "db.log";
        let mut db_url = helper::get_app_roaming_path();
        db_url.push("quantframe.sqlite");
        let db_url: &str = db_url.to_str().unwrap();
        if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
            match Sqlite::create_database(db_url).await {
                Ok(_) => logger::info_con(
                    "Database",
                    format!("Database created at {}", db_url).as_str(),
                ),
                Err(error) => logger::critical(
                    "Database",
                    format!("Error creating database: {:?}", error).as_str(),
                    true,
                    Some(log_file),
                ),
            }
        }
        Ok(DatabaseClient {
            log_file: log_file.to_string(),
            connection: Arc::new(Mutex::new(SqlitePool::connect(db_url).await.unwrap())),
            cache,
            wfm,
        })
    }
    // Initialize the database
    pub async fn initialize(&self) -> Result<bool, AppError> {
        logger::info("Database", "Initialize", true, None);
        let connection = self.connection.lock().unwrap().clone();
        sqlx::query(
            "
                CREATE TABLE IF NOT EXISTS inventorys (
                id integer not null primary key autoincrement,
                item_id text not null,
                item_url text not null,
                item_name text not null,
                item_type text not null,
                rank integer not null default 0,
                price REAL not null default 0,
                listed_price INT default null,
                owned INT not null default 1
            )",
        )
        .execute(&connection)
        .await
        .map_err(|e| AppError("Database", eyre!(e.to_string())))?;
        sqlx::query(
            "
        CREATE TABLE IF NOT EXISTS transactions (
            id integer not null primary key autoincrement,
            item_id text not null,
            item_type text not null,
            item_url text not null,
            item_name text not null,
            item_tags text not null,
            datetime TEXT,
            transaction_type TEXT,
            quantity integer not null default 1,
            rank integer not null default 0,
            price integer not null default 0
        )",
        )
        .execute(&connection)
        .await
        .map_err(|e| AppError("Database", eyre!(e.to_string())))?;
        Ok(true)
    }
    pub fn get_connection(&self) -> Arc<Mutex<Pool<Sqlite>>> {
        self.connection.clone()
    }
    pub async fn get_inventory_names(&self) -> Result<Vec<String>, AppError> {
        let names = match helper::get_column_values(
            self.get_inventorys_df().await?,
            Some(col("owned").gt(0)),
            "item_url",
            ColumnType::String,
        )
        .expect("")
        {
            ColumnValues::String(values) => values,
            _ => return Err(AppError("Database", eyre!(""))),
        };
        Ok(names)
    }
    pub async fn get_inventorys_df(&self) -> Result<DataFrame, AppError> {
        let inventory_vec = self.get_inventorys().await?;

        let df = DataFrame::new(vec![
            Series::new("id", inventory_vec.iter().map(|i| i.id).collect::<Vec<_>>()),
            Series::new(
                "item_id",
                inventory_vec
                    .iter()
                    .map(|i| i.item_id.clone())
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "item_url",
                inventory_vec
                    .iter()
                    .map(|i| i.item_url.clone())
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "item_name",
                inventory_vec
                    .iter()
                    .map(|i| i.item_name.clone())
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "item_type",
                inventory_vec
                    .iter()
                    .map(|i| i.item_type.clone())
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "rank",
                inventory_vec.iter().map(|i| i.rank).collect::<Vec<_>>(),
            ),
            Series::new(
                "price",
                inventory_vec.iter().map(|i| i.price).collect::<Vec<_>>(),
            ),
            Series::new(
                "listed_price",
                inventory_vec
                    .iter()
                    .map(|i| i.listed_price)
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "owned",
                inventory_vec.iter().map(|i| i.owned).collect::<Vec<_>>(),
            ),
        ]);
        Ok(df.unwrap())
    }
    pub async fn get_transactions(&self, sql: &str) -> Result<Vec<Transaction>, AppError> {
        let connection = self.connection.lock().unwrap().clone();

        let inventory_vec: Vec<Transaction> = sqlx::query(sql)
            .fetch_all(&connection)
            .await
            .unwrap()
            .into_iter()
            .map(|row| Transaction {
                id: row.get(0),
                item_id: row.get(1),
                item_type: row.get(2),
                item_url: row.get(3),
                item_name: row.get(4),
                item_tags: row.get(5),
                datetime: row.get(6),
                transaction_type: row.get(7),
                quantity: row.get(8),
                rank: row.get(9),
                price: row.get(10),
            })
            .collect();
        Ok(inventory_vec)
    }
    pub async fn get_inventorys(&self) -> Result<Vec<Invantory>, AppError> {
        let connection = self.connection.lock().unwrap().clone();

        let inventory_vec: Vec<Invantory> = sqlx::query("SELECT * FROM inventorys;")
            .fetch_all(&connection)
            .await
            .unwrap()
            .into_iter()
            .map(|row| Invantory {
                id: row.get(0),
                item_id: row.get(1),
                item_url: row.get(2),
                item_name: row.get(3),
                item_type: row.get(4),
                rank: row.get(5),
                price: row.get(6),
                listed_price: row.get(7),
                owned: row.get(8),
            })
            .collect();
        Ok(inventory_vec)
    }

    pub async fn create_transaction_entry(
        &self,
        item_id: String,
        transaction_type: String,
        quantity: i64,
        rank: i64,
        price: i64,
    ) -> Result<Transaction, AppError> {
        let connection = self.connection.lock().unwrap().clone();
        let item = self.cache.lock()?.get_item_by_url_name(&item_id);
        if item.is_none() {
            return Err(AppError(
                "Database",
                eyre!("Could not find item with id {}", item_id),
            ));
        }

        let item = item.unwrap();
        let transaction = Transaction {
            id: -1,
            item_id,
            item_type: "item".to_string(),
            item_url: item.url_name,
            item_name: item.item_name,
            item_tags: item.tags.unwrap().join(","),
            datetime: chrono::Local::now().to_string(),
            transaction_type,
            quantity,
            rank,
            price,
        };
        let result = sqlx::query(
            "INSERT INTO transactions (item_id, item_type, item_url, item_name,item_tags, datetime, transaction_type, quantity, rank, price) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)")
            .bind(transaction.clone().item_id)
            .bind(transaction.clone().item_type)
            .bind(transaction.clone().item_url)
            .bind(transaction.clone().item_name)
            .bind(transaction.clone().item_tags)
            .bind(transaction.clone().datetime)
            .bind(transaction.clone().transaction_type)
            .bind(transaction.clone().quantity)
            .bind(transaction.clone().rank)
            .bind(transaction.clone().price)
            .execute(&connection).await.map_err(|e| AppError("Database", eyre!(e.to_string())))?;

        let transaction = Transaction {
            id: result.last_insert_rowid(),
            ..transaction
        };
        helper::send_message_to_window(
            "update_data",
            Some(json!({ "type": "transactions",
                "operation": "create",
                "data": transaction.clone()
            })),
        );
        logger::info(
            "Database",
            format!("Created transaction entry with id {}", transaction.id).as_str(),
            true,
            Some(self.log_file.as_str()),
        );
        Ok(transaction)
    }

    pub async fn get_inventory_by_url_name(
        &self,
        url_name: String,
    ) -> Result<Option<Invantory>, AppError> {
        let inventorys = self.get_inventorys().await?;
        let inventory = inventorys.iter().find(|t| t.item_url == url_name);
        Ok(inventory.cloned())
    }
    pub async fn create_inventory_entry(
        &self,
        url_name: String,
        report: bool,
        mut quantity: i64,
        price: i64,
        rank: i64,
    ) -> Result<Invantory, AppError> {
        let inventorys = self.get_inventory_by_url_name(url_name.clone()).await?;
        let connection = self.connection.lock().unwrap().clone();
        let wfm = self.wfm.lock()?.clone();
        let operation = match inventorys {
            Some(_) => "update",
            None => "create",
        };

        if quantity <= 0 {
            quantity = 1;
        }

        let item = self.cache.lock()?.get_item_by_url_name(&url_name).unwrap();
        let inventory = match inventorys {
            Some(t) => {
                let total_owned = t.owned + quantity;
                // Get price per unit
                let total_price = (t.price * t.owned as f64) + price as f64;
                let weighted_price = total_price / total_owned as f64;
                sqlx::query("UPDATE inventorys SET owned = ?1, price = ?2 WHERE id = ?3")
                    .bind(total_owned)
                    .bind(weighted_price)
                    .bind(t.id)
                    .execute(&connection)
                    .await
                    .map_err(|e| AppError("Database", eyre!(e.to_string())))?;
                let mut t = t.clone();
                t.owned = total_owned;
                t.price = weighted_price;
                t
            }
            None => {
                let price = price / quantity;
                let result = sqlx::query(
                    "INSERT INTO inventorys (item_id, item_url, item_name,item_type, rank, price, owned) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")
                    .bind(item.clone().id)
                    .bind(item.clone().url_name)
                    .bind(item.clone().item_name)
                    .bind("item")
                    .bind(rank)
                    .bind(price)
                    .bind(quantity)
                    .execute(&connection).await.map_err(|e| AppError("Database", eyre!(e.to_string())))?;

                let inventory = Invantory {
                    id: result.last_insert_rowid(),
                    item_id: item.clone().id,
                    item_url: item.clone().url_name,
                    item_name: item.clone().item_name,
                    item_type: "item".to_string(),
                    rank: rank as i64,
                    price: price as f64,
                    listed_price: None,
                    owned: quantity,
                };
                inventory
            }
        };
        self.create_transaction_entry(
            item.clone().url_name,
            "buy".to_string(),
            quantity,
            rank,
            price,
        )
        .await?;

        // Send Close Event to Warframe Market API
        if report {
            logger::info(
                "Database",
                format!("Closing order for item {}", item.clone().url_name).as_str(),
                true,
                Some(self.log_file.as_str()),
            );
            wfm.close_order_by_url(&item.clone().url_name).await?;
        }

        helper::send_message_to_window(
            "update_data",
            Some(json!({ "type": "inventorys",
                "operation": operation,
                "data": inventory.clone()
            })),
        );
        logger::info(
            "Database",
            format!("Created inventory entry with id {}", inventory.id).as_str(),
            true,
            Some(self.log_file.as_str()),
        );
        Ok(inventory)
    }
    pub async fn sell_invantory_entry(
        &self,
        id: i64,
        report: bool,
        price: i64,
        mut quantity: i64,
    ) -> Result<Invantory, AppError> {
        let inventorys = self.get_inventorys().await?;
        let wfm = self.wfm.lock()?.clone();
        let inventory = inventorys.iter().find(|t| t.id == id).clone();
        if inventory.is_none() {
            return Err(AppError(
                "Database",
                eyre!("Could not find inventory with id {}", id),
            ));
        }
        let connection = self.connection.lock().unwrap().clone();
        let mut inventory = inventory.unwrap().clone();
        inventory.owned -= 1;

        if quantity <= 0 {
            quantity = 1;
        }

        self.create_transaction_entry(
            inventory.clone().item_url,
            "sell".to_string(),
            quantity,
            0,
            price,
        )
        .await?;
        if inventory.owned <= 0 {
            self.delete_inventory_entry(id).await?;
        } else {
            sqlx::query("UPDATE inventorys SET owned = ?1 WHERE id = ?2")
                .bind(inventory.clone().owned)
                .bind(inventory.clone().id)
                .execute(&connection)
                .await
                .map_err(|e| AppError("Database", eyre!(e.to_string())))?;
            helper::send_message_to_window(
                "update_data",
                Some(json!({ "type": "inventorys",
                    "operation": "update",
                    "data": inventory.clone()
                })),
            );
        }
        // Send Close Event to Warframe Market API
        if report {
            logger::info(
                "Database",
                format!("Closing order for item {}", id).as_str(),
                true,
                Some(self.log_file.as_str()),
            );
            wfm.close_order_by_url(&inventory.item_url).await?;
        } else {
            let ordres: Vec<Order> = wfm.get_user_ordres().await?.sell_orders;
            let order = ordres
                .iter()
                .find(|order| order.item.url_name == inventory.item_url)
                .clone();

            if order.is_some() {
                if inventory.owned <= 0 {
                    wfm.delete_order(
                        &order.unwrap().id,
                        &inventory.item_name,
                        &inventory.item_id,
                        "sell",
                    )
                    .await?;
                } else {
                    wfm.update_order_listing(
                        &order.unwrap().id,
                        order.unwrap().platinum,
                        inventory.owned,
                        order.unwrap().visible,
                        &inventory.item_name,
                        &inventory.item_id,
                        "sell",
                    )
                    .await?;
                }
            }
        }
        Ok(inventory.clone())
    }
    pub async fn delete_inventory_entry(&self, id: i64) -> Result<Option<Invantory>, AppError> {
        let inventorys = self.get_inventorys().await?;
        let inventory = inventorys.iter().find(|t| t.id == id).clone();
        if inventory.is_none() {
            return Ok(None);
        }
        let connection = self.connection.lock().unwrap().clone();
        sqlx::query("DELETE FROM inventorys WHERE id = ?1")
            .bind(id)
            .execute(&connection)
            .await
            .map_err(|e| AppError("Database", eyre!(e.to_string())))?;

        helper::send_message_to_window(
            "update_data",
            Some(json!({ "type": "inventorys",
                "operation": "delete",
                "data": inventory
            })),
        );
        logger::info(
            "Database",
            format!("Deleted inventory entry with id {}", id).as_str(),
            true,
            Some(self.log_file.as_str()),
        );
        Ok(Some(inventory.unwrap().clone()))
    }
    pub async fn get_inventory_by_url(
        &self,
        item_url: String,
    ) -> Result<Option<Invantory>, AppError> {
        let inventorys = self.get_inventorys().await?;
        let inventory = inventorys.iter().find(|t| t.item_url == item_url).clone();
        Ok(inventory.cloned())
    }

    pub async fn update_inventory_by_url(
        &self,
        item_url: String,
        listed_price: Option<i64>,
    ) -> Result<bool, AppError> {
        let inventory = self.get_inventory_by_url(item_url.to_string()).await?;
        if inventory.is_none() {
            return Ok(false);
        }
        let mut inventory = inventory.unwrap();
        let connection = self.connection.lock().unwrap().clone();
        let result = sqlx::query("UPDATE inventorys SET listed_price = ?1 WHERE id = ?2")
            .bind(listed_price)
            .bind(inventory.id.clone())
            .execute(&connection)
            .await
            .map_err(|e| AppError("Database", eyre!(e.to_string())))?;
        inventory.listed_price = listed_price;
        helper::send_message_to_window(
            "update_data",
            Some(json!({ "type": "inventorys",
                "operation": "update",
                "data": inventory
            })),
        );
        Ok(true)
    }
}
