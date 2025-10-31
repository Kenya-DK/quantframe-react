import { TauriClient } from "..";
import { TauriTypes } from "../../types";
export class StockItemModule {
  constructor(private readonly client: TauriClient) {}

  async getPagination(query: TauriTypes.StockItemControllerGetListParams): Promise<TauriTypes.StockItemControllerGetListData> {
    return await this.client.sendInvoke<TauriTypes.StockItemControllerGetListData>("get_stock_item_pagination", {
      query: this.client.convertToTauriQuery(query),
    });
  }

  async getFinancialReport(query: TauriTypes.StockItemControllerGetListParams): Promise<TauriTypes.FinancialReport> {
    return await this.client.sendInvoke<TauriTypes.FinancialReport>("get_stock_item_financial_report", {
      query: this.client.convertToTauriQuery(query),
    });
  }

  async getStatusCounts(query: TauriTypes.StockItemControllerGetListParams): Promise<{ [key: string]: number }> {
    return await this.client.sendInvoke<{ [key: string]: number }>("get_stock_item_status_counts", { query: this.client.convertToTauriQuery(query) });
  }

  async create(input: TauriTypes.CreateStockItem, by?: string) {
    return await this.client.sendInvoke<TauriTypes.StockItem>("stock_item_create", { input, by });
  }

  async update(input: TauriTypes.UpdateStockItem): Promise<TauriTypes.StockItem> {
    return await this.client.sendInvoke<TauriTypes.StockItem>("stock_item_update", { input });
  }
  async updateMultiple(ids: number[], input: TauriTypes.UpdateStockItem): Promise<TauriTypes.StockItem[]> {
    return await this.client.sendInvoke<TauriTypes.StockItem[]>("stock_item_update_multiple", { ids, input });
  }

  async delete(id: number): Promise<TauriTypes.StockItem> {
    return await this.client.sendInvoke<TauriTypes.StockItem>("stock_item_delete", { id });
  }

  async deleteMultiple(ids: number[]): Promise<number> {
    return await this.client.sendInvoke<number>("stock_item_delete_multiple", { ids });
  }

  async sell(entry: TauriTypes.SellStockItem, by?: string): Promise<TauriTypes.StockItem> {
    return await this.client.sendInvoke<TauriTypes.StockItem>("stock_item_sell", { ...entry, by });
  }

  async getById(id: number): Promise<TauriTypes.StockItemDetails> {
    return await this.client.sendInvoke<TauriTypes.StockItemDetails>("stock_item_get_by_id", { id });
  }
  exportJson = async (query: TauriTypes.StockItemControllerGetListParams): Promise<string> => {
    return await this.client.sendInvoke<string>("export_stock_item_json", {
      query: this.client.convertToTauriQuery(query),
    });
  };
}
