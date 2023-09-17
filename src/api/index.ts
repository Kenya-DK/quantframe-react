import { SetupResponse, Wfm, InventoryEntryDto, TransactionEntryDto, Settings, CreateTransactionEntryDto } from '../types'
import { invoke } from '@tauri-apps/api';
import { SendTauriEvent, SendTauriUpdateDataEvent } from '../utils/tauri';
const api = {
  base: {
    updatesettings: async (settings: Settings): Promise<Settings | undefined> => {
      SendTauriUpdateDataEvent("settings", { data: settings, operation: "SET" })
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
    init: async (): Promise<SetupResponse> => {
      const data = await invoke("init") as SetupResponse;
      return data;
    },
  },
  items: {},
  transactions: {
    async create_transaction_entry(input: CreateTransactionEntryDto): Promise<TransactionEntryDto> {
      return await invoke("create_transaction_entry", { id: input.item_id, ttype: input.transaction_type || "buy", quantity: input.quantity, price: input.price, rank: input.rank }) as TransactionEntryDto;
    }
  },
  price_scraper: {
    async start_scraper(days: number): Promise<any> {
      SendTauriEvent("PriceScraper:OnChange", { max: 7, min: 0, current: 0.1 })
      await invoke("generate_price_history", { platform: "pc", days })
    },
  },
  live_scraper: {
    async start_scraper(): Promise<any> {
      SendTauriEvent("LiveScraper:Toggle")
      await invoke("toggle_live_scraper")
    }
  },
  whisper_scraper: {
    async start_scraper(): Promise<any> {
      SendTauriEvent("WhisperScraper:Toggle")
      await invoke("toggle_whisper_scraper")
    }
  },
  inventory: {
    async createInvantoryEntry(input: CreateTransactionEntryDto): Promise<InventoryEntryDto> {
      return await invoke("create_invantory_entry", { id: input.item_id, report: input.report || true, quantity: input.quantity, price: input.price, rank: input.rank }) as InventoryEntryDto;
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