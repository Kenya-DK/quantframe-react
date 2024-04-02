use crate::{database::client::DBClient, helper, utils::{enums::log_level::LogLevel, modules::{error::AppError, logger}}};
use eyre::eyre;
use sea_query::{ColumnDef, Expr, Iden, InsertStatement, Query, SqliteQueryBuilder, Table, Value};
use serde::{Deserialize, Serialize};

pub struct TransactionModule<'a> {
    pub client: &'a DBClient,
}
#[derive(Iden)]
pub enum Transaction {
    Table,
    Id,
    WFMId,
    Url,
    Name,
    ItemType,
    Tags,
    TransactionType,
    Rank,
    Price,
    Quantity,
    Created,
    Properties,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct TransactionStruct {
    pub id: i64,
    pub wfm_id: String,
    pub url: String,
    pub name: String,
    pub item_type: String,
    pub tags: String,
    pub transaction_type: String,
    pub quantity: i32,
    pub rank: i32,
    pub price: i32,
    pub created: String,
    pub properties: Option<sqlx::types::Json<Option<serde_json::Value>>>,
}
impl<'a> TransactionModule<'a> {
    pub async fn initialize(&self) -> Result<bool, AppError> {
        let connection = self.client.connection.lock().unwrap().clone();
        let sql = Table::create()
            .table(Transaction::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Transaction::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(Transaction::WFMId).uuid().not_null())
            .col(ColumnDef::new(Transaction::Url).string().not_null())
            .col(ColumnDef::new(Transaction::Name).string().not_null())
            .col(ColumnDef::new(Transaction::ItemType).string().not_null())
            .col(ColumnDef::new(Transaction::Tags).string().not_null())
            .col(
                ColumnDef::new(Transaction::TransactionType)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Transaction::Quantity)
                    .integer()
                    .not_null()
                    .default(Value::Int(Some(1))),
            )
            .col(
                ColumnDef::new(Transaction::Rank)
                    .integer()
                    .not_null()
                    .default(Value::Int(Some(0))),
            )
            .col(
                ColumnDef::new(Transaction::Price)
                    .integer()
                    .not_null()
                    .default(Value::Int(Some(0))),
            )
            .col(ColumnDef::new(Transaction::Properties).json())
            .col(ColumnDef::new(Transaction::Created).date_time().not_null())
            .build(SqliteQueryBuilder);

        sqlx::query(&sql)
            .execute(&connection)
            .await
            .map_err(|e| AppError::new("Database", eyre!(e.to_string())))?;
        Ok(true)
    }

