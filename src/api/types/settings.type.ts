// Generated by https://quicktype.io

export interface Settings {
  debug: string[];
  dev_mode: boolean;
  live_scraper: SettingsLiveScraper;
  notifications: SettingsNotifications;
}

export interface SettingsLiveScraper {
  stock_item: SettingsStockItem;
  stock_mode: StockMode;
  stock_riven: SettingsStockRiven;
  webhook: string;
}

export interface SettingsStockItem {
  min_profit: number;
  auto_delete: boolean;
  auto_trade: boolean;
  avg_price_cap: number;
  blacklist: string[];
  max_total_price_cap: number;
  min_sma: number;
  order_mode: OrderMode;
  price_shift_threshold: number;
  range_threshold: number;
  report_to_wfm: boolean;
  strict_whitelist: boolean;
  volume_threshold: number;
  whitelist: string[];
}

export interface SettingsStockRiven {
  min_profit: number;
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
export enum OrderMode {
  Buy = "buy",
  Sell = "sell",
  Both = "both",
}