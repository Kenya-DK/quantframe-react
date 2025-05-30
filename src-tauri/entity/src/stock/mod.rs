pub mod item {
    pub mod create;
    pub mod dto {
        pub mod pagination_stock_item;
        pub use pagination_stock_item::*;
    }

    pub mod stock_item;
}

pub mod riven {
    pub mod attribute;
    pub mod dto {
        pub mod pagination_stock_riven;
        pub use pagination_stock_riven::*;
    }
    pub mod create;
    pub mod match_riven;
    pub mod stock_riven;
}
