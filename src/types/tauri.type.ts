import { MinMaxDto, PriceHistory, RivenAttribute, UserStatus, WFMarketTypes } from ".";

export namespace TauriTypes {
  export enum StockMode {
    All = "all",
    Riven = "riven",
    Item = "item",
  }
  export enum TradeMode {
    Buy = "buy",
    Sell = "sell",
    Wishlist = "wishlist",
  }
  export enum Events {
    // App
    All = "*",
    OnInitialize = "App:OnInitialize",
    UpdateSettings = "App:UpdateSettings",
    UpdateAppInfo = "App:UpdateAppInfo",
    UpdateAppError = "App:UpdateAppError",

    // Warframe Market
    UpdateOrders = "WFM:UpdateOrders",
    UpdateTransaction = "WFM:UpdateTransaction",
    UpdateAuction = "WFM:UpdateAuction",

    // Chat
    UpdateChats = "WFM:UpdateChats",
    ChatReceiveMessage = "Chat:ReceiveMessage",
    ChatMessageSent = "Chat:MessageSent",

    // Stock
    RefreshStockItems = "Stock:RefreshStockItems",
    UpdateStockRivens = "Stock:UpdateStockRivens",

    // Wish List
    UpdateWishList = "WishList:Update",

    // User
    UpdateUser = "User:Update",

    // Live Trading
    UpdateLiveTradingRunningState = "LiveTrading:UpdateRunningState",
    OnLiveTradingError = "LiveTrading:OnError",
    OnLiveTradingMessage = "LiveTrading:OnMessage",

    // Notification
    OnNotificationError = "Notification:OnError",
    OnNotificationWarning = "Notification:OnWarning",
    OnNotificationSuccess = "Notification:OnSuccess",

    // Control
    OnToggleControl = "Control:OnToggleControl",

    // Alert
    UpdateAlert = "Alert:Update",
  }

  export enum EventOperations {
    CREATE_OR_UPDATE = "CREATE_OR_UPDATE",
    DELETE = "DELETE",
    SET = "SET",
  }

  export enum StockStatus {
    Pending = "pending",
    Live = "live",
    ToLowProfit = "to_low_profit",
    NoSellers = "no_sellers",
    NoBuyers = "no_buyers",
    InActive = "inactive",
    SMALimit = "sma_limit",
    OrderLimit = "order_limit",
    Overpriced = "overpriced",
    Underpriced = "underpriced",
  }
  export enum TransactionType {
    Purchase = "purchase",
    Sale = "sale",
    Trade = "trade",
  }

  export enum TransactionItemType {
    Item = "item",
    Riven = "riven",
  }
  export interface AppInfo {
    authors: string;
    description: string;
    name: string;
    version: string;
    is_pre_release: boolean;
    is_development: boolean;
  }
  export interface CacheItemBase {
    name: string;
    uniqueName: string;
  }
  export interface CacheRivenAttribute {
    id: string;
    gameRef: string;
    group: string;
    prefix: string;
    suffix: string;
    effect: string;
    url_name: string;
    unit?: string;
    exclusiveTo?: string[];
    positiveIsNegative?: boolean;
    positiveOnly?: boolean;
    negativeOnly?: boolean;
  }

  export interface CacheRivenWeapon {
    disposition: number;
    godRoll: RivenGodRoll;
    name: string;
    riven_type: string;
    uniqueName: string;
    upgrade_type: string;
    wfm_group: string;
    wfm_icon: string;
    wfm_icon_format: string;
    wfm_id: string;
    wfm_thumb: string;
    wfm_url_name: string;
  }

  export interface RivenGodRoll {
    good_rolls: RivenGoodRoll[];
    negative_attributes: string[];
    weapon_url_name: string;
  }

  export interface RivenGoodRoll {
    optional: string[];
    required: string[];
  }

  export interface CacheTradableItem extends CacheItemBase {
    description: string;
    wfm_id: string;
    wfm_url_name: string;
    trade_tax: number;
    mr_requirement: number;
    tags: string[];
    components?: Record<string, number>;
    wiki_url: string;
    image_url: string;
    sub_type?: CacheTradableItemSubType;
  }

