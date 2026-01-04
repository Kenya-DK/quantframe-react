import { TauriClient } from "..";
import { TauriTypes } from "$types";

export interface FinancialReport extends Omit<TauriTypes.FinancialReport, "properties"> {
  properties: {
    graph: FinancialGraph;
    most_purchased_items: Array<Array<number | string>>;
    most_sold_items: Array<Array<number | string>>;
    categories: TauriTypes.FinancialCategoryReport[];
    total_credits: number;
    total_trades: number;
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
  async getState(): Promise<{ was_initialized: boolean }> {
    return await this.client.sendInvoke<{ was_initialized: boolean }>("wfgdpr_get_state");
  }

  async load(path: string): Promise<{ [key: string]: number[] }> {
    return await this.client.sendInvoke("wfgdpr_load", { filePath: path });
  }
  async getTradePagination(query: TauriTypes.WFGDPRLoginControllerGetListParams): Promise<TauriTypes.WFGDPRLoginControllerGetListData> {
    return await this.client.sendInvoke<TauriTypes.WFGDPRLoginControllerGetListData>("wfgdpr_get_trades_pagination", {
      query: this.client.convertToTauriQuery(query),
    });
  }
  async getTradeFinancialReport(query: TauriTypes.WFGDPRLoginControllerGetListParams): Promise<FinancialReport> {
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
}
