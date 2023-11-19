import { SetupResponse, Wfm, WeeklyRiven, TransactionEntryDto, Settings, CreateTransactionEntryDto, CreateStockItemEntryDto, StockItemDto, CreateStockRivenEntryDto, StockRivenDto } from '../types'
import { invoke } from '@tauri-apps/api';
import { SendTauriEvent, SendTauriUpdateDataEvent } from '../utils/tauri';
const api = {
  base: {
    updatesettings: async (settings: Settings): Promise<Settings | undefined> => {
      SendTauriUpdateDataEvent("settings", { data: settings, operation: "SET" })
      return await invoke("update_settings", { settings })
    },
    get_weekly_rivens: async (): Promise<WeeklyRiven[]> => {
      return await invoke("get_weekly_rivens") as WeeklyRiven[];
    },
    openLogsFolder: async (): Promise<any> => {
      return await invoke("open_logs_folder")
    },
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
    async create(input: CreateTransactionEntryDto): Promise<TransactionEntryDto> {
      return await invoke("create_transaction_entry", {
        id: input.item_id,
        itemType: input.item_type,
        ttype: input.transaction_type || "buy",
        quantity: input.quantity,
        price: input.price,
        rank: input.rank,
        subType: input.sub_type,
        attributes: input.attributes,
        masteryRank: input.mastery_rank,
        reRolls: input.re_rolls,
        polarity: input.polarity
      }) as TransactionEntryDto;
    },
    async delete(id: number): Promise<TransactionEntryDto> {
      return await invoke("delete_transaction_entry", { id }) as TransactionEntryDto;
    },
    update: async (id: number, transaction: Partial<TransactionEntryDto>): Promise<any> => {
      return await invoke("update_transaction_entry", {
        id,
        price: transaction.price,
        transaction_type: transaction.transaction_type,
        quantity: transaction.quantity,
        rank: transaction.rank
      }) as TransactionEntryDto;
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
  stock: {
    item: {
      create: async (input: CreateStockItemEntryDto): Promise<StockItemDto> => {
        return await invoke("create_item_stock", {
          id: input.item_id,
          quantity: input.quantity,
          price: input.price,
          miniumPrice: input.minium_price,
          rank: input.rank,
          subType: input.sub_type
        }) as StockItemDto;
      },
      delete: async (id: number): Promise<StockItemDto> => {
        return await invoke("delete_item_stock", { id }) as StockItemDto;
      },
      sell: async (id: number, price: number, quantity: number): Promise<StockItemDto> => {
        return await invoke("sell_item_stock", { id, price, quantity }) as StockItemDto;
      },
      sell_by_name: async (name: string, price: number, quantity: number): Promise<StockItemDto> => {
        return await invoke("sell_item_stock_by_url", { name, price, quantity }) as StockItemDto;
      },
      update: async (id: number, item: Partial<StockItemDto>): Promise<StockItemDto> => {
        return await invoke("update_item_stock", { id, miniumPrice: item.minium_price }) as StockItemDto;
      }
    },
    riven: {
      create: async (input: CreateStockRivenEntryDto): Promise<StockRivenDto> => {
        return await invoke("create_riven_stock", {
          id: input.item_id,
          price: input.price,
          miniumPrice: input.minium_price,
          rank: input.rank,
          attributes: input.attributes,
          masteryRank: input.mastery_rank,
          matchRiven: input.match_riven,
          reRolls: input.re_rolls,
          polarity: input.polarity,
          modName: input.mod_name,
        }) as StockRivenDto;
      },
      delete: async (id: number): Promise<StockRivenDto> => {
        return await invoke("delete_riven_stock", { id }) as StockRivenDto;
      },
      sell: async (id: number, price: number): Promise<StockRivenDto> => {
        return await invoke("sell_riven_stock", { id, price }) as StockRivenDto;
      },
      import_auction: async (id: string, price: number): Promise<StockRivenDto> => {
        return await invoke("import_auction", { id, price }) as StockRivenDto;
      },
      update: async (id: number, riven: Partial<StockRivenDto>): Promise<StockRivenDto> => {
        console.log(riven);
        if (riven.minium_price && riven.minium_price <= 0)
          riven.minium_price = -1;
        return await invoke("update_riven_stock", { id, attributes: riven.attributes, matchRiven: riven.match_riven, miniumPrice: riven.minium_price }) as StockRivenDto;
      }
    }
  },
  auction: {
    search: async (query: Wfm.AuctionSearchQueryDto): Promise<Wfm.Auction<Wfm.AuctionOwner>[]> => {
      return await invoke("auction_search", {
        ...query,
        auctionType: query.auction_type,
        weaponUrlName: query.weapon_url_name,
        positiveStats: query.positive_stats,
        negativeStats: query.negative_stats,
        masteryRankMin: query.mastery_rank_min,
        masteryRankMax: query.mastery_rank_max,
        reRollsMin: query.re_rolls_min,
        reRollsMax: query.re_rolls_max,
        buyoutPolicy: query.buyout_policy,
        sortBy: query.sort_by,
      }) as Wfm.Auction<Wfm.AuctionOwner>[];
    },
    refresh: async () => {
      await invoke("refresh_auctions");
    },
    async delete_all(): Promise<number> {
      const rep = await invoke("delete_all_auctions") as { count: number };
      return rep.count;
    }
  },
  orders: {
    refresh: async () => {
      await invoke("refresh_orders");
    },
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
    },
    async delete_all(): Promise<number> {
      const rep = await invoke("delete_all_orders") as { count: number };
      return rep.count;
    }
  },
}

export default api

export const wfmThumbnail = (thumb: string) => `https://warframe.market/static/assets/${thumb}`