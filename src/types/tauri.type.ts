import { ItemRiven, MinMaxDto, PriceHistory, RivenAttribute, UserStatus } from ".";

export namespace TauriTypes {
  //--------------------------------------------------------------------------------
  //  Enums
  //--------------------------------------------------------------------------------
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
    SYNDICATE_PRICES_SEARCH = "syndicate_prices_search",
    FIND_INTERESTING_RIVENS = "find_interesting_rivens",
  }
  export enum TradeMode {
    Buy = "buy",
    Sell = "sell",
    Wishlist = "wishlist",
  }
  export enum Events {
    OnError = "App:Error",
    OnStartingUp = "App:StartingUp",
    UpdateUser = "User:Update",
    RefreshSettings = "Settings:Refresh",
    UpdateLiveScraperRunningState = "LiveScraper:UpdateRunningState",
    OnLiveScraperMessage = "LiveScraper:OnMessage",
    RefreshCache = "Cache:Refresh",
    RefreshStockItems = "LiveScraper:RefreshStockItems",
    RefreshStockRiven = "LiveScraper:RefreshStockRiven",
    RefreshWishListItems = "LiveScraper:RefreshWishListItems",
    RefreshStockRivens = "LiveScraper:RefreshStockRivens",
    RefreshWfmOrders = "LiveScraper:RefreshWfmOrders",
    OnDeleteWfmOrders = "Wfm:OnDeleteOrders",
    RefreshWfmAuctions = "LiveScraper:RefreshWfmAuctions",
    RefreshTransactions = "Transaction:RefreshTransactions",
    OnDeleteWfmAuctions = "Wfm:OnDeleteAuctions",
    OnNotify = "App:OnNotify",
    OnChatMessage = "Wfm:OnChatMessage",
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

  //--------------------------------------------------------------------------------
  //  Common
  //--------------------------------------------------------------------------------
  export interface SubType {
    rank?: number;
    variant?: string;
    amber_stars?: number;
    cyan_stars?: number;
  }

  //--------------------------------------------------------------------------------
  //  Settings
  //--------------------------------------------------------------------------------
  export interface Settings {
    lang: string;
    live_scraper: LiveScraperSettings;
    summary_settings: SummarySettings;
    log_settings: LogSettings;
    advanced_settings: AdvancedSettings;
    debugging: DebuggingSettings;
    notifications: NotificationsSetting;
    wf_inventory: WFInventorySettings;
    generate_trade_message: GenerateTradeMessageSetting;
    tos_uuid: string;
  }
  export interface LiveScraperSettings {
    general: LiveScraperGeneralSettings;
    items: ItemSettings;
    rivens: RivenSettings;
  }
  export interface LiveScraperGeneralSettings {
    report_to_wfm: boolean;
    auto_delete: boolean;
    auto_trade: boolean;
    stock_mode: StockMode;
    trade_modes: TradeMode[];
    delete_conflicting_orders: boolean;
  }
  export interface WFInventorySettings {
    inv_path: string;
  }
  export interface LogSettings {
    ee_log_path: string;
  }
  export interface ItemSettings {
    general: ItemGeneralSettings;
    wtb: ItemWtbSettings;
    wts: ItemWtsSettings;
  }
  export interface ItemGeneralSettings {
    blacklist: BlackListItemSetting[];
    buy_list: BuyListItemSetting[];
  }
  export interface BlackListItemSetting {
    wfmId: string;
    disabled_for: TradeMode[];
  }
  export interface BuyListItemSetting {
    wfmId: string;
    max_price: number;
  }
  export interface ItemWtbSettings {
    min_sma: number;
    min_profit: number;
  }
  export interface ItemWtsSettings {
    volume_threshold: number;
    profit_threshold: number;
    avg_price_cap: number;
    trading_tax_cap: number;
    max_total_price_cap: number;
    price_shift_threshold: number;
    buy_quantity: number;
    min_wtb_profit_margin: number;
    quantity_per_trade: number;
    max_stock_quantity: number;
  }
  export interface RivenSettings {
    general: RivenGeneralSettings;
    wts: RivenWtsSettings;
  }
  export interface RivenGeneralSettings {
    update_interval: number;
  }
  export interface RivenWtsSettings {
    min_profit: number;
    threshold_percentage: number;
    max_results: number;
  }
  export interface SummarySettings {
    recent_days: number;
    recent_transactions: number;
    categories: SummaryCategorySetting[];
  }
  export interface SummaryCategorySetting {
    icon: string;
    name: string;
    types: string[];
    tags: string[];
  }
  export interface AdvancedSettings {
    http_server: HttpServerSettings;
  }
  export interface DebuggingSettings {
    live_scraper: DebuggingLiveScraperSettings;
  }
  export interface DebuggingLiveScraperSettings {
    entries: DebuggingLiveItemEntry[];
    fake_orders: boolean;
  }
  export interface DebuggingLiveItemEntry {
    stock_id?: number | null;
    wish_list_id?: number | null;
    wfm_url: string;
    sub_type?: SubType | null;
    priority: number;
    buy_quantity: number;
    sell_quantity: number;
    operations: string[];
    order_type: string;
  }
  export interface NotificationsSetting {
    custom_sounds: CustomSound[];
    on_new_conversation: NotificationSetting;
    on_wfm_chat_message: NotificationSetting;
    on_new_trade: NotificationSetting;
  }
  export interface NotificationSetting {
    system_notify: SystemNotify;
    discord_notify: DiscordNotify;
    webhook_notify: WebHookNotify;
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
  export interface WebHookNotify {
    enabled: boolean;
    url: string;
  }
  export interface HttpServerSettings {
    enable: boolean;
    host: string;
    port: number;
  }
  export interface GenerateTradeMessageSetting {
    templates: SaveTemplateSetting[];
  }
  export interface SaveTemplateSetting {
    name: string;
    prefix?: string;
    suffix?: string;
    template: string;
    group_by_key?: string;
    displaySettings: Record<string, DisplaySettings>;
  }
  export interface DisplaySettings {
    prefix?: string;
    suffix?: string;
  }
  export interface CustomSound {
    name: string;
    name_key: string;
    file_name: string;
  }

  //--------------------------------------------------------------------------------
  //  User
  //--------------------------------------------------------------------------------
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

  //--------------------------------------------------------------------------------
  //  App Info
  //--------------------------------------------------------------------------------
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
  export interface InitializeResponds {
    app_info: AppInfo;
    settings: Settings;
  }

  //--------------------------------------------------------------------------------
  //  Financial / Dashboard
  //--------------------------------------------------------------------------------
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
    total_transactions: number;
    average_transaction: number;
    total_profit: number;
    average_profit: number;
    profit_margin: number;
    roi: number;
    sale_count: number;
    highest_revenue: number;
    lowest_revenue: number;
    average_revenue: number;
    revenue: number;
    purchases_count: number;
    highest_expense: number;
    lowest_expense: number;
    average_expense: number;
    expenses: number;
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
    total: number;
    limit: number;
    page: number;
    total_pages: number;
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

  //--------------------------------------------------------------------------------
  //  Stock / Wishlist
  //--------------------------------------------------------------------------------
  export interface StockEntryBase<T = any> {
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
    properties: T;
  }
  export interface StockItem<T = any> extends StockEntryBase<T> {
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
  export interface StockRiven<T = any> extends StockEntryBase<T> {
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
  export interface StockEntryOverview {
    id: string;
    key: string;
    count: number;
    revenue: number;
    expenses: number;
    profit: number;
  }
  export interface WishListItem<T = any> extends Omit<StockEntryBase<T>, "minimum_price"> {
    id: number;
    item_name: string;
    wfm_url: string;
    quantity: number;
    maximum_price?: number;
    minimum_price?: number;
    is_hidden: boolean;
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

  //--------------------------------------------------------------------------------
  //  Cache
  //--------------------------------------------------------------------------------
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
    exclusiveTo?: unknown[];
    formattedValue: string;
    full: string;
    group: string;
    unit?: string;
    highlightedLabel: string;
    label: string;
    negativeOnly?: boolean;
    positiveIsNegative?: boolean;
    positiveOnly?: boolean;
    prefix: string;
    short: string;
    suffix: string;
    uniqueName: string;
    url_name: string;
    wfmId: string;
    wfmUrl: string;
  }
  export interface CacheSyndicate {
    uniqueName: string;
    name: string;
    canSelect: boolean;
    iconColour: string;
    backgroundColour: string;
    titles: CacheSyndicateTitle[];
  }
  export interface CacheSyndicateTitle {
    level: number;
    name: string;
    minStanding: number;
    maxStanding: number;
  }
  export interface CacheRivenWeapon {
    uniqueName: string;
    name: string;
    icon: string;
    wfmUrl: string;
    wfmRivenId: string;
    wfmRivenUrl: string;
    rivenType: string;
    wfmId: string;
    category: string;
    source: string;
    disposition: number;
    dispositionRank: number;
    isVariant: boolean;
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
    wfmId: string;
    wfmUrl: string;
    tradeTax: number;
    masteryReq: number;
    tags: string[];
    icon: string;
    subTypes?: CacheTradableItemSubType;
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
    maxRank?: number;
    variants?: string[];
    amberStars?: number;
    cyanStars?: number;
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

  //--------------------------------------------------------------------------------
  //  Transaction
  //--------------------------------------------------------------------------------
  export interface TransactionDto {
    created_at: string;
    id: number;
    item_name: string;
    item_type: TransactionItemType;
    item_unique_name: string;
    price: number;
    profit?: number;
    credits: number;
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

  //--------------------------------------------------------------------------------
  //  Trade Entry
  //--------------------------------------------------------------------------------
  export interface HandleItem {
    wfm_url: string;
    quantity: number;
    sub_type?: SubType;
    price: number;
    user_name: string;
    order_type: string;
    flags: string[];
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
  export interface TradeEntryDetails extends TradeEntry {}
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
  export interface PlayerTrade {
    credits: number;
    offeredItems: TradeItem[];
    platinum: number;
    playerName: string;
    receivedItems: TradeItem[];
    tradeTime: Date;
    type: string;
  }
  export interface TradeItem<T = any> {
    item_type: string;
    quantity: number;
    raw: string;
    unique_name: string;
    sub_type?: SubType;
    properties?: T;
  }

  //--------------------------------------------------------------------------------
  //  GDPR
  //--------------------------------------------------------------------------------
  export interface WFGDPRAccount {
    account_creation_date: Date;
    activated: boolean;
    country_code: string;
    display_name: string;
    email: string;
    ips: string[];
    language: string;
    last_login_date: Date;
    oid: string;
    signup_country_code: string;
    signup_language: string;
    signup_page: string;
    subscribed_to_emails: boolean;
    logins: WFGDPRLogin[];
    purchases: WFGDPRPurchase[];
    trades: PlayerTrade[];
    transactions: WFGDPRTransaction[];
  }
  export interface WFGDPRTransaction {
    account: string;
    currency: string;
    date: string;
    price: number;
    sku: string;
    vendor: string;
  }
  export interface WFGDPRLogin {
    date: string;
    ip: string;
    client_type: string;
  }
  export interface WFGDPRPurchase {
    date: string;
    items_received: [string, number][];
    price: number;
    shop_id: string;
  }

  //--------------------------------------------------------------------------------
  //  Warframe Inventory
  //--------------------------------------------------------------------------------
  export interface WFInvItemBase<T = any> {
    id: string;
    name: string;
    unique_name: string;
    wfm_url: string;
    quantity: number;
    sub_type?: SubType;
    properties?: Record<string, T>;
  }
  export interface WFItemControllerGetListParams<T = any> {
    page: number;
    limit: number;
    sort_by?: string;
    sort_direction?: "asc" | "desc";
    query?: string;
    properties?: T;
    item_types?: string[];
  }
  export type WFInvRivenControllerGetListData = PaginatedDto & {
    results?: ItemRiven<{
      is_in_stock: boolean;
      challenge_description: string;
      challenge_description_with_complication: string;
      grade: string;
      required?: number;
      progress?: number;
      price?: number;
    }>[];
  };

  //--------------------------------------------------------------------------------
  //  EE Log
  //--------------------------------------------------------------------------------
  export interface EELog {
    index: number;
    date: number;
    ignore_combined: boolean;
    line: string;
    prev_line: string;
  }
  export type EELogControllerGetListData = PaginatedDto & {
    results?: EELog[];
  };
  export interface EELogControllerGetListParams {
    page: number;
    limit: number;
    sort_by?: string;
    sort_direction?: "asc" | "desc";
    query?: string;
    hide_empty?: boolean;
  }

  //--------------------------------------------------------------------------------
  //  Controller Params
  //--------------------------------------------------------------------------------
  export interface StockItemControllerGetListParams {
    page: number;
    limit: number;
    sort_by?: string;
    sort_direction?: "asc" | "desc";
    query?: string;
    status?: StockStatus;
  }
  export type StockItemControllerGetListData = PaginatedDto & {
    results?: StockItem[];
  };
  export interface StockRivenControllerGetListParams {
    page: number;
    limit: number;
    sort_by?: string;
    sort_direction?: "asc" | "desc";
    query?: string;
    status?: StockStatus;
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
    status?: StockStatus;
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
    transaction_type?: TransactionType;
    item_type?: TransactionItemType;
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
}
