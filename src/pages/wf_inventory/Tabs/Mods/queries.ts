import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { TauriTypes } from "$types";

interface QueriesHooks {
  queryData: TauriTypes.WFItemControllerGetListParams;
  isActive?: boolean;
}

export const useQueries = ({ queryData, isActive }: QueriesHooks) => {
  const getModsQuery = useQuery({
    queryKey: ["wf_inventory_get_mods", queryData],
    queryFn: () => api.wf_inventory.getModsPagination(queryData),
    retry: false,
    enabled: isActive,
  });
  return {
    modsQuery: getModsQuery,
    refreshMods: getModsQuery.refetch,
  };
};
