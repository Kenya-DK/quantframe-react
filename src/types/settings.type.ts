import { Wfm } from ".";

export interface Settings {
  mastery_rank: 2, // Trading is unlocked at MR2
  user_email: '',
  user_password: '',
  access_token: string | undefined,
  budget: 0,
  current_plat: 0,
}

export interface CacheBase {
  createdAt: number,
}

export interface TradableItemsCache extends CacheBase {
  items: Wfm.ItemDto[],
}
export interface Cache {
  tradableItems: TradableItemsCache,
}