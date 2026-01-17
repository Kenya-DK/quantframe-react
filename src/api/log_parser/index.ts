import { TauriClient } from "..";
import { TauriTypes } from "$types";
import dayjs from "dayjs";

export interface FinancialReport extends Omit<TauriTypes.FinancialReport, "properties"> {
  properties: {
    graph: FinancialGraph;
    most_purchased_items: Array<Array<number | string>>;
    most_sold_items: Array<Array<number | string>>;
    categories: TauriTypes.FinancialCategoryReport[];
    total_credits: number;
    total_trades: number;
    year: TauriTypes.FinancialReport;
  };
}

export interface FinancialGraph {
  labels: string[];
  values: Values;
}

export interface Values {
  total_trades: number[];
  total_sales: number[];
  total_purchase: number[];
}

export class LogParserModule {
  constructor(private readonly client: TauriClient) {}
  async getState(): Promise<{ was_initialized: boolean; trade_years: string[] }> {
    return await this.client.sendInvoke<{ was_initialized: boolean; trade_years: string[] }>("wfgdpr_get_state");
  }

  async load(path: string): Promise<{ [key: string]: number[] }> {
    return await this.client.sendInvoke("wfgdpr_load", { filePath: path });
  }
  async getTradePagination(query: TauriTypes.WFGDPRTradeControllerGetListParams): Promise<TauriTypes.WFGDPRTradeControllerGetListData> {
    if (query.from_date) query.from_date = dayjs(query.from_date).utc().toISOString();
    if (query.to_date) query.to_date = dayjs(query.to_date).utc().toISOString();
    return await this.client.sendInvoke<TauriTypes.WFGDPRTradeControllerGetListData>("wfgdpr_get_trades_pagination", {
      query: this.client.convertToTauriQuery(query),
    });
  }
  async getTradeFinancialReport(query: TauriTypes.WFGDPRTradeControllerGetListParams): Promise<FinancialReport> {
    if (query.from_date) query.from_date = dayjs(query.from_date).utc().toISOString();
    if (query.to_date) query.to_date = dayjs(query.to_date).utc().toISOString();
    return await this.client.sendInvoke<FinancialReport>("wfgdpr_get_trades_financial_report", {
      query: this.client.convertToTauriQuery(query),
    });
  }

  async getPurchasePagination(query: TauriTypes.WFGDPRPurchaseControllerGetListParams): Promise<TauriTypes.WFGDPRPurchaseControllerGetListData> {
    return await this.client.sendInvoke<TauriTypes.WFGDPRPurchaseControllerGetListData>("wfgdpr_get_purchases_pagination", {
      query: this.client.convertToTauriQuery(query),
    });
  }

  async getLoginPagination(query: TauriTypes.WFGDPRLoginControllerGetListParams): Promise<TauriTypes.WFGDPRLoginControllerGetListData> {
    return await this.client.sendInvoke<TauriTypes.WFGDPRLoginControllerGetListData>("wfgdpr_get_logins_pagination", {
      query: this.client.convertToTauriQuery(query),
    });
  }

  async getTransactionPagination(
    query: TauriTypes.WFGDPRTransactionControllerGetListParams
  ): Promise<TauriTypes.WFGDPRTransactionControllerGetListData> {
    return await this.client.sendInvoke<TauriTypes.WFGDPRTransactionControllerGetListData>("wfgdpr_get_transactions_pagination", {
      query: this.client.convertToTauriQuery(query),
    });
  }
}
