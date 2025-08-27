use crate::m20240406_135307_create_stock_riven_table::StockRiven;

use entity::stock_riven::*;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{ActiveModelTrait, EntityTrait, Set, Statement},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(StockRiven::Table)
                    .add_column(
                        ColumnDef::new(StockRiven::UUID)
                            .uuid()
                            .not_null()
                            .default("00000000-0000-0000-0000-000000000000"),
                    )
                    .to_owned(),
            )
            .await
            .unwrap();

        // Fetch all items
        let db = manager.get_connection();
        let items = Entity::find().all(db).await?;
        // Update each row with deterministic v5 UUID
        for item in items {
            let uuid = item.uuid(); // Compute the UUID using the method from the entity
            let mut active: ActiveModel = item.into();
            active.uuid = Set(uuid.to_string());

            active.update(db).await?;
        }
        // Remove WFMOrderId column
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "ALTER TABLE stock_riven DROP COLUMN wfm_order_id;".to_owned(),
            ))
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(StockRiven::Table)
                    .drop_column(StockRiven::UUID)
                    .to_owned(),
            )
            .await
    }
}
