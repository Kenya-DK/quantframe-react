import { useQuery } from "@tanstack/react-query";
import { TauriTypes } from "$types";
import api from "@api/index";

interface QueriesHooks {
  queryData: TauriTypes.StockRivenControllerGetListParams;
}

export const useStockQueries = ({ queryData }: QueriesHooks) => {
  const getPaginationQuery = useQuery({
    queryKey: ["get_stock_riven_pagination", queryData],
    queryFn: () => api.stock_riven.getPagination(queryData),
    retry: false,
  });
  const getFinancialReportQuery = useQuery({
    queryKey: ["get_stock_riven_financial_report", queryData],
    queryFn: () => api.stock_riven.getFinancialReport(queryData),
    retry: false,
  });
  const getStatusCountsQuery = useQuery({
    queryKey: ["get_stock_riven_status_counts"],
    queryFn: () => api.stock_riven.getStatusCounts({ page: 1, limit: -1 }),
    retry: false,
  });
  const refetchQueries = (refetchStatus: boolean = false) => {
    getPaginationQuery.refetch();
    getFinancialReportQuery.refetch();
    if (refetchStatus) getStatusCountsQuery.refetch();
  };

  // Return the queries
  return {
    paginationQuery: getPaginationQuery,
    financialReportQuery: getFinancialReportQuery,
    statusCountsQuery: getStatusCountsQuery,
    refetchQueries,
  };
};
