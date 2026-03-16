// export type DeepPartial<T> = T extends object
//   ? {
//       [P in keyof T]?: DeepPartial<T[P]>;
//     }
//   : T;
// export type ErrOrResult<RES> = [ResponseError, null] | [null, RES] | [ResponseError, undefined] | [undefined, RES];

import { QuantframeApiTypes } from "./quantframe.type";
import { TauriTypes } from "./tauri.type";
import { WFMarketTypes } from "./warframeMarket.type";

export interface ResponseError extends Error {
  component: string;
  message: string;
  location: string;
  cause?: string;
  context: Record<string, any>;
  log_level: string;
}
export interface SubType {
  rank?: number;
  variant?: string;
  amber_stars?: number;
  cyan_stars?: number;
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
export enum UserStatus {
  Online = "online",
  Invisible = "invisible",
  Ingame = "ingame",
}
export interface MinMaxDto {
  min?: number;
  max?: number;
}
export interface Paginated<T> {
  total: number;
  limit: number;
  page: number;
  results: T[];
}

export interface PriceHistory {
  created_at: Date;
  name: string;
  price: number;
  user_id: string;
}
export interface RivenAttribute<T = any> {
  url_name: string;
  positive: boolean;
  value: number;
  localized_text: string;
  properties?: T;
}

export interface ItemMeta {
  wfm_id?: string;
  wfm_url?: string;
  quantity?: number;
  type?: string;
}

export type ItemWithMeta =
  | (WFMarketTypes.Order<{ name?: string }> & ItemMeta)
  | (TauriTypes.StockItem & ItemMeta)
  | (TauriTypes.StockRiven & ItemMeta)
  | (TauriTypes.WishListItem & ItemMeta)
  | (TauriTypes.TransactionDto & ItemMeta)
  | (TauriTypes.ItemPriceInfo & ItemMeta)
  | (QuantframeApiTypes.ItemPriceDto & ItemMeta)
  | (TauriTypes.DebuggingLiveItemEntry & ItemMeta)
  | (TauriTypes.TradeEntry & ItemMeta)
  | null;
export type ItemWithSubType = TauriTypes.SubType | WFMarketTypes.Order<{ name?: string }> | undefined;

export interface ItemRiven<P = any, A = any> {
  attributes: RivenAttribute<A>[];
  mastery_rank: number;
  mod_name: string;
  name: string;
  polarity: string;
  re_rolls: number;
  riven_type: string;
  sub_type?: SubType;
  unique_name: string;
  uuid: string;
  wfm_url: string;
  quantity: number;
  properties: P;
}
