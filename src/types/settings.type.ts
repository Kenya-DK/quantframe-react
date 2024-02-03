import { StockItemDto, StockRivenDto, TransactionEntryDto, Wfm } from ".";
export interface SetupResponse {
  valid: boolean;
  price_scraper_last_run: number | null;
  user: Wfm.UserDto;
  settings: Settings;
  transactions: TransactionEntryDto[];
  orders: Wfm.OrderDto[];
  auctions: Wfm.Auction<string>[];
  items: Wfm.ItemDto[];
  riven_items: Wfm.RivenItemTypeDto[];
  stock_items: StockItemDto[];
  stock_rivens: StockRivenDto[];
  riven_attributes: Wfm.RivenAttributeInfoDto[];
  chats: Wfm.ChatData[];
  app_info: AppInfo
}
export interface AppInfo {
  app_author: string;
  app_description: string;
  app_name: string;
  app_version: AppVersion;
}

export interface AppVersion {
  current_version: string;
  download_url: string;
  release_notes: string;
  update_available: boolean;
  version: string;
}

export interface RustError {
  component: string;
  cause: string;
  backtrace: string;
  log_level: string;
  extra_data: any;
}

export interface ScraperState {
  is_running: boolean;
  last_run: Date | null;
  message: ScraperMessage | undefined;
  error: RustError | null;
}
export interface ScraperMessage {
  i18n_key: string;
  values: { [key: string]: string };
}

export interface Settings {
  debug: string[];
  dev_mode: boolean;
  live_scraper: LiveScraperSettings;
  notifications: Notifications;
}

export interface LiveScraperSettings {
  webhook: string;
  stock_mode: string;
  stock_item: StockItemSettings;
  stock_riven: StockRivenSettings;
}
export interface StockItemSettings {
  volume_threshold: number;
  max_total_price_cap: number;
  range_threshold: number;
  avg_price_cap: number;
  price_shift_threshold: number;
  strict_whitelist: boolean;
  report_to_wfm: boolean;
  auto_trade: boolean;
  order_mode: string;
  blacklist: string[];
  whitelist: string[];
}
export interface StockRivenSettings {
  range_threshold: number;
}

export interface NotificationBase {
  discord_notify: boolean;
  system_notify: boolean;
  title: string;
  content: string;
  webhook: string;
  user_ids: string[];
}
export interface Notifications {
  on_new_conversation: NotificationBase;
  on_wfm_chat_message: NotificationBase;
}

