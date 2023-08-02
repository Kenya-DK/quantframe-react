import { axiosInstance } from './axios'

import { Wfm } from '../types'
import { settings, cache } from "@store/index";
const WarframeItems = require('warframe-items')
// Docs https://warframe.market/api_docs

const api = {
  auth: {
    async login(email: string, password: string): Promise<Wfm.UserDto> {
      const { data, headers } = await axiosInstance.post('/auth/signin', { email, password });
      let access_token = headers['set-cookie'] as string | undefined
      access_token = access_token ? access_token.slice(4).split(';')[0] : undefined;
      if (!access_token) throw new Error('This shouldn\'t happen')
      await settings.set('access_token', access_token)
      return data.payload.user
    },
    async logout() {
      await settings.set('access_token', undefined)
    }
  },
  items: {
    async list(): Promise<Wfm.ItemDto[]> {
      const { tradableItems } = await cache.get();
      // If cache is older than 24 hours then refresh it
      if (tradableItems.createdAt + 1000 * 60 * 60 * 24 < Date.now()) {
        const { data: { payload: { items } } } = await axiosInstance.get('/items', {});
        // I Use this to find the category names for the warframe market api.
        const wfItems = new WarframeItems({ category: ['All'] });

        const wfmItems = items.map((item: Wfm.ItemDto) => {
          const wfmItem = wfItems.find((i: any) => i.marketInfo?.id === item.id || i.name === item.item_name)
          return {
            ...item,
            category: wfmItem?.category || "Unknown",
          }
        });

        await cache.update({
          tradableItems: {
            createdAt: Date.now(),
            items: wfmItems
          }
        });
        return wfmItems
      }
      return tradableItems.items
    },
    async findByName(name: string): Promise<Wfm.ItemDto | undefined> {
      const items = await this.list();
      return items.find(item => item.item_name === name);
    },
    async findById(id: string): Promise<Wfm.ItemDto | undefined> {
      return this.list().then(items => items.find(item => item.id === id))
    },
    async findByUrlName(url_name: string): Promise<Wfm.ItemDto | undefined> {
      const items = await this.list();
      return items.find(item => item.url_name === url_name);
    }
  },
}

export default api

export const wfmThumbnail = (thumb: string) => `https://warframe.market/static/assets/${thumb}`