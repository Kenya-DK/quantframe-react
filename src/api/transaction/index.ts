import { TauriClient } from "..";
import { TauriTypes } from "$types";

export class TransactionModule {
  constructor(private readonly client: TauriClient) {}

  async reload(): Promise<void> {
    const [err] = await this.client.sendInvoke<void>("transaction_reload");
    if (err) throw err;
  }

  async getAll(): Promise<TauriTypes.TransactionDto[]> {
    const [err, res] = await this.client.sendInvoke<TauriTypes.TransactionDto[]>("transaction_get_all");
    if (err) throw err;
    return res;
  }

  async update(entry: TauriTypes.UpdateTransactionDto): Promise<TauriTypes.TransactionDto> {
    const [err, stockItem] = await this.client.sendInvoke<TauriTypes.TransactionDto>("transaction_update", entry);
    if (err) throw err;
    return stockItem;
  }

  async delete(id: number): Promise<void> {
    const [err, stockItem] = await this.client.sendInvoke<void>("transaction_delete", { id });
    if (err) throw err;
    return stockItem;
  }
}
