import { CacheItemBase } from "."

export interface CacheTradableItem extends CacheItemBase {
  description: string;
  wfm_id: string;
  wfm_url_name: string;
  trade_tax: number;
  mr_requirement: number;
  tags: string[];
  components?: Record<string, number>;
  wiki_url: string;
  image_url: string;
  sub_type?: CacheTradableItemSubType
}

export interface CacheTradableItemSubType {
  max_rank?: number;
  variants?: string[];
  amber_stars?: number;
  cyan_stars?: number;
}
