import { PaginatedDto, PriceHistory, SubType, UserStatus } from "./global.type";
import { TauriTypes } from "./tauri.type";

export namespace WFMarketTypes {
  export enum OrderType {
    Buy = "buy",
    Sell = "sell",
    Closed = "closed",
  }
  export interface Order {
    createdAt: Date;
    id: string;
<<<<<<< HEAD
    last_update: Date;
    user?: UserDto;
    operation: string;
    type: string;
    platform: string;
=======
    itemId: string;
    perTrade?: number;
>>>>>>> better-backend
    platinum: number;
    properties: ItemProperties | null;
    quantity: number;
<<<<<<< HEAD
    region: string;
    itemId: string;
=======
    rank: number;
    cyanStars?: number;
    amberStars?: number;
    type: string;
    subtype: string;
    updatedAt: Date;
>>>>>>> better-backend
    visible: boolean;
    user?: User;
  }
  export interface OrderDetails extends TauriTypes.BaseItemDetails {}

<<<<<<< HEAD
  export interface OrderInfo {
    wfm_url: string;
    closed_avg?: number;
    moving_avg?: number;
    highest_price?: number;
    lowest_price?: number;
    orders?: OrderDto[];
    price_history: PriceHistory[];
    total_buyers?: number;
    total_sellers?: number;
    profit?: number;
    name: string;
    image: string;
    range?: number;
  }

  export interface OrderItemDto {
=======
  export interface User {
>>>>>>> better-backend
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
    operations: string[];
    order_id: string;
    orders: Order[];
    profit: number;
    quantity: number;
    sub_type: SubType;
    trade_sub_type?: TauriTypes.CacheTradableItemSubType;
    price_history: PriceHistory[];
  }
  export interface AuctionProperties {
    auction_id: string;
    auctions: Auction[];
    highest_price: number;
    lowest_price: number;
    operation: string[];
    item_name: string;
    image_url: string;
    can_import: boolean;
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
    effect?: string;
    properties?: Record<string, any>;
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
