import { MinMaxDto, PriceHistory, RivenAttribute, UserStatus, WFMarketTypes } from ".";
import { DisplaySettings } from "../utils/helper";

export namespace TauriTypes {
  export enum StockMode {
    All = "all",
    Riven = "riven",
    Item = "item",
  }
  export enum PermissionsFlags {
    ALL = "all",
    EXPORT_DATA = "export_data",
    RIVEN_PRICES_SEARCH = "riven_prices_search",
    WFM_USER_ACTIVE_HISTORY = "wfm_user_active_history",
    FIND_INTERESTING_RIVENS = "find_interesting_rivens",
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
    // Live Scraper
    UpdateLiveScraperRunningState = "LiveScraper:UpdateRunningState",
    OnLiveScraperMessage = "LiveScraper:OnMessage",
    // Stock
    RefreshStockItems = "LiveScraper:RefreshStockItems",
    RefreshStockRiven = "LiveScraper:RefreshStockRiven",
    RefreshWishListItems = "LiveScraper:RefreshWishListItems",
    RefreshStockRivens = "LiveScraper:RefreshStockRivens",
    // Warframe Market
    RefreshWfmOrders = "LiveScraper:RefreshWfmOrders",
    OnDeleteWfmOrders = "Wfm:OnDeleteOrders",
    RefreshWfmAuctions = "LiveScraper:RefreshWfmAuctions",
    RefreshTransactions = "Transaction:RefreshTransactions",
    OnDeleteWfmAuctions = "Wfm:OnDeleteAuctions",
    OnNotify = "App:OnNotify",
    OnChatMessage = "Wfm:OnChatMessage",
    // Warframe GDPR
    RefreshWFGDPRAll = "WFGDPR:RefreshAll",
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
    is_pre_release: boolean;
    tos_uuid: string;
    use_temp_db: boolean;
    patreon_usernames: string[];
  }
  export interface Settings {
    lang: string;
    debug: string[];
    cross_play: boolean;
    dev_mode: boolean;
    live_scraper: SettingsLiveScraper;
    debugging: SettingsDebugging;
    advanced_settings: SettingsAdvanced;
    notifications: SettingsNotifications;
    http_server: HttpServerSettings;
    generate_trade_message: GenerateTradeMessageSettings;
    custom_sounds: CustomSound[];
  }
  export interface SettingsAdvanced {
    wf_log_path: string;
  }
  export interface SettingsDebugging {
    live_scraper: {
      entries: DebuggingLiveItemEntry[];
    };
  }
  export interface ManualUpdate {
    has_update: boolean;
    message: string;
    download: string;
    version: string;
  }
  export interface SettingsNotifications {
    on_new_conversation: NotificationSetting;
    on_wfm_chat_message: NotificationSetting;
    on_new_trade: NotificationSetting;
  }
  export interface HttpServerSettings {
    enable: boolean;
    host: string;
    port: number;
  }
  export interface NotificationSetting {
    discord_notify: DiscordNotify;
    system_notify: SystemNotify;
    webhook_notify: WebhookNotify;
  }
  export interface WebhookNotify {
    enabled: boolean;
    url: string;
  }
  export interface SystemNotify {
    enabled: boolean;
    title: string;
    content: string;
    sound_file: string;
    volume: number;
  }
  export interface DiscordNotify {
    enabled: boolean;
    content: string;
    webhook: string;
    user_ids: string[];
  }
  export interface DebuggingLiveItemEntry {
    stock_id?: number | null;
    wish_list_id?: number | null;
    wfm_url: string;
    sub_type?: SubType | null;
    priority: number;
    buy_quantity: number;
    sell_quantity: number;
    operation: string[];
    order_type: string;
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
  export interface GenerateTradeMessageSettings {
    templates: SaveTemplateSetting[];
  }

  export interface SaveTemplateSetting {
    name: string;
    prefix: string;
    suffix: string;
    template: string;
    displaySettings: Record<string, DisplaySettings>;
  }

  export interface CustomSound {
    name: string;
    file_name: string;
  }
  export interface User {
    anonymous: boolean;
    auctions_limit: number;
    wfm_avatar?: string;
    check_code: string;
    wfm_id: string;
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
    // General transaction metrics
    total_transactions: number;
    average_transaction: number;

