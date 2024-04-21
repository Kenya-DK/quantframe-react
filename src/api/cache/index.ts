import { TauriClient } from "..";
import { CacheRivenAttribute, CacheRivenWeapon, CacheTradableItem } from "@api/types";


enum CacheType {
  TradableItems = 'tradable_items',
  RivenWeapons = 'riven_weapons',
  RivenAttributes = 'riven_attributes',
}

export class CacheModule {
  private readonly _cache: Map<string, any> = new Map();

  constructor(private readonly client: TauriClient) { }

  async getTradableItems(): Promise<CacheTradableItem[]> {
    if (this._cache.has(CacheType.TradableItems))
      return this._cache.get(CacheType.TradableItems);
    const [err, items] = await this.client.sendInvoke<CacheTradableItem[]>('cache_get_tradable_items');
    if (err)
      throw err;
    this._cache.set(CacheType.TradableItems, items);
    return items;
  }

  async getRivenWeapons(): Promise<CacheRivenWeapon[]> {
    if (this._cache.has(CacheType.RivenWeapons))
      return this._cache.get(CacheType.RivenWeapons);
    const [err, items] = await this.client.sendInvoke<CacheRivenWeapon[]>('cache_get_riven_weapons');
    if (err)
      throw err;
    this._cache.set(CacheType.RivenWeapons, items);
    return items;
  }

  async getRivenAttributes(): Promise<CacheRivenAttribute[]> {
    if (this._cache.has(CacheType.RivenAttributes))
      return this._cache.get(CacheType.RivenAttributes);
    const [err, items] = await this.client.sendInvoke<CacheRivenAttribute[]>('cache_get_riven_attributes');
    if (err)
      throw err;
    this._cache.set(CacheType.RivenAttributes, items);
    return items;
  }
}
