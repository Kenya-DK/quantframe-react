import { TauriClient } from "../..";
import { CreateStockItem, StockItem, SellStockItem, UpdateStockItem, SellByWfmOrder } from "@api/types";

export class StockItemModule {
  constructor(private readonly client: TauriClient) { }

  async reload(): Promise<void> {
    await this.client.sendInvoke<void>('stock_item_reload');
  }


  async getAll(): Promise<StockItem[]> {
    const [err, stockItems] = await this.client.sendInvoke<StockItem[]>('stock_item_get_all');
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
    if (err)
      throw err;
    return stockItem;
  }

  async updateBulk(ids: number[], entry: UpdateStockItem): Promise<number> {
    const [err, stockItem] = await this.client.sendInvoke<number>('stock_item_update_bulk', { ...entry, ids });
    if (err)
      throw err;
    return stockItem;
  }

  async delete(id: number): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>('stock_item_delete', { id });
    if (err)
      throw err;
    return res;
  }

  async deleteBulk(ids: number[]): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>('stock_item_delete_bulk', { ids });
    if (err)
      throw err;
    return res;
  }

  async sell(entry: SellStockItem): Promise<StockItem> {
    const [err, stockItem] = await this.client.sendInvoke<StockItem>('stock_item_sell', entry);
    if (err)
      throw err;
    return stockItem;
  }

  async sellByWfmOrder(entry: SellByWfmOrder): Promise<StockItem> {
    const [err, stockItem] = await this.client.sendInvoke<StockItem>('stock_item_sell_by_wfm_order', entry);
    if (err)
      throw err;
    return stockItem;
  }

}