  export interface CacheTradableItemSubType {
    max_rank?: number;
    variants?: string[];
    amber_stars?: number;
    cyan_stars?: number;
  }

  export interface OnToggleControlPayload {
    id: string;
    state: boolean;
  }
  export interface InitializeResponds {
    app_info: AppInfo;
    settings: Settings;
    stock_items: StockItem[];
    stock_rivens: StockRiven[];
    transactions: TransactionDto[];
    user: User;
    valid: boolean;
    orders?: WFMarketTypes.OrderDto[];
    auctions?: WFMarketTypes.Auction<string>[];
    chats?: WFMarketTypes.ChatData[];
  }

  export interface Settings {
    debug: string[];
    tos_uuid: string;
    cross_play: boolean;
    dev_mode: boolean;
    wf_log_path: string;
    live_scraper: SettingsLiveScraper;
    notifications: SettingsNotifications;
    analytics: SettingsAnalytics;
  }

  export interface SettingsLiveScraper {
    stock_item: SettingsStockItem;
    stock_mode: StockMode;
    trade_modes: TradeMode[];
    should_delete_other_types: boolean;
    stock_riven: SettingsStockRiven;
    webhook: string;
  }

  export interface SettingsStockItem {
    min_profit: number;
    auto_delete: boolean;
    auto_trade: boolean;
    avg_price_cap: number;
    trading_tax_cap: number;
    buy_quantity: number;
    blacklist: string[];
    max_total_price_cap: number;
    min_sma: number;
    price_shift_threshold: number;
    range_threshold: number;
    report_to_wfm: boolean;
    volume_threshold: number;
  }

  export interface SettingsStockRiven {
    min_profit: number;
    threshold_percentage: number;
    limit_to: number;
  }
  export interface SettingsAnalytics {
    transaction: boolean;
    stock_item: boolean;
    stock_riven: boolean;
  }
  export interface SettingsNotifications {
    on_new_conversation: SettingsNotification;
    on_wfm_chat_message: SettingsNotification;
    on_new_trade: SettingsNotification;
  }

  export interface SettingsNotification {
    content: string;
    discord_notify: boolean;
    system_notify: boolean;
    title: string;
    user_ids: any[];
    webhook: string;
  }

  export interface ChartWithValuesDto {
    values: Array<number>;
  }

  export interface ChartWithMultipleValuesDto {
    profit_values: Array<number>;
  }

  export interface ChartWithLabelsDto {
    labels: Array<string>;
  }

  export interface ChartDto extends ChartWithValuesDto, ChartWithLabelsDto {}

  export interface ChartMultipleDto extends ChartWithMultipleValuesDto, ChartWithLabelsDto {}

  export interface StatisticTransactionPreviousPresent extends ChartWithLabelsDto {
    previous: StatisticProfitTransaction;
    present: StatisticProfitTransaction;
  }
  export interface StatisticProfitBase {
    profit: number;
    profit_margin: number;
    average_revenue: number;
    purchases: number;
    sales: number;
    expense: number;
    revenue: number;
  }

  export interface StatisticProfitTransaction extends StatisticProfitBase {
    number_of_trades: number;
    popular_items: StatisticProfitItem[];
  }

  export interface StatisticChartTransactionAndItem extends StatisticProfitTransaction {
    chart_profit: ChartMultipleDto;
    chart_items: ChartMultipleDto;
  }

  export interface StatisticProfitTransactionPreviousPresent extends ChartWithLabelsDto {
    previous: StatisticProfitTransaction & ChartWithMultipleValuesDto;
    present: StatisticProfitTransaction & ChartWithMultipleValuesDto;
  }

  export interface StatisticProfitItem extends StatisticProfitBase {
    wfm_id: string;
    url: string;
    item_type: string;
    name: string;
    tags: string[];
    quantity: number;
  }
  export interface StatisticItemCategoryProfit extends StatisticProfitBase {
    name: string;
    icon: string;
    quantity: number;
  }