    // Profit metrics
    total_profit: number;
    average_profit: number;
    profit_margin: number;
    roi: number; // Return on Investment percentage

    // Revenue metrics
    sale_count: number;
    highest_revenue: number;
    lowest_revenue: number;
    average_revenue: number;
    revenue: number;

    // Expense metrics
    purchases_count: number;
    highest_expense: number;
    lowest_expense: number;
    average_expense: number;
    expenses: number;

    // Properties
    properties?: Record<string, any>;
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
  export interface PaginatedDto {
    /** The total number of items in the database */
    total: number;
    /** The number of items returned in this request */
    limit: number;
    /** The current page */
    page: number;
    /** The total number of pages */
    total_pages: number;
  }
  export interface AppInfo {
    authors: string;
    description: string;
    name: string;
    version: string;
    is_pre_release: boolean;
    is_dev: boolean;
  }
  export interface CacheItemBase {
    name: string;
    unique_name: string;
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
    uniqueName: string;
    group: string;
    prefix: string;
    suffix: string;
    url_name: string;
    unit: string;
    full: string;
    short: string;
    name: string;
    text: string;
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
    unique_name: string;
    upgrade_type: string;
    wfm_group: string;
    wfm_icon: string;
    wfm_icon_format: string;
    wfm_id: string;
    wfm_thumb: string;
    wfm_url_name: string;
    is_variant: boolean;
  }
  export interface CacheChatIcon {
    name: string;
    code: string;
    url: string;
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
    wiki_url: string;
    image_url: string;
    sub_type?: CacheTradableItemSubType;
    components?: ItemComponent[];
  }
  export interface ItemComponent {
    itemCount: number;
    name: string;
    part_of_set: string;
    tradable: boolean;
    uniqueName: string;
    wfm_item_url: string;
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
  export interface ItemPriceInfo {
    wfm_url: string;
    wfm_id: string;
    uuid: string;
    volume: number;
    max_price: number;
    min_price: number;
    avg_price: number;
    moving_avg: number;
    median: number;
    profit: number;
    profit_margin: number;
    trading_tax: number;
    week_price_shift: number;
    sub_type: SubType;
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
    live_scraper: SettingsLiveScraper;
    notifications: SettingsNotifications;
    analytics: SettingsAnalytics;
    summary_settings: SettingsSummary;
    custom_sounds: CustomSound[];
  }
  export interface SettingsSummary {
    categories: SettingsCategorySummary[];
    recent_days: number;
    recent_transactions: number;
  }
  export interface SettingsCategorySummary {
    icon: string;
    name: string;
    tags: string[];
    types: string[];
  }
  export interface SettingsStockItem {
    blacklist: BlackListItemSetting[];
    buy_list: BuyListItemSetting[];
    min_profit: number;
    auto_delete: boolean;
    auto_trade: boolean;
    avg_price_cap: number;
    trading_tax_cap: number;
    buy_quantity: number;
    max_total_price_cap: number;
    min_sma: number;
    price_shift_threshold: number;
    profit_threshold: number;
    report_to_wfm: boolean;
    volume_threshold: number;
    min_wtb_profit_margin: number;
    quantity_per_trade: number;
    max_stock_quantity: number;
  }
  export interface BlackListItemSetting {
    wfm_id: string;
    disabled_for: TradeMode[];
  }
  export interface BuyListItemSetting {
    wfm_id: string;
    max_price: number;
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
  export interface ChartDto extends ChartWithValuesDto, ChartWithLabelsDto { }
  export interface ChartMultipleDto extends ChartWithMultipleValuesDto, ChartWithLabelsDto { }
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
    minimum_profit?: number;
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
    minimum_sma?: number;
    minimum_profit?: number;
    item_unique_name: string;
    owned: number;
    updated_at: string;
    wfm_id: string;
    wfm_url: string;
    info?: StockItemDetails;
  }
  export interface BaseItemDetails {
    item_info: CacheTradableItem;
    last_transactions: TransactionDto[];
    order_info: WFMarketTypes.Order | null;
    report: FinancialReport;
  }
  export interface StockItemDetails extends BaseItemDetails {
    stock: StockItem;
    potential_profit: number;
  }
  export interface CreateStockItem {
    raw: string;
    quantity: number;
    bought: number;
    minimum_price?: number;
    sub_type?: SubType;
  }
  export interface UpdateStockItem {
    id: number;
    owned?: number;
    bought?: number;
    minimum_price?: number;
    minimum_sma?: number;
    minimum_profit?: number;
    list_price?: number;
    status?: StockStatus;
    is_hidden?: boolean;
  }
  export interface SellStockItem {
    id?: number;
    wfm_url: string;
    sub_type?: SubType;
    quantity: number;
    price: number;
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
    mod_name: string;
    mastery_rank: number;
    re_rolls: number;
    polarity: string;
    attributes: RivenAttribute[];
    rank: number;
    bought: number;
  }
  export interface UpdateStockRiven {
    id: number;
    bought?: number;
    minimum_price?: number;
    minimum_profit?: number;
    re_rolls?: number;
    mastery_rank?: number;
    list_price?: number;
    status?: StockStatus;
    filter?: StockRivenFilter;
    is_hidden?: boolean;
  }
  export interface SellStockRiven {
    id?: number;
    wfm_url: string;
    mod_name: string;
    mastery_rank: number;
    re_rolls: number;
    polarity: string;
    rank: number;
    price: number;
    attributes: RivenAttribute[];
  }

