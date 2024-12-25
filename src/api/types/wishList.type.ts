import { StockEntryBase, CreateStockItem, SubType } from ".";

export interface WishListItem extends Omit<StockEntryBase, "minimum_price"> {
  item_name: string;
  quantity: number;
  maximum_price?: number;
}
export interface CreateWishListItem extends Omit<CreateStockItem, "bought" | "minimum_price"> {
  maximum_price?: number;
}

export interface UpdateWishListItem {
  id: number;
  maximum_price?: number;
  sub_type?: SubType;
}
