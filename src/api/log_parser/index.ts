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

  async load(path: string): Promise<{ [key: string]: number[] }> {
    return await this.client.sendInvoke("wfgdpr_load", { filePath: path });
  }

  async getState(): Promise<{ was_initialized: boolean; trade_years: string[] }> {
    return await this.client.sendInvoke<{ was_initialized: boolean; trade_years: string[] }>("wfgdpr_get_state");
  }

  async getAccounts(): Promise<TauriTypes.WFGDPRAccount[]> {
    return await this.client.sendInvoke<TauriTypes.WFGDPRAccount[]>("wfgdpr_get_accounts");
  }
}
