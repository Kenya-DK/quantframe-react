import { TauriClient } from "../..";
import { CreateStockRiven, SellStockRiven, StockRiven, UpdateStockRiven } from "@api/types";

export class StockRivenModule {
  constructor(private readonly client: TauriClient) { }

  async reload(): Promise<void> {
    const [err] = await this.client.sendInvoke<void>('stock_riven_reload');
    await this.client.analytics.sendMetric('StockRiven_Reload', err ? 'failed' : 'success');
  }


  async getAll(): Promise<StockRiven[]> {
    const [err, stockItems] = await this.client.sendInvoke<StockRiven[]>('stock_riven_get_all');
    await this.client.analytics.sendMetric('StockRiven_GetAll', err ? 'failed' : 'success');
    if (err)
      throw err;
    return stockItems;
  }

  async create(riven_entry: CreateStockRiven): Promise<StockRiven> {
    const [err, stockItem] = await this.client.sendInvoke<StockRiven>('stock_riven_create', { rivenEntry: riven_entry });
    if (err)
      throw err;
    return stockItem;
  }

  async update(entry: UpdateStockRiven): Promise<StockRiven> {
    const [err, stockItem] = await this.client.sendInvoke<StockRiven>('stock_riven_update', entry);
    await this.client.analytics.sendMetric('StockRiven_Update', err ? 'failed' : 'success');
    if (err)
      throw err;
    return stockItem;
  }

  async updateBulk(ids: number[], entry: UpdateStockRiven): Promise<number> {
    const [err, stockItem] = await this.client.sendInvoke<number>('stock_riven_update_bulk', { ...entry, ids });
    await this.client.analytics.sendMetric('StockRiven_UpdateBulk', err ? 'failed' : 'success');
    if (err)
      throw err;
    return stockItem;
  }


  async delete(id: number): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>('stock_riven_delete', { id });
    await this.client.analytics.sendMetric('StockRiven_Delete', err ? 'failed' : 'success');
    if (err)
      throw err;
    return res;
  }

  async deleteBulk(ids: number[]): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>('stock_riven_delete_bulk', { ids });
    await this.client.analytics.sendMetric('StockRiven_DeleteBulk', err ? 'failed' : 'success');
    if (err)
      throw err;
    return res;
  }

  async sell(entry: SellStockRiven): Promise<StockRiven> {
    const [err, stockItem] = await this.client.sendInvoke<StockRiven>('stock_riven_sell', entry);
    await this.client.analytics.sendMetric('StockRiven_Sell', err ? 'failed' : 'success');
    if (err)
      throw err;
    return stockItem;
  }
}
