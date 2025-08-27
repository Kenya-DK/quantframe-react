import { useQuery } from "@tanstack/react-query";
import { TauriTypes } from "$types";
import api from "@api/index";

interface QueriesHooks {
  queryData: TauriTypes.WishListControllerGetListParams;
}

export const useWishListQueries = ({ queryData }: QueriesHooks) => {
  const getPaginationQuery = useQuery({
    queryKey: ["get_wish_list_pagination", queryData],
    queryFn: () => api.wish_list.getPagination(queryData),
    retry: false,
  });
  const getFinancialReportQuery = useQuery({
    queryKey: ["get_wish_list_financial_report", queryData],
    queryFn: () => api.wish_list.getFinancialReport(queryData),
    retry: false,
  });
  const getStatusCountsQuery = useQuery({
    queryKey: ["get_wish_list_status_counts"],
    queryFn: () => api.wish_list.getStatusCounts({ page: 1, limit: -1 }),
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
