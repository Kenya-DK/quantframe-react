import { TauriClient } from "..";
import { CacheRivenAttribute, CacheRivenUpgrade, CacheRivenWeapon, CacheTradableItem } from "@api/types";


enum CacheType {
  TradableItems = 'tradable_items',
  RivenWeapons = 'riven_weapons',
  RivenAttributes = 'riven_attributes',
  RivenDataByInternalId = 'riven_data_by_internal_id',
}

export class CacheModule {
  private readonly _cache: Map<string, any> = new Map();

  constructor(private readonly client: TauriClient) { }

  async reload(): Promise<void> {
    const [err, _] = await this.client.sendInvoke<void>('cache_reload');
    if (err)
      throw err;
    this._cache.clear();
  }

  async getTradableItems(): Promise<CacheTradableItem[]> {
    if (this._cache.has(CacheType.TradableItems))
      return this._cache.get(CacheType.TradableItems);
    const [err, items] = await this.client.sendInvoke<CacheTradableItem[]>('cache_get_tradable_items');
    if (err)
      throw err;
    this._cache.set(CacheType.TradableItems, items);
    return items;
  }
  async getTradableItem(input: string, by: string): Promise<CacheTradableItem> {
    const [err, res] = await this.client.sendInvoke<CacheTradableItem>('cache_get_tradable_item', { input, by });
    if (err)
      throw err;
    return res;
  }
  // Rivens
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

  async getWeaponUpgrades(internalId: string): Promise<Record<string, CacheRivenUpgrade>> {
    const [err, items] = await this.client.sendInvoke<Record<string, CacheRivenUpgrade>>('cache_get_weapon_upgrades', { internalId });
    if (err)
      throw err;
    return items;
  }

}
