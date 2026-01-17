import { AppInfo, Settings, StockItem, StockRiven, TransactionDto } from ".";
import { User } from "./user.type";
import { Wfm } from "$types/index";

export interface InitializeResponds {
  app_info: AppInfo;
  settings: Settings;
  stock_items: StockItem[];
  stock_rivens: StockRiven[];
  transactions: TransactionDto[];
  user: User;
  valid: boolean;
  orders?: Wfm.OrderDto[];
  auctions?: Wfm.Auction<string>[];
  chats?: Wfm.ChatData[];
}