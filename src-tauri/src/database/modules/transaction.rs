use crate::{database::client::DBClient, error::AppError, helper, structs::RivenAttribute};
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
    ItemId,
    ItemUrl,
    ItemName,
    ItemType,
    ItemTags,
    Rank,
    SubType,
    Attributes,
    MasteryRank,
    ReRolls,
    Polarity,
    Price,
    TransactionType,
    Quantity,
    Created,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct TransactionStruct {
    pub id: i64,
    pub item_id: String,
    pub item_url: String,
    pub item_name: String,
    pub item_type: String,
    pub item_tags: String,
    pub rank: i32,
    // Used for relics
    pub sub_type: Option<String>,
    // Used for riven mods
    pub attributes: sqlx::types::Json<Vec<RivenAttribute>>,
    // Used for riven mods
    pub mastery_rank: Option<i32>,
    // Used for riven mods
    pub re_rolls: Option<i32>,
    // Used for riven mods
    pub polarity: Option<String>,
    pub price: i32,
    pub transaction_type: String,
    pub quantity: i32,
    pub created: String,
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
            .col(ColumnDef::new(Transaction::ItemId).uuid().not_null())
            .col(ColumnDef::new(Transaction::ItemUrl).string().not_null())
            .col(ColumnDef::new(Transaction::ItemName).string().not_null())
            .col(ColumnDef::new(Transaction::ItemType).string().not_null())
            .col(ColumnDef::new(Transaction::ItemTags).string().not_null())
            .col(
                ColumnDef::new(Transaction::Rank)
                    .integer()
                    .not_null()
                    .default(Value::Int(Some(0))),
            )
            .col(ColumnDef::new(Transaction::SubType).string())
            .col(ColumnDef::new(Transaction::Attributes).json().not_null())
            .col(ColumnDef::new(Transaction::MasteryRank).integer())
            .col(ColumnDef::new(Transaction::ReRolls).integer())
            .col(ColumnDef::new(Transaction::Polarity).string())
            .col(
                ColumnDef::new(Transaction::Price)
                    .integer()
                    .not_null()
                    .default(Value::Int(Some(0))),
            )
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
                Transaction::ItemId,
                Transaction::ItemUrl,
                Transaction::ItemName,
                Transaction::ItemType,
                Transaction::ItemTags,
                Transaction::Rank,
                Transaction::SubType,
                Transaction::Attributes,
                Transaction::MasteryRank,
                Transaction::ReRolls,
                Transaction::Polarity,
                Transaction::Price,
                Transaction::TransactionType,
                Transaction::Quantity,
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
    pub async fn create(
        &self,
        url_name: &str,
        item_type: &str,
        transaction_type: &str,
        quantity: i32,
        price: i32,
        rank: i32,
        sub_type: Option<&str>,
        attributes: Option<Vec<RivenAttribute>>,
        mastery_rank: Option<i32>,
        re_rolls: Option<i32>,
        polarity: Option<&str>,
    ) -> Result<TransactionStruct, AppError> {
        let connection = self.client.connection.lock().unwrap().clone();
        let item = self
            .client
            .cache
            .lock()?
            .get_item_by_url_name(&url_name)
            .unwrap();
        let attributes = match attributes {
            Some(t) => t,
            None => vec![],
        };
        let mut transaction = TransactionStruct {
            id: 0,
            item_id: item.id.clone(),
            item_url: item.url_name.clone(),
            item_name: item.item_name.clone(),
            item_type: item_type.clone().to_string(),
            item_tags: item.tags.unwrap().join(","),
            rank,
            sub_type: sub_type.map(|s| s.to_string()),
            attributes: sqlx::types::Json(attributes.clone()),
            mastery_rank,
            re_rolls,
            polarity: polarity.map(|s| s.to_string()),
            price,
            transaction_type: transaction_type.to_string(),
            quantity,
            created: chrono::Utc::now().to_rfc3339(),
        };

        let sql = InsertStatement::default()
            .into_table(Transaction::Table)
            .columns([
                Transaction::ItemId,
                Transaction::ItemUrl,
                Transaction::ItemName,
                Transaction::ItemType,
                Transaction::ItemTags,
                Transaction::Rank,
                Transaction::SubType,
                Transaction::Attributes,
                Transaction::MasteryRank,
                Transaction::ReRolls,
                Transaction::Polarity,
                Transaction::Price,
                Transaction::TransactionType,
                Transaction::Quantity,
                Transaction::Created,
            ])
            .values_panic([
                transaction.item_id.clone().into(),
                transaction.item_url.clone().into(),
                transaction.item_name.clone().into(),
                transaction.item_type.clone().into(),
                transaction.item_tags.clone().into(),
                transaction.rank.into(),
                transaction.sub_type.clone().into(),
                serde_json::to_value(&transaction.attributes)
                    .unwrap()
                    .into(),
                transaction.mastery_rank.into(),
                transaction.re_rolls.into(),
                transaction.polarity.clone().into(),
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
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        let connection = self.client.connection.lock().unwrap().clone();
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
