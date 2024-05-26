import { TauriClient } from "../..";
import { CreateStockItem, StockItem, SellStockItem, UpdateStockItem } from "@api/types";

export class StockItemModule {
  constructor(private readonly client: TauriClient) { }

  async reload(): Promise<void> {
    await this.client.sendInvoke<void>('stock_item_reload');
  }


  async getAll(): Promise<StockItem[]> {
    const [, stockItems] = await this.client.sendInvoke<StockItem[]>('stock_item_get_all');
    if (!stockItems)
      return [];
    return stockItems;
  }

  async create(entry: CreateStockItem): Promise<StockItem> {
    const [, stockItem] = await this.client.sendInvoke<StockItem>('stock_item_create', entry);
    if (!stockItem)
      throw new Error("Failed to create stock item");
    return stockItem;
  }

  async update(entry: UpdateStockItem): Promise<StockItem> {
    const [, stockItem] = await this.client.sendInvoke<StockItem>('stock_item_update', entry);
    if (!stockItem)
      throw new Error("Failed to update stock item");
    return stockItem;
  }

  async updateBulk(ids: number[], entry: UpdateStockItem): Promise<number> {
    const [, stockItem] = await this.client.sendInvoke<number>('stock_item_update_bulk', { ...entry, ids });
    if (!stockItem)
      throw new Error("Failed to update stock item");
    return stockItem;
  }

  async delete(id: number): Promise<void> {
    await this.client.sendInvoke<void>('stock_item_delete', { id });
  }

  async deleteBulk(ids: number[]): Promise<void> {
    await this.client.sendInvoke<void>('stock_item_delete_bulk', { ids });
  }

  async sell(entry: SellStockItem): Promise<StockItem> {
    const [, stockItem] = await this.client.sendInvoke<StockItem>('stock_item_sell', entry);
    if (!stockItem)
      throw new Error("Failed to create stock item");
    return stockItem;
  }



}
