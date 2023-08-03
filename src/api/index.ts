import { axiosInstance } from './axios'

import { PriceHistoryDto, Wfm } from '../types'
import { settings, user, cache } from "@store/index";
import PriceScraper from './priceScraper';
// Docs https://warframe.market/api_docs
const chachTime = 1000 * 60 * 60 * 24 // 24 hours
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
    async getTradableItems(): Promise<Wfm.ItemDto[]> {
      const { tradableItems } = await cache.get();
      // If cache is older than 24 hours then refresh it
      if (tradableItems.createdAt + chachTime < Date.now())
        return api.items.updateTradableItems()
      return tradableItems.items
    },
    async updateTradableItems(): Promise<Wfm.ItemDto[]> {
      const { data: { payload: { items } } } = await axiosInstance.get('/items', {});
      await cache.update({
        tradableItems: {
          createdAt: Date.now(),
          items: items
        }
      });
      return items
    },
    async findByName(name: string): Promise<Wfm.ItemDto | undefined> {
      const items = await this.getTradableItems();
      return items.find(item => item.item_name === name);
    },
    async findById(id: string): Promise<Wfm.ItemDto | undefined> {
      return this.getTradableItems().then(items => items.find(item => item.id === id))
    },
    async findByUrlName(url_name: string): Promise<Wfm.ItemDto | undefined> {
      const items = await this.getTradableItems();
      return items.find(item => item.url_name === url_name);
    },
  },
  itemprices: {
    async priceHistory(): Promise<PriceHistoryDto[]> {
      const { priceHistory } = await cache.get();
      // If cache is older than 24 hours then refresh it
      if (priceHistory.createdAt + chachTime < Date.now())
        return await api.itemprices.updatePriceHistory()
      return priceHistory.items
    },
    async updatePriceHistory() {
      const priceHistorys = await PriceScraper.list(7);
      await cache.update({
        priceHistory: {
          createdAt: Date.now(),
          items: priceHistorys
        }
      });
      return priceHistorys
    }
  },
  orders: {
    async getOrders(): Promise<Wfm.OrderDto[]> {
      const { ingame_name } = await user.get();
      const { data: { payload: { items } } } = await axiosInstance.get(`profile/${ingame_name}/orders`, {});

      return items as Wfm.OrderDto[]
    },
    async deleteOrder(id: string) {
      const { ingame_name } = await user.get();
      // TODO: Update in database
      await axiosInstance.delete(`profile/${ingame_name}/orders/${id}`, {});
    },
    async deleteAllOrders() {
      const promises = (await this.getOrders()).map(order => this.deleteOrder(order.id))
      await Promise.all(promises);
    },
  },
}

export default api

export const wfmThumbnail = (thumb: string) => `https://warframe.market/static/assets/${thumb}`