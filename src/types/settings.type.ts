import { Wfm } from ".";
export interface Settings {
  mastery_rank: 2, // Trading is unlocked at MR2
  user_email: '',
  user_password: '',
  access_token: string | undefined,
  volume_threshold: number;
  range_threshold: number;
  avg_price_cap: number;
  price_shift_threshold: number;
  blacklist: string[];
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