    pub async fn get_items(&self) -> Result<Vec<TransactionStruct>, AppError> {
        let connection = self.client.connection.lock().unwrap().clone();
        // Read
        let sql = Query::select()
            .columns([
                Transaction::Id,
                Transaction::TransactionType,
                Transaction::WFMId,
                Transaction::Url,
                Transaction::Name,
                Transaction::ItemType,
                Transaction::Tags,
                Transaction::Rank,
                Transaction::Price,
                Transaction::Quantity,
                Transaction::Properties,
                Transaction::Created,
            ])
            .from(Transaction::Table)
            .to_string(SqliteQueryBuilder);

        let rows = sqlx::query_as::<_, TransactionStruct>(&sql)
            .fetch_all(&connection)
            .await
            .unwrap();
        Ok(rows)
    }
    pub async fn get_by_id(&self, id: i64) -> Result<Option<TransactionStruct>, AppError> {
        let transactions = self.get_items().await?;
        let transaction = transactions.iter().find(|t| t.id == id);
        Ok(transaction.cloned())
    }
    pub async fn create(
        &self,
        url_name: &str,
        item_type: &str,
        transaction_type: &str,
        quantity: i32,
        price: i32,
        rank: i32,
        properties: Option<serde_json::Value>,
    ) -> Result<TransactionStruct, AppError> {
        let connection = self.client.connection.lock().unwrap().clone();
        let mut transaction = TransactionStruct {
            id: 0,
            wfm_id: "".to_string(),
            url: "".to_string(),
            name: "".to_string(),
            item_type: item_type.to_string(),
            tags: "".to_string(),
            rank,
            properties: Some(sqlx::types::Json(properties.clone())),
            price,
            transaction_type: transaction_type.to_string(),
            quantity,
            created: chrono::Utc::now().to_rfc3339(),
        };
        if item_type == "riven" {
            let item = self
                .client
                .cache
                .lock()?
                .riven()
                .find_type(&url_name)?
                .unwrap();
            transaction.wfm_id = item.id.clone();
            transaction.url = item.url_name.clone();
            transaction.name = item.item_name.clone();
            transaction.tags = item.riven_type.clone().unwrap_or("Unknown".to_string());
        } else if item_type == "item" {
            let item = self
                .client
                .cache
                .lock()?
                .item()
                .find_type(&url_name)?
                .unwrap();
            transaction.wfm_id = item.id.clone();
            transaction.url = item.url_name.clone();
            transaction.name = item.item_name.replace("\'", "").clone();
            transaction.tags = item.tags.unwrap().join(",");
        }

        logger::info_con(
            "Database",
            format!(
                "Creating Transaction: {} {} {} {} {}",
                transaction.wfm_id,
                transaction.url,
                transaction.name,
                transaction.item_type,
                transaction.rank
            )
            .as_str(),
        );

        let sql = InsertStatement::default()
            .into_table(Transaction::Table)
            .columns([
                Transaction::WFMId,
                Transaction::Url,
                Transaction::Name,
                Transaction::ItemType,
                Transaction::Tags,
                Transaction::Rank,
                Transaction::Properties,
                Transaction::Price,
                Transaction::TransactionType,
                Transaction::Quantity,
                Transaction::Created,
            ])
            .values_panic([
                transaction.wfm_id.clone().into(),
                transaction.url.clone().into(),
                transaction.name.clone().into(),
                transaction.item_type.clone().into(),
                transaction.tags.clone().into(),
                transaction.rank.into(),
                properties.into(),
                transaction.price.into(),
                transaction.transaction_type.clone().into(),
                transaction.quantity.into(),
                transaction.created.clone().into(),
            ])
            .to_string(SqliteQueryBuilder);
        let row = sqlx::query(&sql.replace("\\", ""))
            .execute(&connection)
            .await
            .map_err(|e| AppError::new("Database", eyre!(e.to_string())))?;
        let id = row.last_insert_rowid();
        transaction.id = id;
        self.emit(
            "CREATE_OR_UPDATE",
            serde_json::to_value(transaction.clone()).unwrap(),
        );
        Ok(transaction)
    }
    pub async fn update_by_id(
        &self,
        id: i64,
        price: Option<i64>,
        transaction_type: Option<String>,
        quantity: Option<i64>,
        rank: Option<i64>,
    ) -> Result<TransactionStruct, AppError> {
        let connection = self.client.connection.lock().unwrap().clone();
        let items = self.get_items().await?;
        let transaction = items.iter().find(|t| t.id == id);
        if transaction.is_none() {
            return Err(AppError::new_with_level(
                "Database",
                eyre!("Transaction not found in database"),
                LogLevel::Error,
            ));
        }
        let mut transaction = transaction.unwrap().clone();
        let mut values = vec![];

        if price.is_some() {
            transaction.price = price.unwrap() as i32;
            values.push((Transaction::Price, price.into()));
        }

        if transaction_type.is_some() {
            transaction.transaction_type = transaction_type.unwrap();
            values.push((
                Transaction::TransactionType,
                transaction.transaction_type.clone().into(),
            ));
        }

        if quantity.is_some() && quantity.unwrap() > -1 {
            transaction.quantity = quantity.unwrap() as i32;
            values.push((Transaction::Quantity, quantity.into()));
        }

        if rank.is_some() {
            transaction.rank = rank.unwrap() as i32;
            values.push((Transaction::Rank, rank.into()));
        }
        logger::info_con(
            "Database",
            format!(
                "Updating Transaction: {} {} {} {}",
                transaction.price,
                transaction.transaction_type,
                transaction.quantity,
                transaction.rank
            )
            .as_str(),
        );
        let sql = Query::update()
            .table(Transaction::Table)
            .values(values)
            .and_where(Expr::col(Transaction::Id).eq(id))
            .to_string(SqliteQueryBuilder);
        sqlx::query(&sql.replace("\\", ""))
            .execute(&connection)
            .await
            .map_err(|e| AppError::new("Database", eyre!(e.to_string())))?;

        self.emit(
            "CREATE_OR_UPDATE",
            serde_json::to_value(transaction.clone()).unwrap(),
        );
        Ok(transaction.clone())
    }

    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        let connection = self.client.connection.lock().unwrap().clone();

        logger::info_con("Database", format!("Deleting Transaction: {}", id).as_str());
        let sql = Query::delete()
            .from_table(Transaction::Table)
            .and_where(Expr::col(Transaction::Id).eq(id))
            .to_string(SqliteQueryBuilder);
        sqlx::query(&sql)
            .execute(&connection)
            .await
            .map_err(|e| AppError::new("Database", eyre!(e.to_string())))?;
        Ok(())
    }

    pub fn emit(&self, operation: &str, data: serde_json::Value) {
        helper::emit_update("transactions", operation, Some(data));
    }
}
