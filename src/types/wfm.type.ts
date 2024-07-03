import { RivenAttribute, UserStatus } from "@api/types";

export namespace Wfm {

  export interface UserDto {
    // unread_messages: number
    // has_mail: number
    // check_code: string,
    // written_reviews: string,
    // verification: boolean,
    ingame_name: string,
    avatar: string,
    // anonymous: boolean,
    platform: 'pc',
    // reputation: number,
    // linked_accounts: {}
    id: string,
    region: 'en' | (string & {}),
    locale: 'en' | (string & {}),
    access_token: 'string',
    // background: string|null,
    role: 'user',
    status: UserStatus,
    // avatar: string,
    banned: boolean
  }

  export interface LinkedAccountsDto {
    steam_profile: boolean;
    patreon_profile: boolean;
    xbox_profile: boolean;
  }

  export interface PatreonProfileDto {
    patreon_founder: boolean;
    subscription: boolean;
    patreon_badge: string;
  }

  export interface ItemDto {
    id: string,
    item_name: string,
    url_name: string,
    thumb: string,
    wikia_url: string,
    mr_requirement: number,
    trade_tax: number,
    // Get from warframe items npm package
    set_items: string[] | null,
    tags: string[] | null,
    mod_max_rank: number | null,
    subtypes: string[] | null,
  }
  export interface ItemDetailsDto extends ItemDto {
    tags: string[];
    icon: string;
    icon_format: string;
    set_root: boolean;
    mastery_level: number;
    url_name: string;
    sub_icon: null | string;
    trading_tax: number;
    quantity_for_set?: number;
    ducats: number;
  }
  // Order stuff below
  export enum OrderType {
    Buy = 'buy',
    Sell = 'sell',
  }
  export interface OrderDto {
    id: string;
    platinum: number;
    quantity: number;
    order_type: OrderType;
    platform: string;
    region: string;
    creation_date: string;
    last_update: string;
    user: UserDto;
    visible: boolean;
    // Mod
    mod_rank?: number;
    // Ayatan Sculpture
    cyan_stars?: number;
    amber_stars?: number;
    // Subtype
    subtype?: string;
    item?: OrderItemDto;
  }
  export interface OrderItemDto {
    id: string;
    url_name: string;
    icon: string;
    icon_format: string;
    thumb: string;
    sub_icon: string;
    mod_max_rank: number;
    subtypes: string[];
    tags: string[];
    ducats?: number;
    vaulted?: boolean;
    quantity_for_set: number;
    en?: LangItemNameDto;
    ru?: LangItemNameDto;
    ko?: LangItemNameDto;
    fr?: LangItemNameDto;
    de?: LangItemNameDto;
    sv?: LangItemNameDto;
    zh_hant?: LangItemNameDto;
    zh_hans?: LangItemNameDto;
    pt?: LangItemNameDto;
    es?: LangItemNameDto;
    pl?: LangItemNameDto;
  }

  export interface LangItemNameDto {
    item_name: string;
    description?: string;
    wiki_link?: string;
  }

  export type CreateOrderDto = {
    item: string,
    order_type: OrderType,
    platinum: number,
    quantity: number,
    visible: boolean,
    rank?: number,
    subtype?: string,
  }
  export interface RivenItemTypeDto {
    mastery_level: number;
    url_name: string;
    icon: string;
    id: string;
    exclusive_to: string[];
    riven_type: string;
    icon_format: string;
    group: string;
    item_name: string;
  }
  export interface RivenItemDto {
    weapon_name: string;
    url_name: string;
    mod_name: string;
    mod_rank: number;
    mastery_rank: number;
    attributes: RivenAttribute[],
    re_rolls: number;
    polarity: string;
  }

  export interface RivenAttributeInfoDto {
    negative_only: boolean;
    effect: string;
    id: string;
    group: string;
    units: string;
    search_only: boolean;
    url_name: string;
    suffix: string;
    positive_is_negative: boolean;
    prefix: string;
  }

  export interface AuctionSearchQueryDto {
    auction_type: "riven" | "mod";
    weapon_url_name: string,
    positive_stats?: RivenAttribute[],
    negative_stats?: RivenAttribute,
    polarity?: string,
    mastery_rank_min?: number,
    mastery_rank_max?: number,
    re_rolls_min?: number,
    re_rolls_max?: number,
    buyout_policy?: string,
    sort_by?: string,
  }
  export interface Auction<T> {
    visible: boolean;
    minimal_reputation: number;
    item: AuctionItem;
    buyout_price: number | number;
    note: string;
    starting_price: number;
    owner: T;
    platform: string;
    closed: boolean;
    top_bid: null | any;
    winner: null | any;
    is_marked_for: null | any;
    marked_operation_at: null | any;
    created: string;
    updated: string;
    note_raw: string;
    is_direct_sell: boolean;
    id: string;
    private: boolean;
  }
  export enum AuctionStatus {
    Private = 'private',
    Visible = 'visible',
    Closed = 'closed',
  }
  export interface AuctionItem {
    mastery_level: number;
    re_rolls: number;
    type: string;
    weapon_url_name: string;
    attributes: RivenAttribute[];
    name: string;
    mod_rank: number;
    polarity: string;
    similarity?: number;
    missing_attributes: RivenAttribute[];
    extra_attributes: RivenAttribute[];
    //Liches And Sister
    element: string;
    quirk: string;
    having_ephemera: boolean;
    damage: number;
  }

  export interface AuctionOwner {
    reputation: number;
    locale: string;
    avatar: null;
    last_seen: string;
    ingame_name: string;
    status: string;
    id: string;
    region: string;
  }
  export interface ChatData {
    id: string;
    chat_with: ChatWith[];
    unread_count: number;
    chat_name: string;
    messages: ChatMessage[];
    last_update: string;
  }

  export interface ChatWith {
    reputation: number;
    locale: string;
    avatar: string;
    last_seen: string;
    ingame_name: string;
    status: UserStatus;
    id: string;
    region: string;
  }

  export interface ChatMessage {
    message: string;
    id: string;
    chat_id: string;
    send_date: string;
    message_from: string;
    raw_message: string;
  }

  export enum SocketEvent {
    OnError = 'ERROR',
    OnUserCountChange = 'MESSAGE/ONLINE_COUNT',
    OnUserStatusChange = 'USER/SET_STATUS',
    UpdateUserStatus = '@WS/USER/SET_STATUS',
  }

}