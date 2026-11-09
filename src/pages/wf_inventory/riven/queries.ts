import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { TauriTypes } from "$types";

interface QueriesHooks {
  queryData: TauriTypes.WFItemControllerGetListParams;
  isActive?: boolean;
}

export const useQueries = ({ queryData, isActive }: QueriesHooks) => {
  const getVeiledRivensQuery = useQuery({
    queryKey: ["wf_inventory_get_rivens", queryData],
    queryFn: () => api.wf_inventory.getRivensPagination(queryData),
    retry: false,
    enabled: isActive,
  });
  const refetchQueries = () => {
    getVeiledRivensQuery.refetch();
  };
  // Return the queries
  return {
    veiledRivensQuery: getVeiledRivensQuery,
    refreshVeiledRivens: getVeiledRivensQuery.refetch,
    refetchQueries,
  };
};
