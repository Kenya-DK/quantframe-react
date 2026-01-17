import { TauriClient } from "..";
import { TauriTypes } from "$types";

export class TransactionModule {
  constructor(private readonly client: TauriClient) {}

  async getAll(query: TauriTypes.TransactionControllerGetListParams): Promise<TauriTypes.TransactionControllerGetListData> {
    const [err, stockItems] = await this.client.sendInvoke<TauriTypes.TransactionControllerGetListData>("transaction_get_all", {
      query: this.client.convertToTauriQuery(query),
    });
    if (err) throw err;
    return stockItems;
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
