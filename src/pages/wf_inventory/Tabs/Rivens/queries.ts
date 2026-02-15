import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { TauriTypes } from "$types";

interface QueriesHooks {
  queryData: TauriTypes.VeiledRivenControllerGetListParams;
  isActive?: boolean;
}

export const useQueries = ({ queryData, isActive }: QueriesHooks) => {
  const getVeiledRivensQuery = useQuery({
    queryKey: ["get_veiled_rivens", queryData],
    queryFn: () => api.wf_inventory.getVeiledRivenPagination(queryData),
    retry: false,
    enabled: isActive,
  });
  const getUnveiledRivensQuery = useQuery({
    queryKey: ["get_unveiled_rivens"],
    queryFn: () => api.wf_inventory.getUnveiledRivens(),
    retry: false,
    enabled: isActive,
  });

  // Return the queries
  return {
    veiledRivensQuery: getVeiledRivensQuery,
    unveiledRivensQuery: getUnveiledRivensQuery,
  };
};
