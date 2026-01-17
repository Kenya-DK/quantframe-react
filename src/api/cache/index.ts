import { TauriClient } from "..";
import { TauriTypes } from "$types";
import { useQuery } from "@tanstack/react-query";
enum CacheType {
  TradableItems = "tradable_items",
  RivenWeapons = "riven_weapons",
  RivenAttributes = "riven_attributes",
  ChatIcons = "chat_icons",
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
  async getTradableItemById(id: string): Promise<TauriTypes.CacheTradableItem | undefined> {
    let items = await this.getTradableItems();
    return items.find((i) => i.wfm_id === id);
  }
<<<<<<< HEAD
  // Rivens
  async getRivenWeapons(): Promise<TauriTypes.CacheRivenWeapon[]> {
    if (this._cache.has(CacheType.RivenWeapons)) return this._cache.get(CacheType.RivenWeapons);
    let [err, items] = await this.client.sendInvoke<TauriTypes.CacheRivenWeapon[]>("cache_get_riven_weapons");
    if (err || !items) throw err;

    items = items.filter((i) => !i.is_variant);
    this._cache.set(CacheType.RivenWeapons, items);
    return items;
=======
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
  openThemeFolder(): Promise<void> {
    return this.client.sendInvoke<void>("cache_open_theme_folder");
>>>>>>> better-backend
  }
  async getRivenAttributes(): Promise<TauriTypes.CacheRivenAttribute[]> {
    if (this._cache.has(CacheType.RivenAttributes)) return this._cache.get(CacheType.RivenAttributes);
    const items = await this.client.sendInvoke<TauriTypes.CacheRivenAttribute[]>("cache_get_riven_attributes");
    this._cache.set(CacheType.RivenAttributes, items);
    return items;
  }
  async getChatIcons(): Promise<TauriTypes.CacheChatIcon[]> {
    if (this._cache.has(CacheType.ChatIcons)) return this._cache.get(CacheType.ChatIcons);
    const items = await this.client.sendInvoke<TauriTypes.CacheChatIcon[]>("cache_get_chat_icons");
    this._cache.set(CacheType.ChatIcons, items);
    return items;
  }
  async getRivenWeapons(): Promise<TauriTypes.CacheRivenWeapon[]> {
    if (this._cache.has(CacheType.RivenWeapons)) return this._cache.get(CacheType.RivenWeapons);
    let items = await this.client.sendInvoke<TauriTypes.CacheRivenWeapon[]>("cache_get_riven_weapons");
    items = items.filter((i) => !i.is_variant);
    this._cache.set(CacheType.RivenWeapons, items);
    return items;
  }
  async getRivenWeaponsById(id: string): Promise<TauriTypes.CacheRivenWeapon | undefined> {
    let items = await this.getRivenWeapons();
    return items.find((i) => i.wfm_id === id);
  }
  async getWeaponByUrl(id: string): Promise<TauriTypes.CacheRivenWeapon | undefined> {
    let items = await this.getRivenWeapons();
    return items.find((i) => i.wfm_url_name === id);
  }
  async get_chat_link(unique_name: string): Promise<TauriTypes.ChatLink> {
    return await this.client.sendInvoke<TauriTypes.ChatLink>("cache_get_chat_link", { unique_name });
  }
}
