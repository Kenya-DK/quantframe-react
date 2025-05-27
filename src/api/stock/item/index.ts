import { TauriClient } from "../..";
import { TauriTypes } from "$types";

const ConvertToTauriQuery = (query: TauriTypes.StockItemControllerGetListParams): any => {
  let queryParams: any = { ...query };
  queryParams.pagination = { page: query.page, limit: query.limit };
  return queryParams;
};

export class StockItemModule {
  constructor(private readonly client: TauriClient) {}

  async reload(): Promise<void> {
    const [] = await this.client.sendInvoke<void>("stock_item_reload");
  }

  async getAll(query: TauriTypes.StockItemControllerGetListParams): Promise<TauriTypes.StockItemControllerGetListData> {
    const [err, stockItems] = await this.client.sendInvoke<TauriTypes.StockItemControllerGetListData>("get_stock_items", {
      query: ConvertToTauriQuery(query),
    });
    if (err) throw err;
    return stockItems;
  }

  async create(entry: TauriTypes.CreateStockItem): Promise<TauriTypes.StockItem> {
    const [err, stockItem] = await this.client.sendInvoke<TauriTypes.StockItem>("stock_item_create", entry);
    if (err) throw err;
    return stockItem;
  }

  async update(entry: TauriTypes.UpdateStockItem): Promise<TauriTypes.StockItem> {
    const [err, stockItem] = await this.client.sendInvoke<TauriTypes.StockItem>("stock_item_update", entry);
    if (err) throw err;
    return stockItem;
  }

  async updateBulk(ids: number[], entry: TauriTypes.UpdateStockItem): Promise<number> {
    const [err, stockItem] = await this.client.sendInvoke<number>("stock_item_update_bulk", { ...entry, ids });
    if (err) throw err;
    return stockItem;
  }

  async delete(id: number): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>("stock_item_delete", { id });
    if (err) throw err;
    return res;
  }

  async deleteBulk(ids: number[]): Promise<void> {
    const [err, res] = await this.client.sendInvoke<void>("stock_item_delete_bulk", { ids });
    if (err) throw err;
    return res;
  }

  async sell(entry: TauriTypes.SellStockItem): Promise<TauriTypes.StockItem> {
    const [err, stockItem] = await this.client.sendInvoke<TauriTypes.StockItem>("stock_item_sell", entry);
    if (err) throw err;
    return stockItem;
  }
}
