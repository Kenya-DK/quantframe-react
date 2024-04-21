import { TauriClient } from "..";
import { ErrOrResult, TransactionDto } from "../types";

export class TransactionModule {
  constructor(private readonly client: TauriClient) { }

  async getAll(): Promise<Promise<ErrOrResult<TransactionDto[]>>> {
    return this.client.sendInvoke("transaction_get_all");
  }


}
