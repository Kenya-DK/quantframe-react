import { TauriClient } from "..";
import { TransactionDto, UpdateTransactionDto } from "../types";

export class TransactionModule {
  constructor(private readonly client: TauriClient) { }

  async reload(): Promise<void> {
    const [err] = await this.client.sendInvoke<void>("transaction_reload");
    await this.client.analytics.sendMetric("Transaction_Reload", err ? "failed" : "success");
    if (err)
      throw err;
  }

  async getAll(): Promise<TransactionDto[]> {
    const [err, res] = await this.client.sendInvoke<TransactionDto[]>("transaction_get_all");
    await this.client.analytics.sendMetric("Transaction_GetAll", err ? "failed" : "success");
    if (err)
      throw err;
    return res;
  }

  async update(entry: UpdateTransactionDto): Promise<TransactionDto> {
    const [err, stockItem] = await this.client.sendInvoke<TransactionDto>("transaction_update", entry);
    await this.client.analytics.sendMetric("Transaction_Update", err ? "failed" : "success");
    if (err)
      throw err;
    return stockItem;
  }

  async delete(id: number): Promise<void> {
    const [err, stockItem] = await this.client.sendInvoke<void>("transaction_delete", { id });
    await this.client.analytics.sendMetric("Transaction_Delete", err ? "failed" : "success");
    if (err)
      throw err;
    return stockItem;
  }
}
