import { useQuery } from "@tanstack/react-query";
import { ResponseError, TauriTypes } from "$types";
import api from "@api/index";

interface QueriesHooks {
  queryData: TauriTypes.WFGDPRTradeControllerGetListParams;
  isActive?: boolean;
}

export interface FinancialReport extends Omit<TauriTypes.FinancialReport, "properties"> {
  properties: {
    financial_graph: FinancialGraph;
    most_purchased_items: Array<Array<number | string>>;
    most_sold_items: Array<Array<number | string>>;
    spent_purchase_credits: number;
    spent_sale_credits: number;
    spent_trade_credits: number;
    total_credits: number;
    total_trades: number;
  };
}

export interface FinancialGraph {
  labels: string[];
  values: Values;
}

export interface Values {
  total: number[];
  total_purchases: number[];
  total_sales: number[];
  total_trades: number[];
}

export const useQueries = ({ queryData }: QueriesHooks) => {
  const getPaginationQuery = useQuery({
    queryKey: [
      "get_wfgdpr_trade_pagination",
      queryData.from_date,
      queryData.to_date,
      queryData.page,
      queryData.limit,
      queryData.query,
      queryData.sort_by,
      queryData.sort_direction,
    ],
    queryFn: () => api.sendInvoke<TauriTypes.WFGDPRTradeControllerGetListData>(`wfgdpr_get_trades_pagination`, { query: queryData }),
    retry: false,
    throwOnError(error: ResponseError, query) {
      console.error("Error in query:", query.queryKey, error);
      return false;
    },
  });
  const getFinancialReportQuery = useQuery({
    queryKey: [
      "get_wfgdpr_trade_financial_report",
      queryData.from_date,
      queryData.to_date,
      queryData.page,
      queryData.limit,
      queryData.query,
      queryData.sort_by,
      queryData.sort_direction,
    ],
    queryFn: () => api.sendInvoke<FinancialReport>(`wfgdpr_get_trades_financial_report`, { query: queryData }),
    retry: false,
    throwOnError(error: ResponseError, query) {
      console.error("Error in query:", query.queryKey, error);
      return false;
    },
  });
  const refetchQueries = () => {
    getPaginationQuery.refetch();
    getFinancialReportQuery.refetch();
  };

  // Return the queries
  return {
    paginationQuery: getPaginationQuery,
    financialReportQuery: getFinancialReportQuery,
    refetchQueries,
  };
};
