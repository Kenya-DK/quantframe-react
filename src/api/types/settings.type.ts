// Generated by https://quicktype.io

export interface Settings {
  debug: string[];
  tos_accepted: boolean;
  dev_mode: boolean;
  live_scraper: SettingsLiveScraper;
  notifications: SettingsNotifications;
  analytics: SettingsAnalytics;
}

export interface SettingsLiveScraper {
  stock_item: SettingsStockItem;
  stock_mode: StockMode;
  trade_mode: TradeMode;
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

export enum StockMode {
  All = "all",
  Riven = "riven",
  Item = "item",
}
export enum TradeMode {
  All = "all",
  Buy = "buy",
  Sell = "sell",
  Wishlist = "wishlist",
}
