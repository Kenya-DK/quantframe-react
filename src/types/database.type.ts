import { Wfm } from './wfm.type';

export interface MinMaxDto {
  min: number;
  max: number;
}
export interface StockEntryDto {
  id?: number;
  order_id?: string | null;
  wfm_id: string;
  weapon_id: string;
  weapon_name: string;
  weapon_url: string;
  weapon_type: string;
  url: string;
  tags: string;
  name: string;
  rank: number;
  price: number;
  status: string;
  minium_price?: number | null,
  listed_price?: number | null;
  owned: number;
}

export interface StockItemDto extends StockEntryDto {
  sub_type?: string;
  hidden: boolean;
  owned: number;
}

export interface StockRivenDto extends StockEntryDto {
  attributes: Wfm.RivenAttributeDto[];
  mastery_rank: number;
  mod_name: string;
  re_rolls: number;
  private: boolean;
  match_riven: MatchRivenDto;
}
export interface MatchRivenDto {
  rank?: MinMaxDto;
  mastery_rank?: MinMaxDto;
  re_rolls?: MinMaxDto;
  polarity?: string;
}

export interface CreateStockEntryDto {
  item_id: string;
  rank: number;
  price: number;
  minium_price?: number,
}

export interface CreateStockItemEntryDto extends CreateStockEntryDto {
  quantity: number;
  sub_type?: string;
}

export interface CreateStockRivenEntryDto extends CreateStockEntryDto {
  mod_name: string;
  attributes: Wfm.RivenAttributeDto[];
  match_riven: MatchRivenDto;
  mastery_rank: number;
  re_rolls: number;
  polarity: string;
}

export interface CreateTransactionEntryDto {
  item_id: string;
  item_type: string;
  transaction_type?: string;
  report?: boolean;
  rank: number;
  price: number;
  quantity: number;
  sub_type?: string;
  attributes?: Wfm.RivenAttributeDto[];
  mastery_rank?: number;
  re_rolls?: number;
  polarity?: string;
}
export interface TransactionEntryDto {
  id?: number;
  wfm_id: string,
  url: string,
  name: string,
  item_type: string,
  tags: string,
  transaction_type: string,
  quantity: number,
  rank: number,
  price: number,
  created: string,
  properties?: any
}

export interface TransactionItemEntryDto extends TransactionEntryDto {
  properties?: Omit<StockItemDto, 'owned'>
}

export interface TransactionRivenDto extends TransactionEntryDto {
  properties: StockRivenDto
}

