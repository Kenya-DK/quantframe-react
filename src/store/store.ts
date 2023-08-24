import { merge } from 'lodash'
import { Store } from "tauri-plugin-store-api";
import { SETTINGS_FILE, Cache, Settings, Wfm } from "../types";
export const store = new Store(SETTINGS_FILE)
class Persist<T> {
  private cache: T | null;
  constructor(public name: string, public defaults: T) {
    this.cache = null
  }
  async get(): Promise<T> {
    if (this.cache)
      return this.cache;
    else
      this.cache = await store.get<T>(this.name) || this.defaults;
    return this.cache;
  }
  async set(key: keyof T, value: typeof this.defaults[typeof key]) {
    const currentSettings = await store.get<T>(this.name)
    // @ts-ignore
    currentSettings[key] = value
    const promise = store.set(this.name, currentSettings)
    await store.save()
    this.cache = currentSettings;
    return promise
  }
  async update(newSettings: Partial<T>) {
    const currentSettings = await store.get<T>(this.name)
    const promise = store.set(this.name, merge(this.defaults, currentSettings, newSettings))
    await store.save()
    this.cache = await store.get<T>(this.name)
    return promise
  }
  async reset() {
    const promise = store.set(this.name, this.defaults)
    await store.save()
    this.cache = this.defaults;
    return promise
  }
}

export const settings = new Persist<Settings>('settings', {
  mastery_rank: 2, // Trading is unlocked at MR2
  user_email: '',
  user_password: '',
  access_token: undefined,
  volume_threshold: 1,
  range_threshold: 10,
  avg_price_cap: 600,
  max_total_price_cap: 100000,
  price_shift_threshold: -1,
  blacklist: [],
  whitelist: [],
  strict_whitelist: false,
})

export const user = new Persist<Wfm.UserDto>('user', {
  banned: false,
  id: '',
  avatar: '',
  ingame_name: '',
  locale: 'en',
  platform: 'pc',
  region: 'en',
  role: 'user',
})


export const cache = new Persist<Cache>('cache', {
  tradableItems: { createdAt: 0, items: [] },
})