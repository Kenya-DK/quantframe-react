import { StockEntryBase, CreateStockItem, SubType } from ".";
import { Wfm } from "$types/index";

export interface WishListItem extends Omit<StockEntryBase, "minimum_price"> {
  item_name: string;
  wfm_url: string;
  quantity: number;
  maximum_price?: number;
  is_hidden: boolean;
  info?: WishListItemDetails;
}
export interface CreateWishListItem extends Omit<CreateStockItem, "bought" | "minimum_price"> {
  maximum_price?: number;
}

export interface UpdateWishListItem {
  id: number;
  maximum_price?: number;
  is_hidden?: boolean;
  sub_type?: SubType;
}

export interface BoughtWishListItem {
  id: number;
  price: number;
}

export interface WishListItemDetails {
  highest_price: number;
  lowest_price: number;
  moving_avg: number;
  orders: Wfm.OrderDto[];
  profit: number;
  total_sellers: number;
}
