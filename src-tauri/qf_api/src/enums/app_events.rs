use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ApplicationEvent {
    // --------------------------------------------------
    // Appearance
    // --------------------------------------------------
    ThemeCreate,
    SwitchTheme,
    ThemeOpenFolder,

    // --------------------------------------------------
    // Authentication
    // --------------------------------------------------
    AuthLogin,
    AuthLogout,

    // --------------------------------------------------
    // General
    // --------------------------------------------------
    PageView,
    AppStart,
    AppExit,

    // --------------------------------------------------
    // Stock Item
    // --------------------------------------------------
    StockItemCreate,
    StockItemSell,
    StockItemDelete,
    StockItemUpdate,
    StockItemExport,

    // --------------------------------------------------
    // Stock Riven
    // --------------------------------------------------
    StockRivenCreate,
    StockRivenSell,
    StockRivenDelete,
    StockRivenUpdate,
    StockRivenExport,

    // --------------------------------------------------
    // Syndicate Item
    // --------------------------------------------------
    SyndicateItemImport,
    SyndicateItemCreate,
    SyndicateItemSell,
    SyndicateItemDelete,
    SyndicateItemUpdate,
    SyndicateItemExport,

    // --------------------------------------------------
    // Syndicate Price
    // --------------------------------------------------
    SyndicateItemPricesLookup,
    SyndicateItemPricesExport,

    // --------------------------------------------------
    // Trade Entry
    // --------------------------------------------------
    TradeEntryCreate,
    TradeEntryDelete,
    TradeEntryUpdate,
    TradeEntryExport,

    // --------------------------------------------------
    // Trade Detection
    // --------------------------------------------------
    TradeAccepted,
    TradeAcceptedFailed,
    TradeCancelled,
    TradeFailed,
    TradeUnknown,

    // --------------------------------------------------
    // Warframe GDPR
    // --------------------------------------------------
    WarframeGdprLoad,

    // --------------------------------------------------
    // Wish List
    // --------------------------------------------------
    WishListCreate,
    WishListBought,
    WishListDelete,
    WishListUpdate,
    WishListExport,

    // --------------------------------------------------
    // Transaction
    // --------------------------------------------------
    TransactionDelete,
    TransactionUpdate,
    TransactionExport,
    TransactionCalculateTax,

    // --------------------------------------------------
    // Warframe Market Auction's
    // --------------------------------------------------
    WFMAuctionRefresh,
    WFMAuctionImport,
    WFMAuctionDelete,

    // --------------------------------------------------
    // Warframe Market Chat's
    // --------------------------------------------------
    ChatRefresh,
    ChatDelete,
    ChatGetMessages,
    ChatSendMessage,
    ChatConversationDetected,

    // --------------------------------------------------
    // Debug
    // --------------------------------------------------
    DebugExportEeLogs,
    DebugGetWfmState,
    DebugTest,

    // --------------------------------------------------
    // Item Prices
    // --------------------------------------------------
    ItemPriceLookup,
    ItemPriceExport,
    RivenPriceLookup,
    RivenPriceExport,

    // --------------------------------------------------
    // Orders
    // --------------------------------------------------
    OrderRefresh,
    OrderDeleteAll,
    OrderDeleteById,

    // --------------------------------------------------
    // Sound
    // --------------------------------------------------
    SoundAddCustomSound,
    SoundDeleteCustomSound,

    // --------------------------------------------------
    // WF Inventory
    // --------------------------------------------------
    WFInventoryUpdate,

    // --------------------------------------------------
    // Logs
    // --------------------------------------------------
    LogExport,

    // --------------------------------------------------
    // Handlers
    // --------------------------------------------------
    HandledItems,

    // --------------------------------------------------
    // Live Scraper
    // --------------------------------------------------
    LiveScraperStop,
    LiveScraperError,
}
