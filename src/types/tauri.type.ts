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
    OnError = "App:Error",
    OnStartingUp = "App:StartingUp",

    // User
    UpdateUser = "User:Update",

    // Settings
    RefreshSettings = "Settings:Refresh",
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
    is_dev: boolean;
    tos_uuid: string;
  }

  export interface Settings {
    debug: string[];
    cross_play: boolean;
    dev_mode: boolean;
    wf_log_path: string;
    live_scraper: SettingsLiveScraper;
  }
  export interface SettingsLiveScraper {
    stock_mode: StockMode;
    trade_modes: TradeMode[];
    report_to_wfm: boolean;
    auto_delete: boolean;
    auto_trade: boolean;
    should_delete_other_types: boolean;
    stock_item: SettingsStockItem;
    stock_riven: SettingsStockRiven;
  }
  export interface User {
    anonymous: boolean;
    auctions_limit: number;
    wfm_avatar?: string;
    check_code: string;
    id: string;
    wfm_username: string;
    locale: string;
    order_limit: number;
    platform: string;
    qf_access_token: string;
    qf_banned: boolean;
    qf_banned_reason?: string;
    qf_banned_until?: string;
    region: string;
    wfm_status: UserStatus;
    unread_messages: number;
    verification: boolean;
    wfm_access_token: string;
    wfm_banned: boolean;
    wfm_banned_reason?: string;
    wfm_banned_until?: string;
    patreon_tier?: string;
    permissions?: string;
  }

  export interface DashboardSummary {
    best_seller: FinancialItemReport;
    recent_days: FinancialWithGraph;
    today: FinancialWithGraph;
    total: DashboardTotal;
    resent_transactions: TransactionDto[];
    categories: FinancialCategoryReport[];
  }
  export interface DashboardTotal extends FinancialReport {
    last_year: FinancialWithGraph;
    present_year: FinancialWithGraph;
  }

  export interface FinancialReport {
    average_expense: number;
    average_profit: number;
    average_revenue: number;
    average_transaction: number;
    expenses: number;
    profit_margin: number;
    purchases: number;
    revenue: number;
    roi: number;
    sale_count: number;
    total_profit: number;
    total_transactions: number;
    total_value: number;
  }
  export interface FinancialItemProperties {
    item_name: string;
    item_type: string;
    wfm_id: string;
  }
  export interface FinancialCategoryProperties {
    icon: string;
    name: string;
  }
  export interface FinancialItemReport extends FinancialReport {
    properties: FinancialItemProperties;
  }
  export interface FinancialCategoryReport extends FinancialReport {
    properties: FinancialCategoryProperties;
  }
  export interface FinancialWithGraph {
    summary: FinancialItemReport;
    chart: Chart;
  }

  export interface Chart {
    labels: string[];
    values: number[];
  }
  // Old code for TauriClient
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
  export interface CacheTheme {
    name: string;
    author: string;
    fileName: string;
    iconBase64: string;
    properties: Record<string, any>;
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
    summary_settings: SettingsSummary;
  }
  export interface SettingsSummary {
    categories: SettingsCategorySummary[];
    resent_days: number;
    resent_transactions: number;
  }

  export interface SettingsCategorySummary {
    icon: string;
    name: string;
    tags: string[];
    types: string[];
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
    profit_threshold: number;
    report_to_wfm: boolean;
    volume_threshold: number;
    min_wtb_profit_margin: number;
  }

  export interface SettingsStockRiven {
    min_profit: number;
    threshold_percentage: number;
    limit_to: number;
    update_interval: number;
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

  export interface StockEntryOverview {
    id: string;
    key: string;
    count: number;
    revenue: number;
    expenses: number;
    profit: number;
  }

  export interface ChartWithLabelsDto {
    labels: Array<string>;
  }

  export interface ChartDto extends ChartWithValuesDto, ChartWithLabelsDto {}

  export interface ChartMultipleDto extends ChartWithMultipleValuesDto, ChartWithLabelsDto {}

  export interface TradingSummaryDto {
    best_selling_items: TransactionItemSummaryDto[];
    category_summary: TransactionCategorySummaryDto[];
    recent_days: TransactionSummaryWithChartDto;
    resent_transactions: TransactionDto[];
    today: TransactionSummaryWithChartDto;
    total: TransactionSummaryWithYearDto;
  }

  export interface TransactionSummaryDto {
    average_expense: number;
    average_profit: number;
    average_revenue: number;
    expenses: number;
    profit: number;
    profit_margin: number;
    purchases: number;
    revenue: number;
    sales: number;
    total_transactions: number;
  }
  export interface TransactionSummaryWithChartDto extends TransactionSummaryDto {
    chart: TransactionSummaryChart;
  }
  export interface TransactionSummaryWithYearDto extends TransactionSummaryDto {
    last_year: TransactionSummaryWithChartDto;
    present_year: TransactionSummaryWithChartDto;
  }
  export interface TransactionSummaryChart {
    labels: string[];
    values: number[];
  }

  export interface TransactionItemSummaryDto {
    average_price: number;
    expenses: number;
    item_name: string;
    item_type: string;
    profit: number;
    profit_margin: number;
    purchases: number;
    quantity: number;
    revenue: number;
    sales: number;
    sub_type?: SubType;
    tags: string;
    total_transactions: number;
    wfm_id: string;
  }

  export interface TransactionCategorySummaryDto {
    expenses: number;
    icon: string;
    name: string;
    profit: number;
    profit_margin: number;
    revenue: number;
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
    wfm_url: string;
    mod_name: string;
    mastery_rank: number;
    re_rolls: number;
    polarity: string;
    attributes: RivenAttribute[];
    bought: number;
    rank: number;
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

  export interface WishListItem extends Omit<StockEntryBase, "minimum_price"> {
    id: number;
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

  export interface StockRivenControllerGetListParams {
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

  /** PaginatedResponseOfAlertDto */
  export type StockRivenControllerGetListData = PaginatedDto & {
    results?: StockRiven[];
  };
  export interface WishListControllerGetListParams {
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

  /** PaginatedResponseOfAlertDto */
  export type WishListControllerGetListData = PaginatedDto & {
    results?: WishListItem[];
  };
  export interface TransactionControllerGetListParams {
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
    item_type?: TauriTypes.TransactionItemType;
    /** Filter by transaction type */
    transaction_type?: TauriTypes.TransactionType;
  }

  /** PaginatedResponseOfAlertDto */
  export type TransactionControllerGetListData = PaginatedDto & {
    results?: TransactionDto[];
  };
}
