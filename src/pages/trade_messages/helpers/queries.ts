import { useQuery } from "@tanstack/react-query";
import { TauriTypes } from "$types";
import api from "@api/index";

interface QueriesHooks {
  queryData: TauriTypes.TradeEntryControllerGetListParams;
  isActive?: boolean;
}

export const useQueries = ({ queryData, isActive }: QueriesHooks) => {
  const getPaginationQuery = useQuery({
    queryKey: [
      "get_trade_entry_pagination",
      queryData.group,
      queryData.page,
      queryData.limit,
      queryData.query,
      queryData.tags,
      queryData.sort_by,
      queryData.sort_direction,
    ],
    queryFn: () => api.trade_entry.getPagination(queryData),
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
