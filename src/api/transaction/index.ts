import { TauriClient } from "..";
import { TauriTypes } from "../../types";
import utc from "dayjs/plugin/utc";
import dayjs from "dayjs";
dayjs.extend(utc);
export class TransactionModule {
  constructor(private readonly client: TauriClient) {}

  async getPagination(query: TauriTypes.TransactionControllerGetListParams): Promise<TauriTypes.TransactionControllerGetListData> {
    if (query.from_date) query.from_date = dayjs(query.from_date).utc().toISOString();
    if (query.to_date) query.to_date = dayjs(query.to_date).utc().toISOString();
    return await this.client.sendInvoke<TauriTypes.TransactionControllerGetListData>("get_transaction_pagination", {
      query: this.client.convertToTauriQuery(query),
    });
  }

  async getFinancialReport(query: TauriTypes.TransactionControllerGetListParams): Promise<TauriTypes.FinancialReport> {
    if (query.from_date) query.from_date = dayjs(query.from_date).utc().toISOString();
    if (query.to_date) query.to_date = dayjs(query.to_date).utc().toISOString();
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
