use entity::{
    enums::StockType, stock_item::CreateStockItem, stock_riven::CreateStockRiven,
    wish_list::CreateWishListItem,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CreateStockEntity {
    #[serde(rename = "entity_type")]
    pub entity_type: StockType,

    #[serde(flatten)]
    pub item: CreateStockItem,

    #[serde(flatten)]
    pub riven: CreateStockRiven,

    #[serde(flatten)]
    pub wish_list: CreateWishListItem,
}

impl Default for CreateStockEntity {
    fn default() -> Self {
        CreateStockEntity {
            entity_type: StockType::Unknown,
            item: CreateStockItem::new("", None, 0),
            riven: CreateStockRiven::new("raw", "mod_name", 0, 0, "", vec![], 0),
            wish_list: CreateWishListItem::new("raw", None, 0),
        }
    }
}
