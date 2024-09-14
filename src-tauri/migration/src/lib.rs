pub use sea_orm_migration::prelude::*;

mod m20240406_104026_create_transaction_table;
mod m20240406_135257_create_stock_item_table;
mod m20240406_135307_create_stock_riven_table;
mod m20240406_135323_create_trade_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240406_104026_create_transaction_table::Migration),
            Box::new(m20240406_135257_create_stock_item_table::Migration),
            Box::new(m20240406_135307_create_stock_riven_table::Migration),
        ]
    }
}
