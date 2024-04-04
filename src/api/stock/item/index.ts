import { TauriClient } from "../..";
import { StockItemCreateDto, StockItemDto, StockItemUpdateDto } from "./types";

export class StockItemModule {
  constructor(private readonly client: TauriClient) { }

  async getAll(): Promise<StockItemDto[]> {
    return await this.client.sendInvoke<StockItemDto[]>("stock_item_get_all");
  }

  async getById(id: number): Promise<StockItemDto> {
    return await this.client.sendInvoke<StockItemDto>("stock_item_get_by_id", { id });
  }

  async create(payload: StockItemCreateDto): Promise<StockItemDto> {
    return await this.client.sendInvoke<StockItemDto>("stock_item_create", payload);
  }

  async update(id: number, payload: StockItemUpdateDto): Promise<StockItemDto> {
    return await this.client.sendInvoke<StockItemDto>("stock_item_update", { id, ...payload });
  }

  delete(id: number): Promise<boolean> {
    return this.client.sendInvoke("stock_item_delete", { id });
  }
}
