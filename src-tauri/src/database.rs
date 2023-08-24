use once_cell::sync::Lazy;
use polars::prelude::*;
use rusqlite::Connection;
use std::sync::Mutex;

use crate::{helper::{self, ColumnType, ColumnValues}, structs::{Invantory}};

pub static DB_PATH: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("".to_string()));

pub fn get_connection() -> rusqlite::Result<Connection> {
    let db_path = DB_PATH.lock().unwrap();
    let conn = Connection::open(db_path.as_ref() as &std::path::Path).unwrap();
    Ok(conn)
}

pub fn get_inventory_names() -> Result<Vec<String>, rusqlite::Error> {
    let names = match helper::get_column_values(get_inventory()?, Some(col("owned").gt(0)), "item_url", ColumnType::String).expect("") {
        ColumnValues::String(values) => values,
        _ => return Err(rusqlite::Error::InvalidQuery),
    };
    Ok(names)
}

pub fn get_inventory() -> Result<DataFrame, rusqlite::Error> {
    let conn = get_connection().unwrap();
    let inventory_vec: Result<Vec<Invantory>, rusqlite::Error> = conn
        .prepare("SELECT * FROM Inventorys")?
        .query_map([], |row| {
            Ok(Invantory {
                id: row.get(0)?,
                item_id: row.get(1)?,
                item_url: row.get(2)?,
                item_name: row.get(3)?,
                rank: row.get(4)?,
                price: row.get(5)?,
                listed_price: row.get(6)?,
                owned: row.get(7)?,
            })
        })?
        .collect();
    let inventory_vec = inventory_vec?;

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
