import { useQuery } from "@tanstack/react-query";
import { TauriTypes } from "$types";
import api from "@api/index";

interface QueriesHooks {
  queryData: TauriTypes.WishListControllerGetListParams;
}

export const useStockQueries = ({ queryData }: QueriesHooks) => {
  const getPaginationQuery = useQuery({
    queryKey: ["get_wfm_orders_pagination", queryData],
    queryFn: () => api.order.getPagination(queryData),
    retry: false,
  });
  const getStatusCountsQuery = useQuery({
    queryKey: ["get_wfm_orders_status_counts"],
    queryFn: () => api.order.getStatusCounts({ page: 1, limit: -1 }),
    retry: false,
  });
  const refetchQueries = (refetchStatus: boolean = false) => {
    getPaginationQuery.refetch();
    if (refetchStatus) getStatusCountsQuery.refetch();
  };

  // Return the queries
  return {
    paginationQuery: getPaginationQuery,
    statusCountsQuery: getStatusCountsQuery,
    refetchQueries,
  };
};
