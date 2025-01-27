pub use sea_orm_migration::prelude::*;

mod m20240406_104026_create_transaction_table;
mod m20240406_135257_create_stock_item_table;
mod m20240406_135307_create_stock_riven_table;
mod m20240406_135323_create_wish_list_table;
mod m20250127_210013_add_hide_wishlist;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240406_104026_create_transaction_table::Migration),
            Box::new(m20240406_135257_create_stock_item_table::Migration),
            Box::new(m20240406_135307_create_stock_riven_table::Migration),
            Box::new(m20240406_135323_create_wish_list_table::Migration),
            Box::new(m20250127_210013_add_hide_wishlist::Migration),
        ]
    }
}
