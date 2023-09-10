import { InventoryEntryDto, TransactionEntryDto, Wfm } from ".";
export interface SetupResponse {
  valid: boolean;
  price_scraper_status: string;
  user: Wfm.UserDto;
  settings: Settings;
  transactions: TransactionEntryDto[];
  inventorys: InventoryEntryDto[];
  orders: Wfm.OrderDto[];
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

