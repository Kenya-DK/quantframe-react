import { TauriClient } from "..";
import { ErrOrResult, TransactionDto, UpdateTransactionDto } from "../types";

export class TransactionModule {
  constructor(private readonly client: TauriClient) { }

  async reload(): Promise<void> {
    const [err] = await this.client.sendInvoke<void>("transaction_reload");
    if (err)
      throw err;
  }

  async getAll(): Promise<Promise<ErrOrResult<TransactionDto[]>>> {
    return this.client.sendInvoke("transaction_get_all");
  }

  async update(entry: UpdateTransactionDto): Promise<TransactionDto> {
    const [err, stockItem] = await this.client.sendInvoke<TransactionDto>("transaction_update", entry);
    if (err)
      throw err;
    return stockItem;
  }

  async delete(id: number): Promise<void> {
    const [err, stockItem] = await this.client.sendInvoke<void>("transaction_delete", { id });
    if (err)
      throw err;
    return stockItem;
  }
}
