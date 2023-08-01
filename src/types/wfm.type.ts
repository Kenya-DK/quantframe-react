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

  export type ItemDto = {
    id: string,
    item_name: string,
    url_name: string,
    thumb: string,
  }

  // Order stuff below
  export interface OrderDto {
    id: string;
    platinum: number;
    quantity: number;
    order_type: string;
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
  }

  export type CreateOrderDto = {
    id: string,
    item_name: string,
    url_name: string,
    thumb: string,
  }

}