  // Statistic Type
  export interface StatisticItemBestSeller {
    items: StatisticProfitItem[];
    items_chart: ChartMultipleDto;
    category: StatisticItemCategoryProfit[];
    category_chart: ChartMultipleDto;
  }
  export interface StatisticProfitTransactionTotal extends StatisticProfitTransactionPreviousPresent, StatisticProfitBase {}

  export interface StatisticProfitTransactionToday extends StatisticChartTransactionAndItem {}

  // export interface StatisticItemCategoryProfit extends StatisticProfitBase {
  export interface StatisticRecentTransactions extends StatisticProfitBase {
    transactions: TransactionDto[];
  }
  export interface StatisticProfitTransactionRecentDays extends StatisticChartTransactionAndItem {
    days: number;
  }
  export interface CategoryItemProfitLink {
    name: string;
    icon: string;
    tags: string[];
    types: string[];
  }
  export interface StatisticDto {
    best_seller: StatisticItemBestSeller;
    total: StatisticProfitTransactionTotal;
    today: StatisticProfitTransactionToday;
    recent_days: StatisticProfitTransactionRecentDays;
    recent_transactions: StatisticRecentTransactions;
  }
  export interface StockEntryBase {
    id: number;
    bought: number;
    minimum_price?: number;
    list_price?: number;
    sub_type?: SubType;
    status: StockStatus;
    created_at: string;
    updated_at: string;
    price_history: PriceHistory[];
  }

  export interface StockItem extends StockEntryBase {
    created_at: string;
    id: number;
    is_hidden: boolean;
    item_name: string;
    item_unique_name: string;
    owned: number;
    updated_at: string;
    wfm_id: string;
    wfm_url: string;
    info?: StockItemDetails;
  }

  export interface CreateStockItem {
    wfm_url: string;
    bought: number;
    quantity: number;
    minimum_price?: number;
    sub_type?: SubType;
  }

  export interface UpdateStockItem {
    id?: number;
    bought?: number;
    quantity?: number;
    minimum_price?: number;
    is_hidden?: boolean;
    sub_type?: SubType;
  }

  export interface SellStockItem {
    id: number;
    wfm_url: string;
    sub_type?: SubType;
    quantity: number;
    price: number;
  }

  export interface StockItemDetails {
    highest_price: number;
    lowest_price: number;
    moving_avg: number;
    orders: WFMarketTypes.OrderDto[];
    profit: number;
    total_sellers: number;
  }
  export interface StockRiven extends StockEntryBase {
    attributes: RivenAttribute[];
    comment: string;
    filter: StockRivenFilter;
    id: number;
    is_hidden: boolean;
    mastery_rank: number;
    mod_name: string;
    polarity: string;
    price_history: any[];
    re_rolls: number;
    updated_at: string;
    weapon_name: string;
    weapon_type: string;
    weapon_unique_name: string;
    wfm_order_id: string;
    wfm_weapon_id: string;
    wfm_weapon_url: string;
    info?: StockRivenDetails;
  }

  export interface StockRivenFilter {
    attributes?: StockRivenFilterAttribute[];
    enabled: boolean;
    mastery_rank?: MinMaxDto;
    polarity?: string;
    rank?: MinMaxDto;
    re_rolls?: MinMaxDto;
    required_negative?: boolean;
    similarity?: null | number;
  }

  export interface StockRivenFilterAttribute {
    positive: boolean;
    is_required: boolean;
    url_name: string;
  }

  export interface CreateStockRiven {
    raw: string;
    wfm_url: string;
    bought: number;
    mod_name: string;
    mastery_rank: number;
    re_rolls: number;
    polarity: string;
    rank: number;
    attributes: RivenAttribute[];
    minimum_price?: number;
    is_hidden?: boolean;
  }

  export interface UpdateStockRiven {
    id?: number;
    bought?: number;
    quantity?: number;
    minimum_price?: number;
    is_hidden?: boolean;
    filter?: StockRivenFilter;
  }

