import { WFMarketTypes } from "$types";
import api from "@api/index";
import { useQuery } from "@tanstack/react-query";

interface QueriesHooks {
  queryData: WFMarketTypes.WfmOrderControllerGetListParams;
  isActive?: boolean;
}

export const useOrderQueries = ({ queryData, isActive }: QueriesHooks) => {
  const getPaginationQuery = useQuery({
    queryKey: ["get_wfm_orders_pagination", queryData],
    queryFn: () => api.order.getPagination(queryData),
    retry: false,
    enabled: isActive,
  });
  const getStatusCountsQuery = useQuery({
    queryKey: ["get_wfm_orders_status_counts"],
    queryFn: () => api.order.getStatusCounts({ page: 1, limit: -1 }),
    retry: false,
    enabled: isActive,
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
