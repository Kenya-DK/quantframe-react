import { Wfm } from './wfm.type';

export interface StockEntryDto {
  id?: number;
  wfm_id: string;
  url: string;
  tags: string;
  name: string;
  rank: number;
  price: number;
  listed_price?: number | null;
  owned: number;
}

export interface StockItemDto extends StockEntryDto {
  sub_type?: string;
  owned: number;
}

export interface StockRivenDto extends StockEntryDto {
  attributes: Wfm.RivenAttributeDto[];
  mastery_rank: number;
  re_rolls: number;
  polarity: string;
}

export interface CreateStockEntryDto {
  item_id: string;
  rank: number;
  price: number;
}

export interface CreateStockItemEntryDto extends CreateStockEntryDto {
  report: boolean;
  quantity: number;
  sub_type?: string;
}

export interface CreateStockRivenEntryDto extends CreateStockEntryDto {
  attributes: Wfm.RivenAttributeDto[];
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

