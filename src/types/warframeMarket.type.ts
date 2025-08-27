import { SubType, UserStatus } from "./global.type";

export namespace WFMarketTypes {
  export interface Order {
    createdAt: Date;
    id: string;
    itemId: string;
    perTrade?: number;
    platinum: number;
    properties: Properties | null;
    quantity: number;
    rank: number;
    type: string;
    updatedAt: Date;
    visible: boolean;
    user?: User;
  }
  export interface User {
    id: string;
    ingame_name: string;
    reputation: number;
    avatar?: string;
    status: UserStatus;
  }
  export interface Properties {
    closed_avg: number;
    highest_price: number;
    item_id: string;
    item_name: string;
    lowest_price: number;
    operation: string[];
    order_id: string;
    orders: Order[];
    profit: number;
    quantity: number;
    sub_type: SubType;
  }
}
