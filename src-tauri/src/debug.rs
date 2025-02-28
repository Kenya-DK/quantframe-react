use crate::{
    cache::client::CacheClient,
    log_parser::types::create_stock_entity::CreateStockEntity,
    notification::client::NotifyClient,
    utils::modules::{
        error::{self, AppError},
        logger::{self, LoggerOptions},
        states,
    },
};

use chrono::{DateTime, NaiveDateTime, Utc};
use entity::{
    enums::stock_type::StockType, sub_type::SubType, transaction::transaction::TransactionType,
};
use serde_json::json;
use service::{
    sea_orm::DatabaseConnection, StockItemMutation, StockItemQuery, StockRivenMutation,
    StockRivenQuery, TransactionMutation, TransactionQuery,
};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct DebugClient {}

impl DebugClient {
    pub fn new() -> Self {
        DebugClient {}
    }

    pub async fn migrate_data_transactions(
        &self,
        old_con: &DatabaseConnection,
        new_con: &DatabaseConnection,
    ) -> Result<(), AppError> {
        let cache = states::cache()?;
        let notify = states::notify_client()?;
        // Migrate the database transactions
        let old_items = TransactionQuery::get_old_transactions(old_con)
            .await
            .map_err(|e| AppError::new_db("MigrateDataBase", e))?;

        for item in old_items {
            let mut entity = CreateStockEntity::new(&item.url, item.price as i64);

            entity.sub_type = if item.rank > 0 || item.item_type == "riven" {
                Some(SubType {
                    rank: Some(item.rank as i64),
                    variant: None,
                    cyan_stars: None,
                    amber_stars: None,
                })
            } else {
                None
            };

            if item.item_type == "riven" {
                entity.entity_type = StockType::Riven;
                match item.properties {
                    Some(properties) => {
                        entity.mod_name = properties["mod_name"].as_str().unwrap_or("").to_string();
                        if entity.mod_name == "" {
                            entity.mod_name = properties["name"].as_str().unwrap_or("").to_string();
                        }
                        if entity.mod_name == "" {
                            entity.mod_name = "Unknown".to_string();
                        }
                        entity.mastery_rank = properties["mastery_level"].as_i64().unwrap_or(0);
                        entity.re_rolls = properties["re_rolls"].as_i64().unwrap_or(0);
                        entity.polarity = properties["polarity"].as_str().unwrap_or("").to_string();
                        match properties["attributes"].as_array() {
                            Some(attributes) => {
                                let mut new_attributes = vec![];
                                for attribute in attributes {
                                    let attribute: entity::stock::riven::attribute::RivenAttribute =
                                        serde_json::from_value(attribute.clone()).unwrap();
                                    new_attributes.push(attribute);
                                }
                                entity.attributes = new_attributes;
                            }
                            None => {}
                        };
                    }
                    None => {}
                }
            } else if item.item_type == "item" {
                entity.entity_type = StockType::Item;
            }

            match entity.validate_entity(&cache, "--weapon_by url_name --weapon_lang en --item_by url_name --item_lang en --attribute_by url_name") {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {:?}", e);
                    continue;
                }
            }

            let transaction_type = match item.transaction_type.as_str() {
                "buy" => TransactionType::Purchase,
                "sell" => TransactionType::Sale,
                _ => {
                    return Err(AppError::new(
                        "MigrateDataBase",
                        eyre::eyre!("Invalid transaction type"),
                    ));
                }
            };

            let mut transaction = entity.to_transaction("", transaction_type)?;
            match item.created.parse() {
                Ok(date) => {
                    transaction.created_at = date;
                }
                Err(e) => {
                    let err = AppError::new(
                        "MigrateDataBase",
                        eyre::eyre!(format!("Failed to parse date and time: {}, Item: ", e)),
                    );
                    if !item.created.contains("+") {
                        let naive_datetime =
                            NaiveDateTime::parse_from_str(&item.created, "%Y-%m-%d %H:%M:%S%.f")
                                .unwrap();
                        transaction.created_at = naive_datetime.and_utc();
                    } else {
                        error::create_log_file("migrate_data_transactions.log", &err);
                        continue;
                    }
                }
            }
            transaction.updated_at = transaction.created_at.clone();
            match TransactionMutation::create_from_old(&new_con, transaction).await {
                Ok(_) => {}
                Err(e) => {
                    return Err(AppError::new_db("MigrateDataBase", e));
                }
            }
        }
        let new_items = TransactionQuery::get_all(new_con)
            .await
            .map_err(|e| AppError::new_db("MigrateDataBase", e))?;
        notify.gui().send_event_update(
            crate::utils::enums::ui_events::UIEvent::UpdateTransaction,
            crate::utils::enums::ui_events::UIOperationEvent::Set,
            Some(json!(new_items)),
        );
        Ok(())
    }

    pub async fn migrate_data_stock_item(
        &self,
        old_con: &DatabaseConnection,
        new_con: &DatabaseConnection,
    ) -> Result<(), AppError> {
        let cache = states::cache()?;
        let notify = states::notify_client()?;
        let old_items = StockItemQuery::get_old_stock_items(old_con)
            .await
            .map_err(|e| AppError::new_db("MigrateDataBase", e))?;
        for item in old_items {
            let mut entity = CreateStockEntity::new(&item.url, item.price as i64);
            entity.entity_type = StockType::Item;
            entity.sub_type = if item.rank > 0 {
                Some(SubType {
                    rank: Some(item.rank as i64),
                    variant: None,
                    cyan_stars: None,
                    amber_stars: None,
                })
            } else {
                None
            };

            match entity.validate_entity(&cache, "--item_by url_name --item_lang en") {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            }

            let stock_item = entity.to_stock_item().to_model();

            match StockItemMutation::create(&new_con, stock_item).await {
                Ok(_) => {}
                Err(e) => {
                    return Err(AppError::new_db("MigrateDataBase", e));
                }
            }
        }
        let new_items = StockItemQuery::get_all(new_con)
            .await
            .map_err(|e| AppError::new_db("MigrateDataBase", e))?;
        notify.gui().send_event_update(
            crate::utils::enums::ui_events::UIEvent::UpdateStockItems,
            crate::utils::enums::ui_events::UIOperationEvent::Set,
            Some(json!(new_items)),
        );
        Ok(())
    }

    pub async fn migrate_data_stock_riven(
        &self,
        old_con: &DatabaseConnection,
        new_con: &DatabaseConnection,
    ) -> Result<(), AppError> {
        let cache = states::cache()?;
        let notify = states::notify_client()?;
        let old_items = StockRivenQuery::get_old_stock_riven(old_con)
            .await
            .map_err(|e| AppError::new_db("MigrateDataBase", e))?;
        for item in old_items {
            let mut entity = CreateStockEntity::new(&item.weapon_url, item.price as i64);
            entity.entity_type = StockType::Riven;
            entity.mod_name = item.mod_name.clone();
            entity.mastery_rank = item.mastery_rank as i64;
            entity.re_rolls = item.re_rolls as i64;
            entity.polarity = item.polarity.clone();
            entity.attributes = item.attributes.clone().0;
            entity.sub_type = Some(SubType {
                rank: Some(item.rank as i64),
                variant: None,
                cyan_stars: None,
                amber_stars: None,
            });

            match entity.validate_entity(
                &cache,
                "--weapon_by url_name --weapon_lang en --attribute_by url_name",
            ) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {:?}", e);
                    continue;
                }
            }

            let stock_riven = entity.to_stock_riven().to_model();
            match StockRivenMutation::create(&new_con, stock_riven).await {
                Ok(_) => {}
                Err(e) => {
                    return Err(AppError::new_db("MigrateDataBase", e));
                }
            }
        }
        let new_items = StockRivenQuery::get_all(new_con)
            .await
            .map_err(|e| AppError::new_db("MigrateDataBase", e))?;
        notify.gui().send_event_update(
            crate::utils::enums::ui_events::UIEvent::UpdateStockRivens,
            crate::utils::enums::ui_events::UIOperationEvent::Set,
            Some(json!(new_items)),
        );
        Ok(())
    }
    pub async fn migrate_data_all(
        &self,
        old_con: &DatabaseConnection,
        new_con: &DatabaseConnection,
    ) -> Result<(), AppError> {
        self.migrate_data_transactions(old_con, new_con).await?;
        self.migrate_data_stock_item(old_con, new_con).await?;
        self.migrate_data_stock_riven(old_con, new_con).await?;
        Ok(())
    }

    pub async fn import_algo_trader(
        &self,
        old_con: &DatabaseConnection,
        new_con: &DatabaseConnection,
    ) -> Result<(), AppError> {
        let cache = states::cache()?;
        let notify = states::notify_client()?;
        let old_items = StockItemQuery::get_wat_stock_items(old_con)
            .await
            .map_err(|e| AppError::new_db("MigrateDataBase", e))?;
        for item in old_items {
            let mut entity = CreateStockEntity::new(&item.name, item.bought as i64);
            entity.entity_type = StockType::Item;

            match entity.validate_entity(&cache, "--item_by url_name --item_lang en") {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {:?}", e);
                    continue;
                }
            }
            let tradable_item = cache
                .tradable_items()
                .get_by(&item.name, "--item_by url_name --item_lang en")?;
            if entity.tags.contains(&"mod".to_string()) && tradable_item.is_some() {
                let tradable_item = tradable_item.unwrap();
                entity.sub_type = Some(SubType::new(tradable_item.max_rank, None, None, None));
            }
            let stock_riven = entity.to_stock_item().to_model();
            match StockItemMutation::create(&new_con, stock_riven).await {
                Ok(stock) => {
                    logger::info(
                        "MigrateDataBase",
                        &format!("Created: {}", stock.item_name),
                        LoggerOptions::default(),
                    );
                }
                Err(e) => {
                    return Err(AppError::new_db("MigrateDataBase", e));
                }
            }
        }
        match StockItemQuery::get_all(new_con).await {
            Ok(new_items) => {
                notify.gui().send_event_update(
                    crate::utils::enums::ui_events::UIEvent::UpdateStockItems,
                    crate::utils::enums::ui_events::UIOperationEvent::Set,
                    Some(json!(new_items)),
                );
            }
            Err(e) => {
                return Err(AppError::new_db("MigrateDataBase", e));
            }
        }

        let transactions = TransactionQuery::get_wat_transactions(old_con)
            .await
            .map_err(|e| AppError::new_db("MigrateDataBase", e))?;
        for item in transactions {
            let mut entity = CreateStockEntity::new(&item.name, item.price as i64);
            entity.entity_type = StockType::Item;

            match entity.validate_entity(&cache, "--item_by url_name --item_lang en") {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {:?}", e);
                    continue;
                }
            }
            let tradable_item = cache
                .tradable_items()
                .get_by(&item.name, "--item_by url_name --item_lang en")?;
            if entity.tags.contains(&"mod".to_string()) && tradable_item.is_some() {
                let tradable_item = tradable_item.unwrap();
                entity.sub_type = Some(SubType::new(tradable_item.max_rank, None, None, None));
            }

            let transaction_type = match item.transaction_type.as_str() {
                "buy" => TransactionType::Purchase,
                "sell" => TransactionType::Sale,
                _ => {
                    return Err(AppError::new(
                        "MigrateDataBase",
                        eyre::eyre!("Invalid transaction type"),
                    ));
                }
            };
            // Define the format of the input date string
            let format = "%Y-%m-%d %H:%M:%S%.f";
            let mut transaction = entity.to_transaction("", transaction_type)?;
            // Parse the string to a NaiveDateTime
            match NaiveDateTime::parse_from_str(&item.datetime, format) {
                Ok(naive_date_time) => {
                    // Convert NaiveDateTime to DateTime<Utc>
                    let date_time_utc: DateTime<Utc> =
                        DateTime::from_naive_utc_and_offset(naive_date_time, Utc);
                    transaction.created_at = date_time_utc;
                }
                Err(e) => {
                    println!("Failed to parse date and time: {}", e);
                }
            }
            match TransactionMutation::create_from_old(&new_con, transaction).await {
                Ok(stock) => {
                    logger::info(
                        "MigrateDataBase",
                        &format!("Created: {}", stock.item_name),
                        LoggerOptions::default(),
                    );
                }
                Err(e) => {
                    return Err(AppError::new_db("MigrateDataBase", e));
                }
            }
        }
        match TransactionQuery::get_all(new_con).await {
            Ok(new_items) => {
                notify.gui().send_event_update(
                    crate::utils::enums::ui_events::UIEvent::UpdateTransaction,
                    crate::utils::enums::ui_events::UIOperationEvent::Set,
                    Some(json!(new_items)),
                );
            }
            Err(e) => {
                return Err(AppError::new_db("MigrateDataBase", e));
            }
        }
        Ok(())
    }
}
