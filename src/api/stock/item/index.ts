import { TauriClient } from "../..";
import { CreateStockItem, StockItem, SellStockItem, UpdateStockItem } from "@api/types";

export class StockItemModule {
  constructor(private readonly client: TauriClient) { }

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
    console.log(entry);
    const [, stockItem] = await this.client.sendInvoke<StockItem>('stock_item_update', entry);
    if (!stockItem)
      throw new Error("Failed to create stock item");
    return stockItem;
  }


  async delete(id: number): Promise<void> {
    await this.client.sendInvoke<void>('stock_item_delete', { id });
  }

  async sell(entry: SellStockItem): Promise<StockItem> {
    const [, stockItem] = await this.client.sendInvoke<StockItem>('stock_item_sell', entry);
    if (!stockItem)
      throw new Error("Failed to create stock item");
    return stockItem;
  }
}
