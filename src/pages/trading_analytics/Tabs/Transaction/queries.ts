import { useQuery } from "@tanstack/react-query";
import { TauriTypes } from "$types";
import api from "@api/index";

interface QueriesHooks {
  queryData: TauriTypes.TransactionControllerGetListParams;
  chartParams?: TauriTypes.TransactionControllerGetListParams;
  isActive?: boolean;
  loadFinancialReport?: boolean;
  loadChart?: boolean;
}

export const useQueries = ({
  queryData,
  chartParams,
  isActive,
  loadFinancialReport = false,
  loadChart = false,
}: QueriesHooks) => {
  const getPaginationQuery = useQuery({
    queryKey: ["get_transaction_pagination", queryData],
    queryFn: () => api.transaction.getPagination(queryData),
    retry: false,
    enabled: isActive,
  });
  const getFinancialReportQuery = useQuery({
    queryKey: ["get_transaction_financial_report", queryData],
    queryFn: () => api.transaction.getFinancialReport({ ...queryData, page: 1, limit: -1 }),
    retry: false,
    enabled: isActive && loadFinancialReport,
  });
  const getChartQuery = useQuery({
    queryKey: ["get_transaction_chart", chartParams],
    queryFn: () => api.transaction.getPagination({ ...(chartParams as TauriTypes.TransactionControllerGetListParams) }),
    retry: false,
    enabled: isActive && loadChart && !!chartParams,
  });
  const refetchQueries = () => {
    getPaginationQuery.refetch();
    getFinancialReportQuery.refetch();
    getChartQuery.refetch();
  };

  // Return the queries
  return {
    paginationQuery: getPaginationQuery,
    financialReportQuery: getFinancialReportQuery,
    chartQuery: getChartQuery,
    refetchQueries,
  };
};
