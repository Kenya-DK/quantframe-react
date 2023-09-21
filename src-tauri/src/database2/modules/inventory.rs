use crate::{
    auth::AuthState, database2::client::DBClient, error::AppError, logger, structs::Invantory,
};
use eyre::eyre;
use reqwest::header::HeaderMap;
use serde_json::json;
use sqlx::Row;
pub struct InventoryModule<'a> {
    pub client: &'a DBClient,
}

impl<'a> InventoryModule<'a> {
    // Methods
    pub async fn get_items(&self, sql: &str) -> Result<Vec<Invantory>, AppError> {
        let connection = self.client.connection.lock().unwrap().clone();

        let inventory_vec: Vec<Invantory> = sqlx::query(sql)
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

    pub async fn get_item_by_url_name(
        &self,
        url_name: &str,
    ) -> Result<Option<Invantory>, AppError> {
        let inventorys = self.get_items("SELECT * FROM inventorys;").await?;
        let inventory = inventorys.iter().find(|t| t.item_url == url_name);
        Ok(inventory.cloned())
    }

    pub async fn create(
        &self,
        url_name: String,
        mut quantity: i64,
        price: i64,
        rank: i64,
    ) -> Result<Invantory, AppError> {
        let inventorys = self.get_item_by_url_name(url_name.as_str()).await?;
        let connection = self.client.connection.lock().unwrap().clone();
        let wfm = self.client.wfm.lock()?.clone();

        if quantity <= 0 {
            quantity = 1;
        }

        let item = self
            .client
            .cache
            .lock()?
            .get_item_by_url_name(&url_name)
            .unwrap();
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
                    .map_err(|e| AppError::new("Database", eyre!(e.to_string())))?;
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
                        .execute(&connection).await.map_err(|e| AppError::new("Database", eyre!(e.to_string())))?;

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
        logger::info(
            "Database",
            format!("Created inventory entry with id {}", inventory.id).as_str(),
            true,
            Some(self.client.log_file.as_str()),
        );
        Ok(inventory)
    }

    pub async fn update(
        &self,
        id: i64,
        owned: Option<i64>,
        price: Option<i64>,
        listed_price: Option<i64>,
    ) -> Result<bool, AppError> {
        let connection = self.client.connection.lock().unwrap().clone();
        sqlx::query(
            "UPDATE inventorys SET owned = ?1, listed_price = ?2, price = ?3 WHERE id = ?4",
        )
        .bind(owned)
        .bind(listed_price)
        .bind(price)
        .bind(id)
        .execute(&connection)
        .await
        .map_err(|e| AppError::new("Database", eyre!(e.to_string())))?;

        Ok(true)
    }
    pub async fn delete(&self, id: i64) -> Result<Option<Invantory>, AppError> {
        let inventorys = self.get_items("SELECT * FROM inventorys;").await?;
        let inventory = inventorys.iter().find(|t| t.id == id).clone();
        if inventory.is_none() {
            return Ok(None);
        }
        let connection = self.client.connection.lock().unwrap().clone();
        sqlx::query("DELETE FROM inventorys WHERE id = ?1")
            .bind(id)
            .execute(&connection)
            .await
            .map_err(|e| AppError::new("Database", eyre!(e.to_string())))?;

        logger::info(
            "Database",
            format!("Deleted inventory entry with id {}", id).as_str(),
            true,
            Some(self.client.log_file.as_str()),
        );
        Ok(Some(inventory.unwrap().clone()))
    }

    // End of methods
}
