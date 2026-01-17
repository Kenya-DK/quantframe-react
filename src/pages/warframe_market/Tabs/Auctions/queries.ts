import { useQuery } from "@tanstack/react-query";
import { TauriTypes } from "$types";
import api from "@api/index";

interface QueriesHooks {
  queryData: TauriTypes.WishListControllerGetListParams;
  isActive?: boolean;
}

export const useStockQueries = ({ queryData, isActive }: QueriesHooks) => {
  const getPaginationQuery = useQuery({
    queryKey: ["get_wfm_auctions_pagination", queryData],
    queryFn: () => api.auction.getPagination(queryData),
    retry: false,
    enabled: isActive,
  });
  const getOverviewQuery = useQuery({
    queryKey: ["get_wfm_auctions_overview", queryData],
    queryFn: () => api.auction.getOverview(queryData),
    retry: false,
    enabled: isActive,
  });

  const refetchQueries = () => {
    getPaginationQuery.refetch();
    getOverviewQuery.refetch();
  };

  // Return the queries
  return {
    paginationQuery: getPaginationQuery,
    overviewQuery: getOverviewQuery,
    refetchQueries,
  };
};
