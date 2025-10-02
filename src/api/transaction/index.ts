import { TauriClient } from "..";
import { TauriTypes } from "../../types";
export class TransactionModule {
  constructor(private readonly client: TauriClient) {}

  async getPagination(query: TauriTypes.TransactionControllerGetListParams): Promise<TauriTypes.TransactionControllerGetListData> {
    return await this.client.sendInvoke<TauriTypes.TransactionControllerGetListData>("get_transaction_pagination", {
      query: this.client.convertToTauriQuery(query),
    });
  }

  async getFinancialReport(query: TauriTypes.TransactionControllerGetListParams): Promise<TauriTypes.FinancialReport> {
    return await this.client.sendInvoke<TauriTypes.FinancialReport>("get_transaction_financial_report", {
      query: this.client.convertToTauriQuery(query),
    });
  }

  async update(input: TauriTypes.UpdateTransaction): Promise<TauriTypes.TransactionDto> {
    return await this.client.sendInvoke<TauriTypes.TransactionDto>("transaction_update", { input });
  }

  async delete(id: number): Promise<TauriTypes.TransactionDto> {
    return await this.client.sendInvoke<TauriTypes.TransactionDto>("transaction_delete", { id });
  }
  deleteBulk = async (ids: number[]): Promise<{ deleted_count: number }> => {
    return await this.client.sendInvoke<{ deleted_count: number }>("transaction_delete_bulk", { ids });
  };
  exportJson = async (query: TauriTypes.TransactionControllerGetListParams): Promise<string> => {
    return await this.client.sendInvoke<string>("export_transaction_json", {
      query: this.client.convertToTauriQuery(query),
    });
  };
}
