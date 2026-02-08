import { useQuery } from "@tanstack/react-query";
import { TauriTypes } from "$types";
import api from "@api/index";

interface QueriesHooks {
  queryData: TauriTypes.TransactionControllerGetListParams;
  isActive?: boolean;
  loadFinancialReport?: boolean;
}

export const useQueries = ({ queryData, isActive, loadFinancialReport = false }: QueriesHooks) => {
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
