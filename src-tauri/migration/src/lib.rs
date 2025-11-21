pub use sea_orm_migration::prelude::*;

mod m20240406_104026_create_transaction_table;
mod m20240406_135257_create_stock_item_table;
mod m20240406_135307_create_stock_riven_table;
mod m20240406_135323_create_wish_list_table;
mod m20250127_210013_add_hide_wishlist;
mod m20250823_135631_add_uuid_to_stock_riven;
mod m20251016_203616_stock_add_min_profit_sma;
mod m20251108_000558_add_profit_to_transactions;
mod m20251118_125228_create_trade_entry;

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
            Box::new(m20250823_135631_add_uuid_to_stock_riven::Migration),
            Box::new(m20251016_203616_stock_add_min_profit_sma::Migration),
            Box::new(m20251108_000558_add_profit_to_transactions::Migration),
            Box::new(m20251118_125228_create_trade_entry::Migration),
        ]
    }
}
