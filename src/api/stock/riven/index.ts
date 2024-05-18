import { TauriClient } from "../..";
import { CreateStockRiven, SellStockRiven, StockRiven, UpdateStockRiven } from "@api/types";

export class StockRivenModule {
  constructor(private readonly client: TauriClient) { }

  async getAll(): Promise<StockRiven[]> {
    const [, stockItems] = await this.client.sendInvoke<StockRiven[]>('stock_riven_get_all');
    if (!stockItems)
      return [];
    return stockItems;
  }

  async create(entry: CreateStockRiven): Promise<StockRiven> {
    const [, stockItem] = await this.client.sendInvoke<StockRiven>('stock_riven_create', entry);
    if (!stockItem)
      throw new Error("Failed to create stock item");
    return stockItem;
  }

  async update(entry: UpdateStockRiven): Promise<StockRiven> {
    const [, stockItem] = await this.client.sendInvoke<StockRiven>('stock_riven_update', entry);
    if (!stockItem)
      throw new Error("Failed to create stock item");
    return stockItem;
  }

  async updateBulk(ids: number[], entry: UpdateStockRiven): Promise<number> {
    const [, stockItem] = await this.client.sendInvoke<number>('stock_riven_update_bulk', { ...entry, ids });
    if (!stockItem)
      throw new Error("Failed to create stock item");
    return stockItem;
  }


  async delete(id: number): Promise<void> {
    await this.client.sendInvoke<void>('stock_riven_delete', { id });
  }

  async deleteBulk(ids: number[]): Promise<void> {
    await this.client.sendInvoke<void>('stock_riven_delete_bulk', { ids });
  }

  async sell(entry: SellStockRiven): Promise<StockRiven> {
    const [, stockItem] = await this.client.sendInvoke<StockRiven>('stock_riven_sell', entry);
    if (!stockItem)
      throw new Error("Failed to create stock item");
    return stockItem;
  }
}
