use std::{collections::HashMap, hash::Hash};

use chrono::{
    DateTime, Datelike, Duration, Local, NaiveDate, NaiveDateTime, TimeZone, Timelike, Utc,
};
use entity::{
    dto::*,
    stock::{item::dto::StockItemPaginationQueryDto, riven::dto::StockRivenPaginationQueryDto},
    transaction::{self, dto::TransactionPaginationQueryDto},
    wish_list::dto::WishListPaginationQueryDto,
};
use sea_orm::{Database, DatabaseConnection};
use service::{StockItemQuery, StockRivenQuery, TransactionQuery, WishListQuery};
use tokio::time::timeout;

async fn get_connection() -> DatabaseConnection {
    let debug_file_name = "quantframeV2_debug.sqlite";
    // Create the database connection URL
    let db_url = format!("sqlite://{}?mode=rwc", debug_file_name,);

    // Create the database connection and store it and run the migrations
    Database::connect(db_url)
        .await
        .expect("Database connection failed")
}
