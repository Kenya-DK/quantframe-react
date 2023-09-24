import { Wfm } from './wfm.type';

export interface InventoryEntryDto {
  id?: number;
  item_id: string;
  item_url: string;
  item_name: string;
  rank: number;
  price: number;
  listed_price?: number | null;
  owned: number;
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
}
export interface TransactionEntryDto {
  id?: number;
  item_id: string;
  item_type: string;
  item_url: string;
  item_name: string;
  item_tags: string[];
  rank: number;
  price: number;
  quantity: number;
  datetime: string;
  transaction_type: string;
}

