import { SetupResponse, Wfm, InventoryEntryDto, TransactionEntryDto, Settings } from '../types'
import { invoke } from '@tauri-apps/api';
const api = {
  base: {
    updatesettings: async (settings: Settings): Promise<Settings | undefined> => {
      return await invoke("update_settings", { settings })
    }
  },
  debug: {
    importWarframeAlgoTraderData: async (dbPath: string, type: string): Promise<any> => {
      try {
        return await invoke("import_warframe_algo_trader_data", { dbPath, importType: type })
      } catch (error) {
        console.error(error)
      }
    },
    reset_data: async (reset_type: string): Promise<any> => {
      try {
        return await invoke("reset_data", { resetType: reset_type })
      } catch (error) {
        console.error(error)
      }
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
    async createInvantoryEntry(id: string, report: boolean, quantity: number, price: number, rank: number): Promise<InventoryEntryDto> {
      return await invoke("create_invantory_entry", { id, report, quantity, price, rank }) as InventoryEntryDto;
    },
    async sellInvantoryEntry(id: number, report: boolean, price: number, quantity: number): Promise<InventoryEntryDto> {
      return await invoke("sell_invantory_entry", { id, report, price, quantity }) as InventoryEntryDto;
    },
    async deleteInvantoryEntry(id: number): Promise<InventoryEntryDto> {
      return await invoke("delete_invantory_entry", { id });
    },
  },
  orders: {
    async getOrders(): Promise<Wfm.OrderDto[]> {
      return await invoke("get_orders") as Wfm.OrderDto[];
    },
    async deleteOrder(id: string): Promise<Wfm.OrderDto> {
      return await invoke("delete_order", { id }) as Wfm.OrderDto;
    },
    async createOrder(id: string, quantity: number, price: number, rank: number, type: string): Promise<Wfm.OrderDto> {
      return await invoke("create_order", { id, order_type: type, quantity, price, rank }) as Wfm.OrderDto;
    },
    async updateOrder(id: string, quantity: number, price: number, rank: number, type: string): Promise<Wfm.OrderDto> {
      return await invoke("update_order", { id, order_type: type, quantity, price, rank }) as Wfm.OrderDto;
    }
  },
}

export default api

export const wfmThumbnail = (thumb: string) => `https://warframe.market/static/assets/${thumb}`