  export interface SellStockRiven {
    id: number;
    quantity: number;
    price: number;
  }
  export interface StockRivenDetails {
    auctions: WFMarketTypes.Auction<WFMarketTypes.AuctionOwner>[];
    changes: string;
    highest_price: number;
    is_dirty: boolean;
    lowest_price: number;
    profit: number;
    total_buyers: number;
    total_sellers: number;
  }
  export interface SubType {
    rank?: number;
    variant?: string;
    amber_stars?: number;
    cyan_stars?: number;
  }
  export interface TransactionDto {
    created_at: string;
    id: number;
    item_name: string;
    item_type: TransactionItemType;
    item_unique_name: string;
    price: number;
    properties: Record<string, any>;
    quantity: number;
    sub_type: SubType;
    tags: string;
    transaction_type: string;
    updated_at: string;
    user_name: string;
    wfm_id: string;
    wfm_url: string;
  }

  export interface UpdateTransactionDto {
    id: number;
    price: number;
    quantity: number;
  }
  export interface User {
    anonymous: boolean;
    auctions_limit: number;
    avatar?: string;
    check_code: string;
    id: string;
    ingame_name: string;
    locale: string;
    order_limit: number;
    platform: string;
    qf_access_token: string;
    qf_banned: boolean;
    qf_banned_reason?: string;
    qf_banned_until?: string;
    region: string;
    status: UserStatus;
    unread_messages: number;
    verification: boolean;
    wfm_access_token: string;
    wfm_banned: boolean;
    wfm_banned_reason?: string;
    wfm_banned_until?: string;
    patreon_tier?: string;
    permissions?: string;
  }

  export interface WishListItem extends Omit<StockEntryBase, "minimum_price"> {
    item_name: string;
    wfm_url: string;
    quantity: number;
    maximum_price?: number;
    is_hidden: boolean;
    info?: WishListItemDetails;
  }
  export interface CreateWishListItem extends Omit<CreateStockItem, "bought" | "minimum_price"> {
    maximum_price?: number;
  }

  export interface UpdateWishListItem {
    id: number;
    maximum_price?: number;
    is_hidden?: boolean;
    sub_type?: SubType;
  }

  export interface BoughtWishListItem {
    id: number;
    price: number;
  }

  export interface WishListItemDetails {
    highest_price: number;
    lowest_price: number;
    moving_avg: number;
    orders: WFMarketTypes.OrderDto[];
    profit: number;
    total_sellers: number;
  }

  export interface StockEntryBaseDto {
    id: number;
    bought: number;
    minimum_price?: number;
    list_price?: number;
    sub_type?: SubType;
    status: StockStatus;
    created_at: string;
    updated_at: string;
    price_history: PriceHistory[];
  }

  export interface StockItemDto extends StockEntryBaseDto {
    created_at: string;
    id: number;
    is_hidden: boolean;
    item_name: string;
    item_unique_name: string;
    owned: number;
    updated_at: string;
    wfm_id: string;
    wfm_url: string;
    info?: StockItemDetails;
  }

  export interface StockItemControllerGetListParams {
    /**
     * For pagination. Defines which page the results are fetched from.
     * @min 1
     * @default 1
     */
    page: number;
    /**
     * For pagination. Defines how many entries are returned per page.
     * @min 0
     * @max 100
     * @default 25
     */
    limit: number;
    sort_by?: string;
    /** Sort direction used when sorting by a specific field. */
    sort_direction?: "asc" | "desc";
    /** A search query to filter the users by name or email. */
    query?: string;
    /** Filter by stock status */
    status?: TauriTypes.StockStatus;
  }

  export interface PaginatedDto {
    /** The total number of items in the database */
    total: number;
    /** The number of items returned in this request */
    limit: number;
    /** The current page */
    page: number;
  }

  /** PaginatedResponseOfAlertDto */
  export type StockItemControllerGetListData = PaginatedDto & {
    results?: StockItemDto[];
  };
}
