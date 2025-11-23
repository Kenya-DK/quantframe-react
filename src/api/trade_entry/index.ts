import { TauriClient } from "..";
import { TauriTypes } from "../../types";
export class TradeEntryModule {
  constructor(private readonly client: TauriClient) {}

  async getPagination(query: TauriTypes.TradeEntryControllerGetListParams): Promise<TauriTypes.TradeEntryControllerGetListData> {
    return await this.client.sendInvoke<TauriTypes.TradeEntryControllerGetListData>("get_trade_entry_pagination", {
      query: this.client.convertToTauriQuery(query),
    });
  }

  async create(input: TauriTypes.CreateTradeEntry) {
    return await this.client.sendInvoke<TauriTypes.TradeEntry>("trade_entry_create", { input });
  }
  async createMultiple(inputs: TauriTypes.CreateTradeEntry[]): Promise<TauriTypes.TradeEntry[]> {
    return await this.client.sendInvoke<TauriTypes.TradeEntry[]>("trade_entry_create_multiple", { inputs });
  }

  async update(input: TauriTypes.UpdateTradeEntry): Promise<TauriTypes.TradeEntry> {
    return await this.client.sendInvoke<TauriTypes.TradeEntry>("trade_entry_update", { input });
  }
  async updateMultiple(ids: number[], input: TauriTypes.UpdateTradeEntry): Promise<TauriTypes.TradeEntry[]> {
    return await this.client.sendInvoke<TauriTypes.TradeEntry[]>("trade_entry_update_multiple", { ids, input });
  }
  async delete(id: number): Promise<TauriTypes.TradeEntry> {
    return await this.client.sendInvoke<TauriTypes.TradeEntry>("trade_entry_delete", { id });
  }
  async deleteMultiple(ids: number[]): Promise<number> {
    return await this.client.sendInvoke<number>("trade_entry_delete_multiple", { ids });
  }
  async getById(id: number): Promise<TauriTypes.TradeEntryDetails> {
    return await this.client.sendInvoke<TauriTypes.TradeEntryDetails>("trade_entry_get_by_id", { id });
  }
  exportJson = async (query: TauriTypes.TradeEntryControllerGetListParams): Promise<string> => {
    return await this.client.sendInvoke<string>("export_trade_entry_json", {
      query: this.client.convertToTauriQuery(query),
    });
  };
}
