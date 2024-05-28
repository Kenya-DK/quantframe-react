import { TauriClient } from "../..";
import { CreateStockRiven, SellStockRiven, StockRiven, UpdateStockRiven } from "@api/types";

export class StockRivenModule {
  constructor(private readonly client: TauriClient) { }

  async reload(): Promise<void> {
    await this.client.sendInvoke<void>('stock_riven_reload');
  }


  async getAll(): Promise<StockRiven[]> {
    const [err, stockItems] = await this.client.sendInvoke<StockRiven[]>('stock_riven_get_all');
    if (err)
      throw err;
    return stockItems;
  }

  async create(entry: CreateStockRiven): Promise<StockRiven> {
    const [err, stockItem] = await this.client.sendInvoke<StockRiven>('stock_riven_create', entry);
    if (err)
      throw err;
    return stockItem;
  }

  async update(entry: UpdateStockRiven): Promise<StockRiven> {
    const [err, stockItem] = await this.client.sendInvoke<StockRiven>('stock_riven_update', entry);
    if (err)
      throw err;
    return stockItem;
  }

  async updateBulk(ids: number[], entry: UpdateStockRiven): Promise<number> {
    const [err, stockItem] = await this.client.sendInvoke<number>('stock_riven_update_bulk', { ...entry, ids });
    if (err)
      throw err;
    return stockItem;
  }


  async delete(id: number): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>('stock_riven_delete', { id });
    if (err)
      throw err;
    return res;
  }

  async deleteBulk(ids: number[]): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>('stock_riven_delete_bulk', { ids });
    if (err)
      throw err;
    return res;
  }

  async sell(entry: SellStockRiven): Promise<StockRiven> {
    const [err, stockItem] = await this.client.sendInvoke<StockRiven>('stock_riven_sell', entry);
    if (err)
      throw err;
    return stockItem;
  }
}
