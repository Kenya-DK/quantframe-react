import { WFMarketTypes } from "$types";
import api from "@api/index";
import { useQuery } from "@tanstack/react-query";

interface QueriesHooks {
  queryData: WFMarketTypes.WfmOrderControllerGetListParams;
  isActive?: boolean;
}

export const useStockQueries = ({ queryData, isActive }: QueriesHooks) => {
  const getPaginationQuery = useQuery({
    queryKey: ["get_wfm_orders_pagination", queryData],
    queryFn: () => api.order.getPagination<{ standingCost: number; syndicate: string; name: string }>(queryData),
    retry: false,
    enabled: isActive,
  });
  const refetchQueries = () => {
    getPaginationQuery.refetch();
  };

  // Return the queries
  return {
    paginationQuery: getPaginationQuery,
    refetchQueries,
  };
};
