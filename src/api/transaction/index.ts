import { TauriClient } from "..";
import { TransactionDto, TransactionUpdateDto } from "./types";

export class TransactionModule {
  constructor(private readonly client: TauriClient) { }

  async getTransactions(): Promise<TransactionDto[]> {
    return this.client.sendInvoke("tra_get_all");
  }

  async getTransaction(id: number): Promise<TransactionDto> {
    return this.client.sendInvoke(`tra_get_by_id`, { id });
  }

  async updateTransaction(id: number, payload: TransactionUpdateDto) {
    return this.client.sendInvoke(`tra_update_by_id`, { id, ...payload });
  }

  async deleteTransaction(id: number): Promise<boolean> {
    return this.client.sendInvoke(`tra_update_by_id`, { id });
  }
}
