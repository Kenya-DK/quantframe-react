import { TauriClient } from "../..";
import { StockRivenCreateDto, StockRivenDto, StockRivenUpdateDto } from "./types";

export class StockRivenModule {
  constructor(private readonly client: TauriClient) { }

  async getAll(): Promise<StockRivenDto[]> {
    return await this.client.sendInvoke<StockRivenDto[]>("stock_riven_get_all");
  }

  async getById(id: number): Promise<StockRivenDto> {
    return await this.client.sendInvoke<StockRivenDto>("stock_riven_get_by_id", { id });
  }

  async create(payload: StockRivenCreateDto): Promise<StockRivenDto> {
    return await this.client.sendInvoke<StockRivenDto>("stock_riven_create", payload);
  }

  async update(id: number, payload: StockRivenUpdateDto): Promise<StockRivenDto> {
    return await this.client.sendInvoke<StockRivenDto>("stock_riven_update", { id, ...payload });
  }

  delete(id: number): Promise<boolean> {
    return this.client.sendInvoke("stock_riven_delete", { id });
  }
}
