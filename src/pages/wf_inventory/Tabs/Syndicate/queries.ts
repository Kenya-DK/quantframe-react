import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { TauriTypes } from "$types";

interface QueriesHooks {
  queryData: TauriTypes.WFItemControllerGetListParams;
  isActive?: boolean;
}

export const useQueries = ({ queryData, isActive }: QueriesHooks) => {
  const getSyndicatesQuery = useQuery({
    queryKey: ["wf_inventory_get_syndicates", queryData],
    queryFn: () => api.wf_inventory.getSyndicatesPagination(queryData),
    retry: false,
    enabled: isActive,
  });
  const refetchQueries = () => {
    getSyndicatesQuery.refetch();
  };
  // Return the queries
  return {
    syndicatesQuery: getSyndicatesQuery,
    refetchQueries,
  };
};
