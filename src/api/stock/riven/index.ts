import { TauriClient } from "../..";
import { TauriTypes } from "$types";

export class StockRivenModule {
  constructor(private readonly client: TauriClient) {}

  async reload(): Promise<void> {
    await this.client.sendInvoke<void>("stock_riven_reload");
  }

  async getAll(): Promise<TauriTypes.StockRiven[]> {
    const [err, stockItems] = await this.client.sendInvoke<TauriTypes.StockRiven[]>("stock_riven_get_all");
    if (err) throw err;
    return stockItems;
  }

  async create(riven_entry: TauriTypes.CreateStockRiven): Promise<TauriTypes.StockRiven> {
    const [err, stockItem] = await this.client.sendInvoke<TauriTypes.StockRiven>("stock_riven_create", { rivenEntry: riven_entry });
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
