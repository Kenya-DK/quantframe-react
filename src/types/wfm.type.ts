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
    // background: string|null,
    role: 'user',
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
    // Get from warframe items npm package
    category: string,
    max_rank: number,
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
  export type OrderType = 'sell' | 'buy';
  export interface OrderDto {
    id: string;
    platinum: number;
    quantity: number;
    order_type: OrderType;
    platform: string;
    region: string;
    creation_date: string;
    last_update: string;
    subtype: string;
    visible: boolean;
    item: OrderItemDto;
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
    ducats: number;
    quantity_for_set: number;
    en: LangItemNameDto;
    ru: LangItemNameDto;
    ko: LangItemNameDto;
    fr: LangItemNameDto;
    de: LangItemNameDto;
    sv: LangItemNameDto;
    zh_hant: LangItemNameDto;
    zh_hans: LangItemNameDto;
    pt: LangItemNameDto;
    es: LangItemNameDto;
    pl: LangItemNameDto;
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
    thumb: string;
    mastery_level: number;
    url_name: string;
    icon: string;
    id: string;
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
    attributes: RivenAttributeDto[],
    re_rolls: number;
    polarity: string;
  }

  export interface RivenAttributeInfoDto {
    negative_only: boolean;
    effect: string;
    id: string;
    exclusive_to: string[];
    group: string;
    units: string;
    search_only: boolean;
    url_name: string;
    suffix: string;
    positive_is_negative: boolean;
    prefix: string;
  }
  export interface RivenAttributeDto extends RivenAttributeInfoDto {
    positive: boolean;
    value: number;
  }
}