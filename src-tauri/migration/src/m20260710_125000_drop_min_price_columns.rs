use sea_orm::{ConnectionTrait, Statement};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = db.get_database_backend();

        // Migrate existing column values into the properties JSON
        // For stock_item table
        db.execute(Statement::from_string(
            backend.clone(),
            "UPDATE stock_item SET properties = json_set(COALESCE(properties, '{}'), '$.min_price', minimum_price) WHERE minimum_price IS NOT NULL".to_string(),
        ))
        .await?;

        db.execute(Statement::from_string(
            backend.clone(),
            "UPDATE stock_item SET properties = json_set(COALESCE(properties, '{}'), '$.min_profit', minimum_profit) WHERE minimum_profit IS NOT NULL".to_string(),
        ))
        .await?;

        db.execute(Statement::from_string(
            backend.clone(),
            "UPDATE stock_item SET properties = json_set(COALESCE(properties, '{}'), '$.min_sma', minimum_sma) WHERE minimum_sma IS NOT NULL".to_string(),
        ))
        .await?;
        // For wish_list table
        db.execute(Statement::from_string(
            backend.clone(),
            "UPDATE wish_list SET properties = json_set(COALESCE(properties, '{}'), '$.max_price', maximum_price) WHERE maximum_price IS NOT NULL".to_string(),
        ))
        .await?;
        db.execute(Statement::from_string(
            backend.clone(),
            "UPDATE wish_list SET properties = json_set(COALESCE(properties, '{}'), '$.min_price', minimum_price) WHERE minimum_price IS NOT NULL".to_string(),
        ))
        .await?;

        // For stock riven table
        db.execute(Statement::from_string(
            backend.clone(),
            "UPDATE stock_riven SET properties = json_set(COALESCE(properties, '{}'), '$.min_price', minimum_price) WHERE minimum_price IS NOT NULL".to_string(),
        ))
        .await?;

        // Drop columns from stock_item table
        for col in &["minimum_price", "minimum_profit", "minimum_sma"] {
            db.execute(Statement::from_string(
                backend.clone(),
                format!("ALTER TABLE stock_item DROP COLUMN {col}"),
            ))
            .await?;
        }
        // Drop columns from stock_riven table
        for col in &["minimum_price"] {
            db.execute(Statement::from_string(
                backend.clone(),
                format!("ALTER TABLE stock_riven DROP COLUMN {col}"),
            ))
            .await?;
        }

        // Drop columns from wish_list table
        for col in &["minimum_price", "maximum_price"] {
            db.execute(Statement::from_string(
                backend.clone(),
                format!("ALTER TABLE wish_list DROP COLUMN {col}"),
            ))
            .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = db.get_database_backend();

        for (col, col_type) in &[
            ("minimum_price", "INTEGER"),
            ("minimum_profit", "INTEGER"),
            ("minimum_sma", "INTEGER"),
        ] {
            db.execute(Statement::from_string(
                backend.clone(),
                format!("ALTER TABLE stock_item ADD COLUMN {col} {col_type}"),
            ))
            .await?;
        }

        Ok(())
    }
}
