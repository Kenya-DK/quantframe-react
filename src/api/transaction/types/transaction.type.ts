export enum TransactionType {
  Buy = "buy",
  Sell = "sell"
}

export interface TransactionDto {
  created: string;
  id: number;
  item_type: string;
  name: string;
  price: number;
  properties: null;
  quantity: number;
  rank: number;
  tags: string;
  transaction_type: string;
  url: string;
  wfm_id: string;
}