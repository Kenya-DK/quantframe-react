pub mod auction_ext;
pub mod auction_list_ext;
pub mod create_stock_item_ext;
pub mod create_stock_riven_ext;
pub mod create_wish_list_item_ext;
pub mod enums;
pub mod error_ext;
pub mod modules;
pub mod order_ext;
pub mod order_list_ext;
pub mod sub_type_ext;
pub mod wfm_auction_pagination_query_dto;
pub mod wfm_chat_pagination_query_dto;
pub mod wfm_order_pagination_query_dto;

// Re-export the error extension trait for convenience
pub use auction_ext::AuctionExt;
pub use auction_list_ext::AuctionListExt;
pub use create_stock_item_ext::CreateStockItemExt;
pub use create_stock_riven_ext::CreateStockRivenExt;
pub use create_wish_list_item_ext::CreateWishListItemExt;
pub use error_ext::ErrorFromExt;
pub use order_ext::OrderExt;
pub use order_list_ext::OrderListExt;
pub use sub_type_ext::SubTypeExt;
pub use wfm_auction_pagination_query_dto::*;
pub use wfm_chat_pagination_query_dto::*;
pub use wfm_order_pagination_query_dto::*;
