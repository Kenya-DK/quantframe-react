use crate::{
    auth::AuthState,
    database::client::DBClient,
    error::AppError,
    helper,
    logger::{self, LogLevel},
    structs::RivenAttribute,
};
use eyre::eyre;
use polars::{
    prelude::{DataFrame, NamedFrom},
    series::Series,
};
use reqwest::header::HeaderMap;
use sea_query::{ColumnDef, Expr, Iden, InsertStatement, Query, SqliteQueryBuilder, Table, Value};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::Row;

#[derive(Iden)]
pub enum StockRiven {
    Table,
    Id,
    WeaponId,
    WeaponUrl,
    WeaponName,
    WeaponType,
    ModName,
    Rank,
    Attributes,
    MasteryRank,
    ReRolls,
    Polarity,
    Price,
    ListedPrice,
    Created,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct StockRivenStruct {
    pub id: i64,
    pub weapon_id: String,
    pub weapon_url: String,
    pub weapon_name: String,
    pub weapon_type: String,
    pub mod_name: String,
    pub rank: i32,
    pub attributes: sqlx::types::Json<Vec<RivenAttribute>>,
    pub mastery_rank: Option<i32>,
    pub re_rolls: Option<i32>,
    pub polarity: Option<String>,
    pub price: f64,
    pub listed_price: Option<i32>,
    pub owned: i32,
    pub created: String,
}

pub struct StockRivenModule<'a> {
    pub client: &'a DBClient,
}

impl<'a> StockRivenModule<'a> {
    // Methods sea-query

    // Initialize the database
    pub async fn initialize(&self) -> Result<bool, AppError> {
        let connection = self.client.connection.lock().unwrap().clone();
        let sql = Table::create()
            .table(StockRiven::Table)
            .if_not_exists()
            .col(ColumnDef::new(StockRiven::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(StockRiven::WeaponId).uuid().not_null())
            .col(ColumnDef::new(StockRiven::WeaponUrl).string().not_null())
            .col(ColumnDef::new(StockRiven::WeaponName).string().not_null())
            .col(ColumnDef::new(StockRiven::WeaponType).string().not_null())
            .col(ColumnDef::new(StockRiven::ModName).string().not_null())
            .col(ColumnDef::new(StockRiven::Rank).integer().not_null().default(Value::Int(Some(0))))
            .col(ColumnDef::new(StockRiven::Attributes).json())
            .col(ColumnDef::new(StockRiven::MasteryRank).integer().not_null().default(Value::Int(Some(0))))
            .col(ColumnDef::new(StockRiven::ReRolls).integer().not_null().default(Value::Int(Some(0))))
            .col(ColumnDef::new(StockRiven::Polarity).string().not_null())
            .col(ColumnDef::new(StockRiven::Price).float().not_null().default(Value::Int(Some(0))))
            .col(ColumnDef::new(StockRiven::ListedPrice).integer().default(Value::Int(None)))
            .col(ColumnDef::new(StockRiven::Created).date_time().not_null())
            .build(SqliteQueryBuilder);

        sqlx::query(&sql)
            .execute(&connection)
            .await
            .map_err(|e| AppError::new("Database", eyre!(e.to_string())))?;
        Ok(true)
    }
}
