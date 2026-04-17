import { PaginatedDto, RivenAttribute, UserStatus } from "./global.type";

export namespace WFMarketTypes {
  export enum OrderType {
    All = "all",
    Buy = "buy",
    Sell = "sell",
    Closed = "closed",
  }
  export interface Order<T = any> {
    createdAt: Date;
    id: string;
    itemId: string;
    per_trade?: number;
    platinum: number;
    properties?: T;
    quantity: number;
    rank: number;
    cyanStars?: number;
    amberStars?: number;
    type: string;
    subtype: string;
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

  export interface Auction<T = any> {
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
    properties?: T;
    starting_price: number;
    top_bid: unknown | null;
    updated: string;
    uuid: string;
    visible: boolean;
    owner: User | null;
    winner: unknown | null;
  }
  export interface Item {
    attributes: RivenAttribute[];
    mastery_level: number;
    mod_rank: number;
    name: string;
    polarity: string;
    re_rolls: number;
    similarity: Similarity;
    type: string;
    weapon_url_name: string;
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
  export type WfmAuctionControllerGetListData = PaginatedDto & {
    results?: WFMarketTypes.Auction[];
  };
  export interface WfmAuctionControllerGetListParams {
    page: number;
    limit: number;
    sort_by?: string;
    sort_direction?: "asc" | "desc";
    auction_type?: string;
    query?: string;
  }
  export interface WfmChatDataControllerGetListParams {
    page: number;
    limit: number;
    sort_by?: string;
    sort_direction?: "asc" | "desc";
    query?: string;
  }
  export type WfmChatDataControllerGetListData = PaginatedDto & {
    results?: WFMarketTypes.ChatData[];
  };
  export interface ChatData {
    id: string;
    chat_with: User[];
    unread_count: number;
    chat_name: string;
    messages: ChatMessage[];
    last_update: string;
  }

  export interface ChatMessageSent {
    message: ChatMessage;
    temp_id: string;
  }

  export interface ChatMessage {
    id: string;
    message: string;
    chat_id: string;
    send_date: string;
    message_from: string;
    raw_message: string;
    requirer_refresh?: boolean;
  }
}
