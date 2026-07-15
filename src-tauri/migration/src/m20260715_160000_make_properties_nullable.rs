use sea_orm::{ConnectionTrait, Statement};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = db.get_database_backend();

        for table in ["stock_riven", "wish_list"] {
            recreate_properties_column(db, backend, table, "JSON DEFAULT '{}'", "properties")
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = db.get_database_backend();

        for table in ["stock_riven", "wish_list"] {
            recreate_properties_column(
                db,
                backend,
                table,
                "JSON NOT NULL DEFAULT '{}'",
                "COALESCE(properties, '{}')",
            )
            .await?;
        }

        Ok(())
    }
}

async fn recreate_properties_column(
    db: &dyn ConnectionTrait,
    backend: sea_orm::DatabaseBackend,
    table: &str,
    column_definition: &str,
    value_expression: &str,
) -> Result<(), DbErr> {
    for sql in [
        format!("ALTER TABLE {table} ADD COLUMN properties_new {column_definition}"),
        format!("UPDATE {table} SET properties_new = {value_expression}"),
        format!("ALTER TABLE {table} DROP COLUMN properties"),
        format!("ALTER TABLE {table} RENAME COLUMN properties_new TO properties"),
    ] {
        db.execute(Statement::from_string(backend, sql)).await?;
    }

    Ok(())
}
