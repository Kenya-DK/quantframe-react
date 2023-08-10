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
    },
    isTokenValid: async (): Promise<boolean> => {
      const res = await api.orders.createOrder({
        item: "56783f24cbfa8f0432dd89a2",
        order_type: "buy",
        platinum: 1,
        quantity: 1,
        visible: false,
      });
      if (!res) return false
      await api.orders.deleteOrder(res.id);
      return true
    },
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
      const data = await fetch('https://raw.githubusercontent.com/WFCD/warframe-items/master/data/json/All.json');
      const wfItems = await data.json();
      const wfmItems = items.map((item: Wfm.ItemDto) => {
        const wfmItem = wfItems.find((i: any) => i.marketInfo?.id === item.id || i.name === item.item_name)
        return {
          ...item,
          category: wfmItem?.category || "Unknown",
          max_rank: wfmItem?.fusionLimit || (wfmItem?.levelStats?.length - 1) || 0,
        }
      });
      await cache.update({
        tradableItems: {
          createdAt: Date.now(),
          items: wfmItems
        }
      });
      return wfmItems
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
    async getItemDetails(url_name: string): Promise<Wfm.ItemDetailsDto> {
      const { data } = await axiosInstance.get(`items/${url_name}`, {});
      return data.payload.item
    }
  },
  itemprices: {
    async updatePriceHistory(days: number = 7): Promise<PriceHistoryDto[]> {
      const priceHistorys = await PriceScraper.list(days);
      return priceHistorys
    }
  },
  orders: {
    async getOrders(): Promise<Wfm.OrderDto[]> {
      const { ingame_name } = await user.get();
      const { data } = await axiosInstance.get(`profile/${ingame_name}/orders`);

      console.log(data);

      return []
    },
    async createOrder(order: Wfm.CreateOrderDto): Promise<Wfm.OrderDto | undefined> {
      const { ingame_name } = await user.get();
      try {
        const { data } = await axiosInstance.post(`profile/${ingame_name}/orders`, order);
        return data.payload.order
      } catch (error) {
        return undefined;
      }
    },
    async deleteOrder(id: string) {
      const { ingame_name } = await user.get();
      // TODO: Update in database
      await axiosInstance.delete(`profile/${ingame_name}/orders/${id}`, {});
    },
    async deleteAllOrders() {
      const orders = await this.getOrders();
      const promises = orders.map(order => this.deleteOrder(order.id))
      await Promise.all(promises);
    },
  },
}

export default api

export const wfmThumbnail = (thumb: string) => `https://warframe.market/static/assets/${thumb}`