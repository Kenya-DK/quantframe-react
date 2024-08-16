import { TauriClient } from "../..";
import { CreateStockItem, StockItem, SellStockItem, UpdateStockItem, SellByWfmOrder } from "@api/types";

export class StockItemModule {
  constructor(private readonly client: TauriClient) { }

  async reload(): Promise<void> {
    const [err] = await this.client.sendInvoke<void>('stock_item_reload');
    await this.client.analytics.sendMetric('StockItem_Reload', err ? 'failed' : 'success');
  }


  async getAll(): Promise<StockItem[]> {
    const [err, stockItems] = await this.client.sendInvoke<StockItem[]>('stock_item_get_all');
    await this.client.analytics.sendMetric('StockItem_GetAll', err ? 'failed' : 'success');
    if (err)
      throw err;
    return stockItems;
  }

  async create(entry: CreateStockItem): Promise<StockItem> {
    const [err, stockItem] = await this.client.sendInvoke<StockItem>('stock_item_create', entry);
    if (err)
      throw err;
    return stockItem;
  }

  async update(entry: UpdateStockItem): Promise<StockItem> {
    const [err, stockItem] = await this.client.sendInvoke<StockItem>('stock_item_update', entry);
    await this.client.analytics.sendMetric('StockItem_Update', err ? 'failed' : 'success');
    if (err)
      throw err;
    return stockItem;
  }

  async updateBulk(ids: number[], entry: UpdateStockItem): Promise<number> {
    const [err, stockItem] = await this.client.sendInvoke<number>('stock_item_update_bulk', { ...entry, ids });
    await this.client.analytics.sendMetric('StockItem_UpdateBulk', err ? 'failed' : 'success');
    if (err)
      throw err;
    return stockItem;
  }

  async delete(id: number): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>('stock_item_delete', { id });
    await this.client.analytics.sendMetric('StockItem_Delete', err ? 'failed' : 'success');
    if (err)
      throw err;
    return res;
  }

  async deleteBulk(ids: number[]): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>('stock_item_delete_bulk', { ids });
    await this.client.analytics.sendMetric('StockItem_DeleteBulk', err ? 'failed' : 'success');
    if (err)
      throw err;
    return res;
  }

  async sell(entry: SellStockItem): Promise<StockItem> {
    const [err, stockItem] = await this.client.sendInvoke<StockItem>('stock_item_sell', entry);
    await this.client.analytics.sendMetric('StockItem_Sell', err ? 'failed' : 'success');
    if (err)
      throw err;
    return stockItem;
  }

  async sellByWfmOrder(entry: SellByWfmOrder): Promise<StockItem> {
    const [err, stockItem] = await this.client.sendInvoke<StockItem>('stock_item_sell_by_wfm_order', entry);
    await this.client.analytics.sendMetric('StockItem_SellBy_WFMOrder', err ? 'failed' : 'success');
    if (err)
      throw err;
    return stockItem;
  }

}