  export interface RollEvaluation {
    tolerated_negative_attributes: ToleratedNegativeAttribute[];
    valid_rolls: ValidRoll[];
  }

  export interface ToleratedNegativeAttribute {
    label: string;
    matches: boolean;
  }

  export interface ValidRoll {
    optional: ToleratedNegativeAttribute[];
    required: ToleratedNegativeAttribute[];
  }

  export interface RivenSummaryFinancialReport extends FinancialReport {
    bought_price: number;
    potential_profit: number;
    last_transactions: TransactionDto[];
  }
  export interface RivenSummary {
    weapon_name: string;
    stock_status?: StockStatus;
    sub_name: string;
    endo: number;
    kuva: number;
    rank: number;
    mastery_rank: number;
    polarity: string;
    grade: string;
    image: string;
    rerolls: number;
    roll_evaluation?: RollEvaluation;
    stat_with_weapons: StatWithWeapon[];
    financial_summary: RivenSummaryFinancialReport;
    similarly_auctions: WFMarketTypes.Auction[];
    price_history: PriceHistory[];
  }

  export interface StatWithWeapon {
    by_level: { [key: string]: RivenAttribute[] };
    disposition: number;
    disposition_rank: number;
    name: string;
  }

  export interface StockRivenDetails extends RivenSummary { }
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
    profit?: number;
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
  export interface UpdateTransaction {
    id: number;
    price?: number;
    quantity?: number;
    user_name?: string;
    properties?: Record<string, any>;
    created_at?: string;
  }
  export interface WishListItem extends Omit<StockEntryBase, "minimum_price"> {
    id: number;
    item_name: string;
    wfm_url: string;
    quantity: number;
    maximum_price?: number;
    minimum_price?: number;
    is_hidden: boolean;
  }
  export interface WishListItemDetails extends BaseItemDetails {
    stock: WishListItem;
  }
  export interface CreateWishListItem extends Omit<CreateStockItem, "bought" | "minimum_price"> {
    maximum_price?: number;
  }
  export interface UpdateWishListItem {
    id: number;
    quantity?: number;
    minimum_price?: number;
    maximum_price?: number;
    list_price?: number;
    status?: StockStatus;
    is_hidden?: boolean;
  }
  export interface BoughtWishListItem {
    id: number;
    wfm_url: string;
    sub_type?: SubType;
    quantity: number;
    price: number;
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
  export interface ChatLink {
    prefix: string;
    link: string;
    suffix: string;
    properties?: Record<string, string | number>;
  }
  export interface TradeEntry {
    id: number;
    wfm_id: string;
    name: string;
    sub_type?: SubType;
    price: number;
    group: string;
    tags: string;
    updated_at: string;
    created_at: string;
    properties: Record<string, any>;
  }
  export interface TradeEntryDetails extends TradeEntry { }

