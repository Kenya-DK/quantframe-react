// Generated by https://quicktype.io

import { MinMaxDto, RivenAttribute, StockEntryBase } from ".";

export interface StockRiven extends StockEntryBase {
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
  attributes: StockRivenFilterAttribute[];
  enabled: boolean;
  mastery_rank: MinMaxDto;
  polarity: string;
  rank: MinMaxDto;
  re_rolls: MinMaxDto;
  required_negative: boolean;
  similarity: null | number;
}

export interface StockRivenFilterAttribute {
  is_negative: boolean;
  is_required: boolean;
  url_name: string;
}