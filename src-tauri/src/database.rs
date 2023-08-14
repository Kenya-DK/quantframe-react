use once_cell::sync::Lazy;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::Window;

use crate::structs::Invantory;

pub static DB_PATH: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("".to_string()));
pub static WINDOW: Lazy<Mutex<Option<Window>>> = Lazy::new(|| Mutex::new(None));

pub fn get_connection() -> rusqlite::Result<Connection> {
    let db_path = DB_PATH.lock().unwrap();
    let conn = Connection::open(db_path.as_ref() as &std::path::Path).unwrap();
    Ok(conn)
}

pub fn get_inventory_names() -> Result<Vec<String>, rusqlite::Error> {
    let conn = get_connection().unwrap();
    let mut stmt = conn
        .prepare("SELECT item_url FROM Inventorys WHERE owned > 0")
        .unwrap();
    let inventory_names: Vec<String> = stmt
        .query_map([], |row| row.get(0))
        .unwrap()
        .map(|item_url| item_url.unwrap())
        .collect();
    Ok(inventory_names)
}

pub async fn get_inventorys(sql: &str) -> Result<Vec<Invantory>, rusqlite::Error> {
    let conn = get_connection().unwrap();
    let mut stmt = conn.prepare(sql)?;
    let inventorys = stmt.query_map([], |row| {
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
    })?;
    let mut inventorys_vec = Vec::new();
    for inventory in inventorys {
        inventorys_vec.push(inventory.unwrap());
    }
    Ok(inventorys_vec)
}
