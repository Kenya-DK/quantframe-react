import { SetupResponse, Wfm, InventoryEntryDto, TransactionEntryDto, Settings } from '../types'
import { invoke } from '@tauri-apps/api';
const api = {
  base: {
    updatesettings: async (settings: Settings): Promise<Settings | undefined> => {
      return await invoke("update_settings", { settings })
    }
  },
  auth: {
    async login(email: string, password: string): Promise<Wfm.UserDto> {
      const user = await invoke("login", {
        email: email,
        password: password,
      }) as Wfm.UserDto;
      return user
    },
    async logout() {
      // await settings.set('access_token', undefined)
    },
    validate: async (): Promise<SetupResponse> => {
      const data = await invoke("setup") as SetupResponse;
      return data;
    },
  },
  items: {},
  transactions: {
    async create_transaction_entry(id: string, quantity: number, price: number, rank: number, type: string): Promise<TransactionEntryDto> {
      return await invoke("create_transaction_entry", { id, ttype: type, quantity, price, rank }) as TransactionEntryDto;
    }
  },
  inventory: {
    async createInvantoryEntry(id: string, quantity: number, price: number, rank: number): Promise<InventoryEntryDto> {
      return await invoke("create_invantory_entry", { id, quantity, price, rank }) as InventoryEntryDto;
    },
    async sellInvantoryEntry(id: number, price: number): Promise<InventoryEntryDto> {
      return await invoke("sell_invantory_entry", { id, price }) as InventoryEntryDto;
    },
    async deleteInvantoryEntry(id: number): Promise<InventoryEntryDto> {
      return await invoke("delete_invantory_entry", { id });
    },
  },
  orders: {
  },
}

export default api

export const wfmThumbnail = (thumb: string) => `https://warframe.market/static/assets/${thumb}`