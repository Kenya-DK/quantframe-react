import { TauriClient } from "../..";
import { TauriTypes } from "$types";

export class StockRivenModule {
  constructor(private readonly client: TauriClient) {}

  async getAll(query: TauriTypes.StockRivenControllerGetListParams): Promise<TauriTypes.StockRivenControllerGetListData> {
    const [err, stockItems] = await this.client.sendInvoke<TauriTypes.StockRivenControllerGetListData>("get_stock_rivens", {
      query: this.client.convertToTauriQuery(query),
    });
    if (err) throw err;
    return stockItems;
  }

  async create(input: TauriTypes.CreateStockRiven): Promise<TauriTypes.StockRiven> {
    const [err, stockItem] = await this.client.sendInvoke<TauriTypes.StockRiven>("stock_riven_create", input);
    if (err) throw err;
    return stockItem;
  }

  async update(entry: TauriTypes.UpdateStockRiven): Promise<TauriTypes.StockRiven> {
    const [err, stockItem] = await this.client.sendInvoke<TauriTypes.StockRiven>("stock_riven_update", entry);
    if (err) throw err;
    return stockItem;
  }

  async updateBulk(ids: number[], entry: TauriTypes.UpdateStockRiven): Promise<number> {
    const [err, stockItem] = await this.client.sendInvoke<number>("stock_riven_update_bulk", { ...entry, ids });
    if (err) throw err;
    return stockItem;
  }

  async delete(id: number): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>("stock_riven_delete", { id });
    if (err) throw err;
    return res;
  }

  async deleteBulk(ids: number[]): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>("stock_riven_delete_bulk", { ids });
    if (err) throw err;
    return res;
  }

  async sell(entry: TauriTypes.SellStockRiven): Promise<TauriTypes.StockRiven> {
    const [err, stockItem] = await this.client.sendInvoke<TauriTypes.StockRiven>("stock_riven_sell", entry);
    if (err) throw err;
    return stockItem;
  }
}
