import { SubType } from ".";

export interface TransactionDto {
  created_at: string;
  id: number;
  item_name: string;
  item_type: TransactionItemType;
  item_unique_name: string;
  price: number;
  properties: Record<string, any>;
  quantity: number;
  sub_type: SubType;
  tags: string;
  transaction_type: string;
  updated_at: string;
  user_name: string;
  wfm_id: string;
  wfm_url: string;
}

export enum TransactionType {
  Purchase = "purchase",
  Sale = "sale",
  Trade = "trade",
}

export enum TransactionItemType {
  Item = "item",
  Riven = "riven",
}