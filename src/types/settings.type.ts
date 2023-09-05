import { InventoryEntryDto, TransactionEntryDto, Wfm } from ".";
export interface SetupResponse {
  valid: boolean;
  user: Wfm.UserDto;
  settings: Settings;
  transactions: TransactionEntryDto[];
  inventorys: InventoryEntryDto[];
  orders: Wfm.OrderDto[];
}

export interface Settings {
  volume_threshold: number;
  max_total_price_cap: number;
  range_threshold: number;
  avg_price_cap: number;
  price_shift_threshold: number;
  strict_whitelist: boolean;
  ping_on_notif: boolean;
  webhook: string;
  blacklist: string[];
  whitelist: string[];
}

export interface CacheBase {
  createdAt: number,
}

export interface TradableItemsCache extends CacheBase {
  items: Wfm.ItemDto[],
}
export interface Cache {
  tradableItems: TradableItemsCache,
}
export interface PriceHistoryDto {
  name: string;
  datetime: string;
  order_type: string;
  volume: number;
  min_price: number;
  max_price: number;
  range?: number;
  median: number;
  avg_price: number;
  mod_rank?: number;
  item_id: string;

  id?: string;
  open_price?: number;
  closed_price?: number;
  wa_price?: number;
  moving_avg?: number;
  donch_top?: number;
  donch_bot?: number;

}
export interface PriceHistoryCache extends CacheBase {
  items: PriceHistoryDto[],
}