  export interface CreateTradeEntry {
    raw: string;
    override_existing?: boolean;
    name?: string;
    sub_type?: SubType;
    price: number;
    group: string;
    tags?: string[];
    properties?: Record<string, any>;
  }
  export interface UpdateTradeEntry {
    id: number;
    price?: number;
    sub_type?: SubType;
    group?: string;
    tags?: string[];
    properties?: Record<string, any>;
  }
  export interface StockItemControllerGetListParams {
    page: number;
    limit: number;
    sort_by?: string;
    sort_direction?: "asc" | "desc";
    query?: string;
    status?: TauriTypes.StockStatus;
  }
  export type StockItemControllerGetListData = PaginatedDto & {
    results?: StockItemDto[];
  };
  export interface StockRivenControllerGetListParams {
    page: number;
    limit: number;
    sort_by?: string;
    sort_direction?: "asc" | "desc";
    query?: string;
    status?: TauriTypes.StockStatus;
  }
  export type StockRivenControllerGetListData = PaginatedDto & {
    results?: StockRiven[];
  };
  export interface WishListControllerGetListParams {
    page: number;
    limit: number;
    sort_by?: string;
    sort_direction?: "asc" | "desc";
    query?: string;
    status?: TauriTypes.StockStatus;
  }
  export type WishListControllerGetListData = PaginatedDto & {
    results?: WishListItem[];
  };
  export interface TransactionControllerGetListParams {
    page: number;
    limit: number;
    sort_by?: string;
    sort_direction?: "asc" | "desc";
    query?: string;
    transaction_type?: TauriTypes.TransactionType;
    item_type?: TauriTypes.TransactionItemType;
    tags?: string[];
    from_date?: string;
    to_date?: string;
  }
  export type TransactionControllerGetListData = PaginatedDto & {
    results?: TransactionDto[];
  };
  export interface TradeEntryControllerGetListParams {
    page: number;
    limit: number;
    sort_by?: string;
    sort_direction?: "asc" | "desc";
    query?: string;
    group?: string;
    tags?: string[];
  }
  export type TradeEntryControllerGetListData = PaginatedDto & {
    results?: TradeEntry[];
  };

  export interface PlayerTrade {
    credits: number;
    offeredItems: TradeItem[];
    platinum: number;
    playerName: string;
    receivedItems: TradeItem[];
    tradeTime: Date;
    type: string;
  }

  export interface TradeItem {
    item_type: string;
    quantity: number;
    raw: string;
    unique_name: string;
    sub_type?: SubType;
  }

  export type WFGDPRTradeControllerGetListData = PaginatedDto & {
    results?: PlayerTrade[];
  };
  export interface WFGDPRTradeControllerGetListParams {
    page: number;
    limit: number;
    sort_by?: string;
    sort_direction?: "asc" | "desc";
    query?: string;
    to_date?: string;
    from_date?: string;
    transaction_type?: TauriTypes.TransactionType;
    year: number;
  }
  export interface WFGDPRPurchase {
    date: string;
    items_received: [string, number][];
    price: number;
    shop_id: string;
  }
  export type WFGDPRPurchaseControllerGetListData = PaginatedDto & {
    results?: WFGDPRPurchase[];
  };
  export interface WFGDPRPurchaseControllerGetListParams {
    page: number;
    limit: number;
    sort_by?: string;
    sort_direction?: "asc" | "desc";
    query?: string;
  }
  export interface WFGDPRLogin {
    date: string;
    ip: string;
    client_type: string;
  }
  export type WFGDPRLoginControllerGetListData = PaginatedDto & {
    results?: WFGDPRLogin[];
  };
  export interface WFGDPRLoginControllerGetListParams {
    page: number;
    limit: number;
    from_date?: string;
    to_date?: string;
    sort_by?: string;
    sort_direction?: "asc" | "desc";
    query?: string;
  }

  export interface WFGDPRTransaction {
    account: string;
    currency: string;
    date: string;
    price: number;
    sku: string;
    vendor: string;
  }
  export type WFGDPRTransactionControllerGetListData = PaginatedDto & {
    results?: WFGDPRTransaction[];
  };
  export interface WFGDPRTransactionControllerGetListParams {
    page: number;
    limit: number;
    sort_by?: string;
    from_date?: string;
    to_date?: string;
    sort_direction?: "asc" | "desc";
    query?: string;
  }
}
