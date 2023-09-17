import { InventoryEntryDto, TransactionEntryDto, Wfm } from ".";
export interface SetupResponse {
  valid: boolean;
  price_scraper_last_run: number | null;
  user: Wfm.UserDto;
  settings: Settings;
  transactions: TransactionEntryDto[];
  inventorys: InventoryEntryDto[];
  orders: Wfm.OrderDto[];
  items: Wfm.ItemDto[];
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
  error: RustError | null;
}

export interface Settings {
  live_scraper: LiveScraperSettings;
  whisper_scraper: WhisperScraperSettings;
}

export interface LiveScraperSettings {
  volume_threshold: number;
  max_total_price_cap: number;
  range_threshold: number;
  avg_price_cap: number;
  price_shift_threshold: number;
  strict_whitelist: boolean;
  webhook: string;
  blacklist: string[];
  whitelist: string[];
}

export interface WhisperScraperSettings {
  ping_on_notif: boolean;
  webhook: string;
}

