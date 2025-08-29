import { PaginatedDto, SubType, UserStatus } from "./global.type";

export namespace WFMarketTypes {
  export enum OrderType {
    Buy = "buy",
    Sell = "sell",
    Closed = "closed",
  }
  export interface Order {
    createdAt: Date;
    id: string;
    itemId: string;
    perTrade?: number;
    platinum: number;
    properties: ItemProperties | null;
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
  export interface ItemProperties {
    closed_avg: number;
    highest_price: number;
    item_id: string;
    item_name: string;
    image_url: string;
    lowest_price: number;
    operation: string[];
    order_id: string;
    orders: Order[];
    profit: number;
    quantity: number;
    sub_type: SubType;
  }
  export interface AuctionProperties {
    auction_id: string;
    auctions: Auction[];
    highest_price: number;
    lowest_price: number;
    operation: string[];
  }

  export interface Auction {
    buyout_price: number;
    closed: boolean;
    created: string;
    id: string;
    is_direct_sell: boolean;
    is_marked_for: string | null;
    item: Item;
    marked_operation_at: string | null;
    minimal_reputation: number;
    note: string;
    note_raw: string;
    platform: string;
    private: boolean;
    properties: AuctionProperties | null;
    starting_price: number;
    top_bid: unknown | null;
    updated: string;
    uuid: string;
    visible: boolean;
    owner: User | null;
    winner: unknown | null;
  }
  export interface Item {
    attributes: Attribute[];
    mastery_level: number;
    mod_rank: number;
    name: string;
    polarity: string;
    re_rolls: number;
    similarity: Similarity;
    type: string;
    weapon_url_name: string;
  }
  export interface Attribute {
    positive: boolean;
    url_name: string;
    value: number;
  }

  export interface Similarity {
    extra: any[];
    missing: any[];
    score: number;
  }
  export type WfmOrderControllerGetListData = PaginatedDto & {
    results?: WFMarketTypes.Order[];
  };
  export interface WfmOrderControllerGetListParams {
    page: number;
    limit: number;
    sort_by?: string;
    sort_direction?: "asc" | "desc";
    order_type?: WFMarketTypes.OrderType;
    query?: string;
  }
}
