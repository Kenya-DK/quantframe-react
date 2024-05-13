
import { StockEntryBase, SubType, UserStatus } from ".";
import { Wfm } from "$types/index";

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
  extra?: StockItemExtra;
}

export interface StockItemExtra {
  profit: number;
  sma_price: number;
  trades: Wfm.OrderDto[];
}

export interface User {
  id: string;
  ingame_name: string;
  reputation: number;
  avatar?: string;
  status: UserStatus;
}


export interface CreateStockItem {
  wfm_url: string;
  bought: number;
  quantity: number;
  minimum_price?: number;
  sub_type?: SubType;
}

export interface UpdateStockItem {
  id?: number;
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