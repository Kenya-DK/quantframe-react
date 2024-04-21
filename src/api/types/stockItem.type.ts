
import { StockEntryBase, SubType } from ".";

export interface StockItem extends StockEntryBase {
  created_at: string;
  id: number;
  is_hidden: boolean;
  item_name: string;
  item_unique_name: string;
  owned: number;
  updated_at: string;
  wfm_id: string;
  wfm_url: string;
}

export interface CreateStockItem {
  wfm_url: string;
  bought: number;
  quantity: number;
  minimum_price?: number;
  sub_type?: SubType;
}

export interface UpdateStockItem {
  id: number;
  bought?: number;
  quantity?: number;
  minimum_price?: number;
  is_hidden?: boolean;
  sub_type?: SubType;
}

export interface SellStockItem {
  id: number;
  quantity: number;
  price: number;
}