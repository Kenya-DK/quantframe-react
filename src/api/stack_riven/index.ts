import { TauriClient } from "..";
import { TauriTypes } from "../../types";
export class StockRivenModule {
  constructor(private readonly client: TauriClient) {}

  async getPagination(query: TauriTypes.StockRivenControllerGetListParams): Promise<TauriTypes.StockRivenControllerGetListData> {
    return await this.client.sendInvoke<TauriTypes.StockRivenControllerGetListData>("get_stock_riven_pagination", {
      query: this.client.convertToTauriQuery(query),
    });
  }

  async getFinancialReport(query: TauriTypes.StockRivenControllerGetListParams): Promise<TauriTypes.FinancialReport> {
    return await this.client.sendInvoke<TauriTypes.FinancialReport>("get_stock_riven_financial_report", {
      query: this.client.convertToTauriQuery(query),
    });
  }

  async getStatusCounts(query: TauriTypes.StockRivenControllerGetListParams): Promise<{ [key: string]: number }> {
    return await this.client.sendInvoke<{ [key: string]: number }>("get_stock_riven_status_counts", {
      query: this.client.convertToTauriQuery(query),
    });
  }

  async create(input: TauriTypes.CreateStockRiven) {
    return await this.client.sendInvoke<TauriTypes.StockRiven>("stock_riven_create", { input });
  }

  async update(input: TauriTypes.UpdateStockRiven): Promise<TauriTypes.StockRiven> {
    return await this.client.sendInvoke<TauriTypes.StockRiven>("stock_riven_update", { input });
  }

  async delete(id: number): Promise<TauriTypes.StockRiven> {
    return await this.client.sendInvoke<TauriTypes.StockRiven>("stock_riven_delete", { id });
  }

  async sell(entry: TauriTypes.SellStockRiven): Promise<TauriTypes.StockRiven> {
    return await this.client.sendInvoke<TauriTypes.StockRiven>("stock_riven_sell", { ...entry });
  }
  async getById(id: number): Promise<TauriTypes.StockRivenDetails> {
    return await this.client.sendInvoke<TauriTypes.StockRivenDetails>("stock_riven_get_by_id", { id });
  }
  exportJson = async (query: TauriTypes.StockRivenControllerGetListParams): Promise<string> => {
    return await this.client.sendInvoke<string>("export_stock_riven_json", {
      query: this.client.convertToTauriQuery(query),
    });
  };
}
