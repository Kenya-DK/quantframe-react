import { useQuery } from "@tanstack/react-query";
import { TauriTypes } from "$types";
import api from "@api/index";

interface QueriesHooks {
  queryData: TauriTypes.StockItemControllerGetListParams;
  isActive?: boolean;
}

export const useStockQueries = ({ queryData, isActive }: QueriesHooks) => {
  const getPaginationQuery = useQuery({
    queryKey: ["get_stock_item_pagination", queryData],
    queryFn: () => api.stock_item.getPagination(queryData),
    retry: false,
    enabled: isActive,
  });
  const getFinancialReportQuery = useQuery({
    queryKey: ["get_stock_item_financial_report", queryData],
    queryFn: () => api.stock_item.getFinancialReport(queryData),
    retry: false,
  });
  const getStatusCountsQuery = useQuery({
    queryKey: ["get_stock_item_status_counts"],
    queryFn: () => api.stock_item.getStatusCounts({ page: 1, limit: -1 }),
    retry: false,
    enabled: isActive,
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
