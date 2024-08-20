
import { StockEntryBase, SubType } from ".";
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
  info?: StockItemDetails;
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
  url: string;
  sub_type?: SubType;
  quantity: number;
  price: number;
  is_from_order: boolean;
}

export interface StockItemDetails {
  highest_price: number;
  lowest_price: number;
  moving_avg: number;
  orders: Wfm.OrderDto[];
  profit: number;
  total_sellers: number;
}