import { TauriClient } from "..";
import { TauriTypes } from "$types";
import { useQuery } from "@tanstack/react-query";
enum CacheType {
  TradableItems = "tradable_items",
  RivenWeapons = "riven_weapons",
  RivenAttributes = "riven_attributes",
  RivenDataByInternalId = "riven_data_by_internal_id",
}
export class CacheModule {
  private readonly _cache: Map<string, any> = new Map();
  constructor(private readonly client: TauriClient) {}

  async getTradableItems(): Promise<TauriTypes.CacheTradableItem[]> {
    if (this._cache.has(CacheType.TradableItems)) return this._cache.get(CacheType.TradableItems);
    const items = await this.client.sendInvoke<TauriTypes.CacheTradableItem[]>("cache_get_tradable_items");
    this._cache.set(CacheType.TradableItems, items);
    return items;
  }
  getThemePresets() {
    return useQuery({
      queryKey: ["cache_get_theme_presets"],
      queryFn: () => this.client.sendInvoke<TauriTypes.CacheTheme[]>("cache_get_theme_presets"),
      retry: false,
    });
  }
  createTheme(name: string, author: string, properties: any): Promise<void> {
    return this.client.sendInvoke<void>("cache_create_theme", { name, author, properties });
  }
}
